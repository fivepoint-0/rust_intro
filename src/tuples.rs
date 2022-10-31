pub fn tuples_example() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let tup2 = (500, 6.4, 1);
    let five_hundred = tup2.0;
    let six_point_four = tup2.1;
    let one = tup2.2;
}