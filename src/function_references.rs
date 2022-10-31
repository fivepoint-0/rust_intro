pub fn add_to_string_example() {
    let mut original_string: String = String::from("Hello");

    let adding_string: String = String::from(", world!");

    println!("The value of original_string BEFORE add_to_string() call is {original_string}");

    add_to_string(&mut original_string, adding_string);

    println!("The value of original_string AFTER add_to_string() call is {original_string}");
}

fn add_to_string(string: &mut String, string_to_add: String) {
    string.push_str(&string_to_add);
}