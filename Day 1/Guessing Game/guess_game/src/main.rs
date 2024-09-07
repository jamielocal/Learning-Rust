use std::io;
use rand::Rng;
use std::cmp::Ordering;




fn main() {
    println!("Guess A Number!");
    let numb = rand::thread_rng().gen_range(1..=100);
    println!("Number is {numb}");
    loop {
        println!("Input Number Below");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed To Read Line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        println!("Guessed {}", guess);
        match guess.cmp(&numb) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
