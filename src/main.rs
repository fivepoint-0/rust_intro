// Import a local module namespace
mod guessing_game;
mod mutable_vars;
mod split_section;

// Import specific functoin from "mod split_section"
use split_section::split_section;

fn main() {
    // Hello World
    println!("Hello, world!");

    split_section();

    // Guessing Game
    // module::public_function() is how you call an imported function without specific importing
    guessing_game::guessing_game();

    split_section();

    // Mutable Vars example
    mutable_vars::mutable_vars_example();

    split_section();

    
}
