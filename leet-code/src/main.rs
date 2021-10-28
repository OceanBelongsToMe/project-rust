use leet_code::fibonacci;
// use std::io;

fn main() {
    println!("Please input your guess.");
    // let mut guess = String::new();
    // io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Failed to read line");
    // guess.push_str("184");
    // let guess: usize = guess.trim().parse().expect("Please type a number!");
    // println!("you give number is {:?}", guess);
    println!("{}", fibonacci::fib(20));
    // get_arr(10);
    // println!("{}", memo::fib_memo(guess));
    // println!("{}", fibonacci::fib_dp(guess));
    // println!("{}", fibonacci::fib_small_space_dp(guess));
}

// fn get_arr(n: usize) {
//     const N: usize = 10;
//     // pointer sized
//     let arr = [0; N];
//     print1ln!("{:?}", arr);
// }
