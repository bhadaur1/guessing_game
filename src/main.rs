use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("secret_number: {}", secret_number);
    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
                .expect("Failed to create line");

    println!("You entered {}", guess);

    let guess_int : u32 = guess.trim().parse().unwrap();

    match guess_int.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Equal => println!("Bingo!"),
        Ordering::Greater => println!("Too big"),
    }

}
