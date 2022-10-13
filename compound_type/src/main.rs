fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    // can access tuple element by using a period(.) followed by index
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("five_hundred: {five_hundred}");
    println!("six_point_four: {six_point_four}");
    println!("one: {one}");

    // arrays
    let a = [1, 2, 3, 4, 5];
    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let same_val = [3; 5]; // 5 3's
    println!("array a: {:?}", a);
    println!("array months: {:?}", months);
    println!("array same_val: {:?}", same_val);
}
