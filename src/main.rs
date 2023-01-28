// a rust game
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("welcome to the guess game");
    println!("please enter a first guess");
    let mut guess = String::new();
    let secret_num = rand::thread_rng().gen_range(1..=100);
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read the string");
    println!("you guess the number is {:?}", guess);
    println!("the secret number is {:?}", secret_num);

    let guess: u32 = guess.trim().parse().expect("please type a number");
    match guess.cmp(&secret_num) {
        Ordering::Less => println!("your guess is too low"),
        Ordering::Greater => println!("your guess is too big"),
        Ordering::Equal => {
            println!("you win")
        }
    }
}
