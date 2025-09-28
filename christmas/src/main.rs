use std::io;

fn main() {
    let day = [
    "first", "second", "third", "fourth",
    "fifth", "sixth", "seventh", "eighth",
    "ninth", "tenth", "eleventh", "twelfth"
    ]

    let gift = [
    "And a patridge in a pear tree", 
    "Two turtle doves", 
    "Three French hens",
    "Four calling birds",
    "Five g-o-l-d-e-n rings",
    "Six gesse a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven Pipers Piping",
    "Twelve Drummers drumming"
    ]

    loop {
        println!("What day of Christmas is it??");

        let mut day_of = String;;new();

        io::stdin()
        .read_line(&mut day_of)
        .expect("Failed to read line");

        let day_of: u32 = match day_of.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let index: u32 = day_of - 1;

        let clamped_index: u32 = index.clamp(0, 11) {
            Ok(num) => num,
            Err(_) => continue,
        };


    }
}
