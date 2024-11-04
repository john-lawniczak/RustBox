fn main() {
    let x: u32 = 5;
    println!("Outer x: {}", x); // Print the outer x

    {
        let x = 1; // This `x` shadows the outer `x` in this block
        println!("Inner x (shadowed): {}", x + x); // Print the inner shadowed x
    }

    println!("Outer x again: {}", x); // Print the outer x again
}
