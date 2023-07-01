use std::{thread, time};

pub fn split_section() {
    let sleep_time = time::Duration::from_millis(250);
    thread::sleep(sleep_time);

    println!("\n---------------------------------\n");
}