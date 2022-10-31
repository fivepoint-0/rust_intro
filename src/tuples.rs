pub fn destructured_tuple_example() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}

pub fn access_tuple_element_by_index_example() {
    let tup2 = (500, 6.4, 1);
    let five_hundred = tup2.0;
    let six_point_four = tup2.1;
    let one = tup2.2;
    println!("The value of five_hundred os {five_hundred}");
    println!("The value of six_point_four os {six_point_four}");
    println!("The value of one os {one}");
}