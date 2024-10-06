fn main() {
    // Number of sequences
    const N: u32 = 20;
    
    let mut x = 0;
    let mut y = 1;
    let mut sum;

    // Counter for the loop, starts at 2 as x and y are already given
    let mut count = 2;

    // Initialise
    println!("Fibonacci sequence to {N}");
    println!("{x}\n{y}");

    // Main loop
    while count < N {
        sum = x + y;
        x = y;
        y = sum;
        
        println!("{sum}");
        count += 1;
    }
}
