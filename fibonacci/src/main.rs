use std::io;

fn main() {
    println!("Fiboncci Number!");

    loop {
        println!("Enter a number for fiboncci position: ");

        let mut n = String::new();

        io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

        let n: i32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        fib(n);
        break;
    }


}

fn fib(num: i32) {
    let mut fib1: i128 = 1;
    let mut fib2: i128 = 1;
    let mut count = 0;
    for _number in 0..(num + 1) {
        if count == 0 {
            println!("{}", 0);
            count += 1;
        }
        else if count <= 2 {
            println!("{}", fib1);
            count += 1;
        }
        else {
            let fib: i128 = fib1 + fib2;
            fib2 = fib1;
            fib1 = fib;
            println!("{}", fib);
        }
    }
}
    


