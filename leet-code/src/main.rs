use leet_code::fibonacci::{self, memo};
use std::io;

fn main() {
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: usize = guess.trim().parse().expect("Please type a number!");
    println!("you give number is {:?}", guess);
    // println!("{}", fibonacci::fib(guess));
    // get_arr(10);
    println!("{}", memo::fib_memo(guess));
    println!("{}", fibonacci::fib_dp(guess));
    println!("{}", fibonacci::fib_small_space_dp(150));
}

// fn get_arr(n: usize) {
//     const N: usize = 10;
//     // pointer sized
//     let arr = [0; N];
//     print1ln!("{:?}", arr);
// }