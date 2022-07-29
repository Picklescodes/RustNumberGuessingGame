use std::io;

pub fn play(secret_number: i32)  {
    println!("Input your guess between 0 - 101: ");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to Read");
    
    let guess_int: i32 = guess.trim().parse().expect("Input not an integer");

    let correct = if guess_int == secret_number{true} else {false};
    if correct == true {
        panic!("You Won!");
    } else {
        let high = if guess_int > secret_number{true} else {false};
        if high == true  {
            println!("You were to high"); 
        }  else  {
            println!("You were to low"); 
        }
    }
}