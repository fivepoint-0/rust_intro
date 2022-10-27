pub fn mutable_vars_example() {
    // Mutable and immutable variables
    let apples = 5; // immutable
    let mut bananas = 5; // mutable 
    println!("Immutable apples before: {}", apples);
    println!("Mutable bananas before: {}", bananas);
    bananas = 6;
    println!("Immutable apples after: {}", apples);
    println!("Mutable bananas after: {}", bananas);
}