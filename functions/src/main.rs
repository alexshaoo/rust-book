fn main() {
    println!("Hello, world!");
    another_function(5);
    let y = {
        let x = 3;
        x+1
    };

    println!("y value is: {y}");

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number: {number}");
}

fn another_function(x: i32) {
    println!("val of x: {x}");
}
