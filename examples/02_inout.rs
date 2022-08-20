use std::io;

fn main(){
  
  // say hello
  println!("Guess the number!");
  
  // get user input
  println!("Input your guess:");
  let mut guess: String = String::new();
  let nbytes = io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
  
  // print 
  println!("guess raw = \"{guess}\" with nbytes = {nbytes}");
  
  // remove trailing newline/whitespace
  let guess_trimmed = guess.as_mut_str().trim_end();
  println!("guess trimmed = \"{guess_trimmed}\"");
}
