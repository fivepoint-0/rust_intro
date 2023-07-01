// Import a local module namespace
mod guessing_game;
mod mutable_vars;
mod split_section;
mod loops;
mod tuples;
mod function_references;
mod structs;
mod slice;

// Import specific functoin from "mod split_section"
use split_section::split_section;

fn main() {
    // Hello World
    println!("Hello, world!"); split_section();

    // Guessing Game
    // module::public_function() is how you call an imported function without specific importing
    // guessing_game::guessing_game_01(); split_section();

    mutable_vars::mutable_vars_example(); split_section();

    loops::named_loop_example(); split_section();

    loops::while_loop_example(); split_section();

    loops::access_array_in_loop_example(); split_section();

    loops::foreach_loop_example(); split_section();

    loops::fib_example(); split_section();

    tuples::destructured_tuple_example(); split_section();

    tuples::access_tuple_element_by_index_example(); split_section();

    function_references::add_to_string_example(); split_section();

    structs::rectangle_area_example(); split_section();

    slice::first_word_example(&String::from("Hello world!")); split_section();

    slice::second_word_example(&String::from("Hello world!")); split_section();
}
