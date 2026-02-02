fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{0}, {1}", tup.0, tup.2);
    let (x, y, z) = tup;
    println!("{x}, {y}");
    let mut mutable: (i32, i32) = (1, 2);
    mutable.0 += 1;
    mutable.1 += 100;
    println!("{0}, {1}", mutable.0, mutable.1);
    let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"]; // don't need this type, rust can figure this out, just did it for transparency
    let a = [3; 5]; // [3, 3, 3, 3, 3];
    let message = "The temperature today is:";
//    let xx = [message, 100];
//    println!("{} {}", x[0], x[1]);
    let t = ([1; 2], [3; 4]);
    let (a, b) = t;
    println!("{}", a[0] + t.1[0]);
}
