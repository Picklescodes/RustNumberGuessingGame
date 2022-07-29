mod mainplay;

use rand::Rng;

fn main()  {
    println!("Guess the number!");
    let mut guess_count: i32 = 5;
    println!("You have {0} guesses", guess_count);
    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);

    loop  { 
    if guess_count == 0 {
        println!("Oh no, you lost! The secret number was {0}", secret_number);
        break;
        }  else  {
            guess_count -= 1;
            mainplay::play(secret_number);
            println!("You have {0} guesses", guess_count);
        }
    }
}