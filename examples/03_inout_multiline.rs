/*
This demonstrates to read multiple lines into 'guess' variable and 
that `io::stdin().read_line() will append to the variable referenced as parameter`
*/

use std::io;

fn main(){
  
  // say hello
  println!("Guess the number!");
  
  // get user input - 3 times one line of input
  println!("Input your guess:");
  let mut guess: String = String::new();
  
  let nbytes = io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
  let nbytes = io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line") + nbytes;
  let nbytes = io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line") + nbytes;
  
    
  // print 
  println!("guess raw = \n\"{guess}\" \n... with nbytes = {nbytes}");
  
  // remove trailing newline/whitespace
  let guess_trimmed = guess.as_mut_str().trim_end();
  println!("guess trimmed = \"{guess_trimmed}\"");
}
