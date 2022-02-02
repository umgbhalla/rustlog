use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number");
    println!("Enter you guess");
    let mut guess = String::new();
    let secret_num = rand::thread_rng().gen_range(1, 101);
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
    println!(
        "oh my god, wao!!  You guessed: {} , secret: {} lamao",
        guess, secret_num
    );
}
