use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number!");
    let secreat_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secreat_number}!");
    loop {
        println!("Please input your gusess!");

        let mut gusess = String::new();

        io::stdin()
            .read_line(&mut gusess)
            .expect("Failed to read Line");

        // 必须将string转化为number才能够比较
        let gusess: u32 = match gusess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed:{gusess}");
        // 比较部分
        match gusess.cmp(&secreat_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
