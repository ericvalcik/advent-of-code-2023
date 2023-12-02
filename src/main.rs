mod day1;
mod day2;

fn main() {
    match day2::calc_number() {
        Ok(number) => println!("{}", number),
        Err(_) => println!("Error"),
    }
}
