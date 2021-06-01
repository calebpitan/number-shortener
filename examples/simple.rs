extern crate number_shortener;
use number_shortener::{shorten_number};

fn main() {
    let num = 350000_f32;

    println!("Number to shorten: {}", num);

    let short_number = shorten_number(num);

    println!("Shortened number is: {:?}", short_number);
}
