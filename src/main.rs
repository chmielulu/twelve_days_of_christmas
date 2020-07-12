fn main() {
    let numbers = [
        "first",
        "second",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];

    for (pos, number) in numbers.iter().enumerate() {
        println!("On the {} day of Christmas", number);
        println!("My true love gave to me");

        if pos >= 11 { println!("12 drummers drumming"); }
        if pos >= 10 { println!("Eleven pipers piping"); }
        if pos >= 9 { println!("Ten lords a leaping"); }
        if pos >= 8 { println!("Nine ladies dancing"); }
        if pos >= 7 { println!("Eight maids a milking"); }
        if pos >= 6 { println!("Seven swans a swimming"); }
        if pos >= 5 { println!("Six geese a laying"); }
        if pos >= 4 { println!("Five gold rings, badam-pam-pam"); }
        if pos >= 3 { println!("Four calling birds"); }
        if pos >= 2 { println!("Three French hens"); }
        if pos >= 1 { println!("Two turtle doves"); }

        if pos == 0 { print!("A"); }
        else { print!("And a"); }
        print!(" partridge in pear tree");

        println!();
        println!();
    }
}
