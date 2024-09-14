use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let guessing_number = 50;

    'guessing_game: loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 입력된 문자열을 숫자로 변환
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a valid number.");
                return;
            }
        };

        println!("You guessed: {}", guess);

        if guess < guessing_number {
            println!("Add more!");
        } else if guess > guessing_number {
            println!("try Minus");
        } else {
            println!("You did it!");
            break 'guessing_game;
        }
    }
}
