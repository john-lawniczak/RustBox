use std::time::{SystemTime, UNIX_EPOCH};

type U256 = u128;

#[derive(Debug)]
enum Err {
    InsufficientLiquidity,
    Slippage,
    OracleStale,
    HealthFactorTooLow,
    AmountTooSmall,
    MathOverflow,
    PositionUnderwater,
    NotRepaidInFlashloan,
}

fn now_ts() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

struct OraclePrice {
    price_e6: u64,      // price with 6 decimals
    last_update_ts: u64,
}

impl OraclePrice {
    fn assert_fresh(&self, max_age: u64) -> Result<(), Err> {
        if now_ts().saturating_sub(self.last_update_ts) > max_age { return Err::OracleStale.into() }
        Ok(())
    }
}

// --- 1) Swap exact-in (with slippage + fee) ----------------------------------
fn swap_exact_in(
    amount_in: U256,
    reserve_in: U256,
    reserve_out: U256,
    fee_bps: u32,           // e.g., 30 = 0.30%
    min_out: U256,
) -> Result<(U256 /*amount_out*/, U256 /*new_in*/, U256 /*new_out*/), Err> {
    if amount_in == 0 { return Err::AmountTooSmall.into(); }
    // Constant-product x*y=k, fee on input
    let amount_in_less_fee = amount_in
        .saturating_mul((10_000 - fee_bps) as U256)
        .checked_div(10_000).ok_or(Err::MathOverflow)?;

    let numerator   = amount_in_less_fee.checked_mul(reserve_out).ok_or(Err::MathOverflow)?;
    let denominator = reserve_in.checked_add(amount_in_less_fee).ok_or(Err::MathOverflow)?;
    let amount_out  = numerator.checked_div(denominator).ok_or(Err::MathOverflow)?;

    if amount_out < min_out { return Err::Slippage.into(); }

    let new_in  = reserve_in.checked_add(amount_in_less_fee).ok_or(Err::MathOverflow)?;
    let new_out = reserve_out.checked_sub(amount_out).ok_or(Err::InsufficientLiquidity)?;
    Ok((amount_out, new_in, new_out))
}

// --- 2) Provide liquidity (mint LP shares) -----------------------------------
fn add_liquidity_proportional(
    add_a: U256,
    add_b: U256,
    reserve_a: U256,
    reserve_b: U256,
    total_lp: U256,
) -> Result<(U256 /*lp_minted*/, U256 /*new_res_a*/, U256 /*new_res_b*/, U256 /*new_total_lp*/), Err> {
    if total_lp == 0 {
        // first LP mints sqrt(add_a * add_b)
        let k = add_a.checked_mul(add_b).ok_or(Err::MathOverflow)?;
        let lp_minted = integer_sqrt(k);
        let new_total = lp_minted;
        Ok((lp_minted, add_a, add_b, new_total))
    } else {
        // proportional add: lp = min(add_a/res_a, add_b/res_b) * total_lp
        let lp_a = add_a.checked_mul(total_lp).ok_or(Err::MathOverflow)?
                        .checked_div(reserve_a).ok_or(Err::MathOverflow)?;
        let lp_b = add_b.checked_mul(total_lp).ok_or(Err::MathOverflow)?
                        .checked_div(reserve_b).ok_or(Err::MathOverflow)?;
        let lp_minted = lp_a.min(lp_b);
        let new_res_a = reserve_a.checked_add(add_a).ok_or(Err::MathOverflow)?;
        let new_res_b = reserve_b.checked_add(add_b).ok_or(Err::MathOverflow)?;
        let new_total = total_lp.checked_add(lp_minted).ok_or(Err::MathOverflow)?;
        Ok((lp_minted, new_res_a, new_res_b, new_total))
    }
}

fn integer_sqrt(x: U256) -> U256 {
    // tiny integer sqrt for demo (Newton method)
    if x == 0 { return 0; }
    let mut r = x;
    let mut s = (x >> 1) + 1;
    while s < r { r = s; s = (x / s + s) >> 1; }
    r
}

// --- 3) Remove liquidity (burn LP shares) ------------------------------------
fn remove_liquidity(
    lp_burn: U256,
    reserve_a: U256,
    reserve_b: U256,
    total_lp: U256,
) -> Result<(U256 /*out_a*/, U256 /*out_b*/, U256 /*new_res_a*/, U256 /*new_res_b*/, U256 /*new_total_lp*/), Err> {
    if lp_burn == 0 || total_lp == 0 { return Err::AmountTooSmall.into(); }
    let out_a = reserve_a.checked_mul(lp_burn).ok_or(Err::MathOverflow)?
                         .checked_div(total_lp).ok_or(Err::MathOverflow)?;
    let out_b = reserve_b.checked_mul(lp_burn).ok_or(Err::MathOverflow)?
                         .checked_div(total_lp).ok_or(Err::MathOverflow)?;
    let new_res_a = reserve_a.checked_sub(out_a).ok_or(Err::MathOverflow)?;
    let new_res_b = reserve_b.checked_sub(out_b).ok_or(Err::MathOverflow)?;
    let new_total = total_lp.checked_sub(lp_burn).ok_or(Err::MathOverflow)?;
    Ok((out_a, out_b, new_res_a, new_res_b, new_total))
}

