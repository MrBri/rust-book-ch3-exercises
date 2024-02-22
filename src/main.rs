const GIFTS: [&str; 11] = [
    "Two turtle doves",
    "Three French hens",
    "Four calling birds",
    "Five gold rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
];

const DAYS: [&str; 11] = [
    "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelveth",
];

fn main() {
    println!("On the first day of Christmas my true love sent to me");
    println!("A partridge in a pear tree.\n");

    for (i, day) in DAYS.iter().enumerate() {
        println!("On the {day} day of Christmas my true love sent to me");
        let upto = &GIFTS[0..=i];
        for j in upto.iter().rev() {
            println!("{j},");
        }
        println!("And a partridge in a pear tree.\n");
    }
}
