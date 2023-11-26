fn main() {
    let x: usize = 5;
    let y: f64 = 1.5;
    let z = (x as f64) / y;
    println!("{} / {} = {}", x, y, z);

    let c = 'z';
    let heart_eyed_cat = '😻';
    println!("{} {}", c, heart_eyed_cat);


    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("{} {} {}", x, y, z);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    println!("{} {}", five_hundred, six_point_four);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    let a = [3; 5];
    let first = a[0];
    let second = a[1];
    println!("{} {}", first, second);
}
