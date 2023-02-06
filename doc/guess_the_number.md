# 猜字谜游戏

```rust
use std::io;

use rand::Rng;
fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..100);

    loop {
        // 获取用户输入
        println!("Please input your guess:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();

        let guess_number: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("input parse err: {}", err);
                continue;
            }
        };

        if guess_number > secret_number {
            println!("too high");
        } else if guess_number < secret_number {
            println!("too low");
        } else {
            println!(
                "Congratulations, you answered correctly! The correct number is {}",
                guess_number
            );
            break;
        }
    }
}
```