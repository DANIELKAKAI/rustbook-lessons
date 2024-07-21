mod options_and_result;

use options_and_result::{test_error, test_option};

fn main() {
    let b = test_error(4).expect("bad value");

    println!("{}", b);
}
