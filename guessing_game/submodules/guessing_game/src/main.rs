use std::io;

fn main() {
    println!("Convert Fahrenheit to Celsius!");

    loop {
        println!("Enter a temperature, as only a number, in Fahrenheit:");
        
        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let celsius = f2c(temp);
        println!("The temperature {temp} degrees F is {:?} degrees C", celsius);
        break;
    }

}

fn f2c(f: f32) -> f32 {
    (f - 32.0) * (5.0 / 9.0)
}

