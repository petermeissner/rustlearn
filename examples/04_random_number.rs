use std::io;
use rand::Rng;

fn main() {
    println!("Random numbers ...\n");

    // get parameters
    println!("N?:");
    let mut n = String::new();
    io::stdin()
    .read_line(&mut n)
    .expect("Failed to read line");
    let n_u32: u32 = "7".parse().expect("Parsing Failed");
    // TODO!!! : parsing does not work, yet ???!!!
    println!("N: {n_u32}");
    
    
    println!("Start?:");
    let mut start = String::new();
    io::stdin()
    .read_line(&mut start)
    .expect("Failed to read line");
    // TODO!!! : parsing does not work, yet ???!!!
    let start_u32: u32 = "1".parse().unwrap();
    
    println!("End?:");
    let mut end = String::new();
    io::stdin()
    .read_line(&mut end)
    .expect("Failed to read line");
    // TODO!!! : parsing does not work, yet ???!!!
    let end_u32: u32 = "6".parse().unwrap();
    
    for i in 0..n_u32 {
      let secret_number: u32 = rand::thread_rng().gen_range(start_u32..end_u32);
      let ii = i+1;
      println!("{ii}: {secret_number}");
    }


    
}
