// This example is to illustrate infinite loops with labels 
// so that you can break a particular loop inside of mutliple
// loops
pub fn named_loop_example() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

pub fn while_loop_example() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

pub fn access_array_in_loop_example() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

pub fn foreach_loop_example() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

pub fn fib(n: u128) -> u128 {
    fn inner(n: u128, penultimate: u128, last: u128) -> u128 {
        match n {
            0 => penultimate,
            1 => last,
            _ => inner(n - 1, last, penultimate + last),
        }
    }
    inner(n, 0, 1)
}

pub fn fib_example() {
    for int in 0..100 {
        println!("Fib({}) is {}", int, fib(int));
    }
}