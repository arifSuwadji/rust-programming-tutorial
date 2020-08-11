use std::io;
use std::io::Write;

fn main(){
    print!("nama buah ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    if input.trim() == "apel" {
        println!("betul");
    }else{
        println!("bukan");
    }
}