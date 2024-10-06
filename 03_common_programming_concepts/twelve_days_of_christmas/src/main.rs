fn main() {
    /* This program prints "The Twelve Days of Christmas"
       as an exercise in loops and conditionals */

    // Counter to incremenet the loop and index to print the number for each day
    let mut count: usize = 0;

    // Main loop
    while count < 12 {
        // Print the "On the x day..."
        print_prelude(count);

        // Twelfth day
        if count >= 11 {
            println!("Twelve drummers drumming,");
        }
        // Eleventh day
        if count >= 10 {
            println!("Eleven pipers piping,");
        }
        // Tenth day
        if count >= 9 {
            println!("Ten lords a-leaping,");
        }
        // Ninth day
        if count >= 8 {
            println!("Nine ladies dancing,");
        }
        // Eighth day
        if count >= 7 {
            println!("Eight maids a-milking,");
        }
        // Seventh day
        if count >= 6 {
            println!("Seven swans-a-swimming,");
        }
        // Sixth day
        if count >= 5 {
            println!("Six geese a-laying,");
        }
        // Fifth day
        if count >= 4 {
            println!("Five golden rings,");
        }
        // Fourth day
        if count >= 3 {
            println!("Four calling birds,");
        }
        // Third day
        if count >= 2 {
            println!("Three french hens,");
        }
        // Second day
        if count >= 1 {
            println!("Two turtle doves,");
        }

        // First day - check if it's the first day to include 'And'
        if count >= 1{
            println!("And a partridge in a pear tree.\n");
        }
        else {
            println!("A partridge in a pear tree.\n");
        }
        
        // Increment and loop again
        count += 1;
    }

    println!("彡(._.)ミ");
    println!("  ^   ^");
}

fn print_prelude(index: usize) {
    let beginning = "On the";
    let end = "day of christmas,\nmy true love gave to me";

    let middle_list = ["first", "second", "third", "fourth",
        "fifth", "sixth", "seventh", "eighth",
        "ninth", "tenth", "eleventh", "twelfth"];
    
    println!("{} {} {}", beginning, middle_list[index], end);
}