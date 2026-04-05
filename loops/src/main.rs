fn main() {
    // infinite loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2
        }
    };
    println!("result is {result}");

    // nested loops can be broken out of using a loop label
    let mut count = 0;
    'counting_up: loop { // loop labels must begin with single quote
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
    };
    println!("end count = {count}");

    // conditional while loop
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("liftoff!");

    let a = [19, 29, 39, 4];
    let mut index = 0;
    while index < 4 {
        println!("val is: {}", a[index]);
        index += 1;
    }
    for element in a {
        println!("val is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("liftoff!");
    
}
