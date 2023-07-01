pub fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..]
}

pub fn second_word(s: &String) -> &str {
    let v: Vec<&str> = s.split(' ').collect();

    if v.len() > 1 {
        return v[1];
    }

    return &s[..];
}

pub fn first_word_example(s: &String) {
    println!("First word example: Input = '{}', Output = '{}'", s, first_word(s))
}

pub fn second_word_example(s: &String) {
    println!("Second word example: Input = '{}', Output = '{}'", s, second_word(s))
}