mod options_and_result;

use options_and_result::{test_error, test_option};

fn main() {
    println!("Test Option");

    let x = test_option(5).expect("less than 10");
    println!("{}", x);

    let y = test_option(15).expect("less than 10");
    println!("{}", y);

    match test_option(5) {
        Some(value) => println!("{}", value),
        None => println!("none"),
    }

    // unwrap will return the value inside the Ok. If the Result is the Err variant, unwrap will call the panic! macro for us.
    let z = test_option(5).unwrap();
    println!("{}", z);

    let a = test_option(15).unwrap();
    println!("{}", a);

    println!("Test Error");

    //let b = test_error(4).expect("bad value");

    //println!("{}", b);

    let mut x = 10;

    loop {
        println!("{x}");
        x+=1;
        if x == 20{
            break;
        }
    }
}
