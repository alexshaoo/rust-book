fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{0}, {1}", tup.0, tup.2);
    let (x, y, _z) = tup;
    println!("{x}, {y}");
    let mut mutable: (i32, i32) = (1, 2);
    mutable.0 += 1;
    mutable.1 += 100;
    println!("{0}, {1}", mutable.0, mutable.1);
    let _months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ]; // don't need this type, rust can figure this out, just did it for transparency
    let _a = [3; 5]; // [3, 3, 3, 3, 3];
    let _message = "The temperature today is:";
    //    let xx = [message, 100];
    //    println!("{} {}", x[0], x[1]);
    let t = ([1; 2], [3; 4]);
    let (a, _b) = t;
    println!("{}", a[0] + t.1[0]);

    // wrapping with wrapping_*
    let value: u8 = 255;
    let result = value.wrapping_add(2); // if just 255 + 2, panic
    println!("wrapping {} + 2 = {}", value, result);

    // checked_*
    let value: u8 = 255;
    let result = value.checked_add(2);
    match result {
        Some(v) => println!("result is {}", v),
        None => println!("Error: math overflow"),
    }

    // overflowing_*
    let value: u8 = 255;
    let (result, overflowed) = value.overflowing_add(1);
    println!("value: {}, did it overflow? {}", result, overflowed);

    // saturating_*
    // sticks at edges, if you try to go above maximum it'll stay at minimum, same with min, etc
    let value: u8 = 255;
    let result = value.saturating_add(10);
    println!("saturating: {} + 10 = {}", value, result);
    let small_val: u8 = 0;
    println!(
        "saturating sub: {} - 1 = {}",
        small_val,
        small_val.saturating_sub(1)
    );

    // floating point types are all signed
    let x = 2.0;
    let y: f32 = 3.0;
    // integer division truncates TOWARDS 0
    let quotient = 56.7 / 32.2; // 1.7608695...
    let truncated = -5 / 3; // -1
    println!("{} {}", quotient, truncated);
    // see https://rust-book.cs.brown.edu/appendix-02-operators.html for more operators
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