// --- 4) Accrue interest via index (lending market) ---------------------------
fn accrue_interest(
    borrow_index_e18: U256, // scaled index (1e18 = no change)
    total_borrows: U256,
    borrow_rate_per_sec_e18: U256,
    last_accrual_ts: u64,
    now_ts: u64,
) -> Result<(U256 /*new_index*/, U256 /*new_total_borrows*/, u64 /*new_last*/ ), Err> {
    if now_ts <= last_accrual_ts { return Ok((borrow_index_e18, total_borrows, last_accrual_ts)); }
    let dt = (now_ts - last_accrual_ts) as U256;
    // simple interest: factor = 1 + rate * dt
    let delta = borrow_rate_per_sec_e18.checked_mul(dt).ok_or(Err::MathOverflow)?;
    let factor_e18 = 1_000_000_000_000_000_000u128.checked_add(delta).ok_or(Err::MathOverflow)?;

    let new_index = borrow_index_e18.checked_mul(factor_e18).ok_or(Err::MathOverflow)?
                                   .checked_div(1_000_000_000_000_000_000).ok_or(Err::MathOverflow)?;
    let new_total = total_borrows.checked_mul(factor_e18).ok_or(Err::MathOverflow)?
                                 .checked_div(1_000_000_000_000_000_000).ok_or(Err::MathOverflow)?;
    Ok((new_index, new_total, now_ts))
}

// --- 5) Borrow against collateral (health factor gate) -----------------------
fn borrow(
    collateral_value_e6: U256,
    debt_value_e6: U256,
    add_debt_e6: U256,
    liq_threshold_bps: u32,   // e.g., 85_00 = 85.00%
) -> Result<U256 /*new_debt_e6*/, Err> {
    // After borrowing, ensure collateral * LT >= debt
    let new_debt = debt_value_e6.saturating_add(add_debt_e6);
    let max_debt_e6 = collateral_value_e6
        .saturating_mul(liq_threshold_bps as U256)
        .checked_div(10_000).ok_or(Err::MathOverflow)?;
    if new_debt > max_debt_e6 { return Err::HealthFactorTooLow.into(); }
    Ok(new_debt)
}

// --- 6) Liquidate (repay bad debt & seize collateral with bonus) -------------
fn liquidate(
    price_collat_e6: U256,
    price_debt_e6: U256,
    collateral_amount: U256,
    debt_outstanding: U256,
    repay_amount: U256,
    liq_bonus_bps: u32,       // e.g., 10500 = 5% bonus
    oracle: &OraclePrice,
) -> Result<(U256 /*collateral_seized*/, U256 /*new_collat*/, U256 /*new_debt*/), Err> {
    oracle.assert_fresh(60)?; // must be fresh within 60s for demo

    if repay_amount == 0 || repay_amount > debt_outstanding {
        return Err::AmountTooSmall.into();
    }
    // value of repay in USD-e6
    let repay_value = repay_amount.checked_mul(price_debt_e6).ok_or(Err::MathOverflow)?;
    // collateral seized with bonus
    let seize_value = repay_value.saturating_mul(liq_bonus_bps as U256) / 10_000;
    let collateral_seized = seize_value.checked_div(price_collat_e6).ok_or(Err::MathOverflow)?;

    if collateral_seized > collateral_amount { return Err::PositionUnderwater.into(); }

    let new_collateral = collateral_amount - collateral_seized;
    let new_debt = debt_outstanding - repay_amount;
    Ok((collateral_seized, new_collateral, new_debt))
}

// --- 7) Flash loan (with callback-style accounting) --------------------------
fn flash_loan<F>(
    pool_liquidity: U256,
    amount: U256,
    fee_bps: u32,
    mut callback: F,
) -> Result<U256 /*new_liquidity*/, Err>
where
    F: FnMut(U256 /*amount_borrowed*/) -> Result<U256 /*amount_repaid*/, Err>,
{
    if amount == 0 || amount > pool_liquidity { return Err::InsufficientLiquidity.into(); }
    let fee = amount.saturating_mul(fee_bps as U256) / 10_000;
    let expected = amount.saturating_add(fee);

    // send out amount (conceptually), run borrower logic:
    let repaid = callback(amount)?;

    if repaid < expected { return Err::NotRepaidInFlashloan.into(); }

    let new_liq = pool_liquidity.saturating_sub(amount).saturating_add(repaid);
    Ok(new_liq)
}

// --- 8) Rebalance a vault (target weights with oracle & bands) ---------------
fn rebalance_to_target(
    assets_value_e6: [U256; 2],    // current USD values [A, B]
    target_bps: [u32; 2],          // target weights in bps summing to 10_000
    band_bps: u32,                  // tolerance band, e.g., 100 = 1%
) -> Result<Option<&'static str>, Err> {
    let total = assets_value_e6[0].saturating_add(assets_value_e6[1]);
    if total == 0 { return Ok(None); }

    let current_a_bps = assets_value_e6[0].saturating_mul(10_000).checked_div(total).ok_or(Err::MathOverflow)? as i64;
    let target_a_bps  = target_bps[0] as i64;
    let diff = (current_a_bps - target_a_bps).abs() as u32;

    if diff <= band_bps { return Ok(None); } // within band, no-op

    // In real code: compute trade size; here, just return intent:
    if current_a_bps > target_a_bps {
        Ok(Some("sell A for B"))
    } else {
        Ok(Some("buy A with B"))
    }
}


