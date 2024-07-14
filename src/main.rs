use std::io ; 

fn main() {
    println!("Hello, world!");
    println!("guess the number!");
    println!("pleas input your guess.");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("failed to read line ");
    println!("You guessed : {guess}");
}
