use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::{thread, time};

fn main() {
    println!("Guess the number 1 to 100!");

    thread::sleep(time::Duration::from_secs(2));

    let secret_num = rand::thread_rng().gen_range(1..=110);


    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! The number was {secret_num}");
                thread::sleep(time::Duration::from_secs(5));
                break;
            }
        }
   }
}