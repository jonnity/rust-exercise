use std::cmp::Ordering;
use std::io;

mod guess;
use guess::Guess;
fn main() {
    println!("Guess the number!"); // 数を当ててごらん
    let secret_number = rand::random_range(1..101);
    // println!("The secret number is: {}", secret_number); //秘密の数字は次の通り: {}
    loop {
        println!("Please input your guess."); // ほら、予想を入力してね

        let mut user_guess = String::new();

        io::stdin()
            .read_line(&mut user_guess)
            .expect("Failed to read line");
        let user_guess: u32 = match user_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let user_guess = Guess::new(user_guess).value();

        println!("You guessed: {}", user_guess);
        match user_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),  //小さすぎ！
            Ordering::Greater => println!("Too big!"), //大きすぎ！
            Ordering::Equal => {
                println!("You win!");
                break;
            } //やったね！
        }
    }
}
