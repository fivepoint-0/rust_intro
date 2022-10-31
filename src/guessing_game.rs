use std::io;

pub fn guessing_game_01() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .unwrap();

    println!("You guessed: {guess}");
}