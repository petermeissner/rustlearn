// source: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("\n\n**** Guess the number! **** \n");

    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);

    let mut i = 0;
    loop {
      // increment counter
      i = i + 1;
      
      // get input
      let mut guess: String = String::new();
      println!("\nPlease input your guess.");
      io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");
      
      // parse and compare
      let guess: u8 = 
        match guess.trim().parse() 
        {
          Ok(num) => num,
          Err(_) => {
            eprintln!("Error: Please input number!");
            continue
          },
        };
      
        // check input
      match guess.cmp(&secret_number) 
      {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
          println!("You win after {i} tries.");
          break;
        },
      }
  }
}
