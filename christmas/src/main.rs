use std::io;

fn main() {
    let day = [
    "first", "second", "third", "fourth",
    "fifth", "sixth", "seventh", "eighth",
    "ninth", "tenth", "eleventh", "twelfth"
    ];

    let gift = [
    "And a patridge in a pear tree", 
    "Two turtle doves", 
    "Three French hens",
    "Four calling birds",
    "Five g-o-l-d-e-n rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven Pipers Piping",
    "Twelve Drummers drumming"
    ];

    loop {
        println!("What day of Christmas is it??");

        let mut day_of = String::new();

        io::stdin()
        .read_line(&mut day_of)
        .expect("Failed to read line");

        let day_of: u32 = match day_of.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if day_of == 0 {
            println!("Zero days of Christmas?!?!");
            continue;
        }
        else if day_of > 12 {
            println!("There's only twelve days of Christmas!");
            continue;
        }
        else {
            let index: u32 = day_of - 1;
            let clamped_index: usize = index.clamp(0, 11).try_into().unwrap();
            song(clamped_index, day, gift);
            break;
        }
    }
}

fn song (n: usize, day: [&str; 12], gift: [&str; 12]) {
    if n == 0 {
        println!("On the first day of Christmas my true love sent to me");
        println!("A Partridge in a pear tree");
    }
    else {
        println!("On the {} day of Christmas my true love sent to me", day[n]);
        for i in (0..=n).rev() {
            println!("{}", gift[i]);
        }
    }
}


