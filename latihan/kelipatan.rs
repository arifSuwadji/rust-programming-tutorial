use std::io;
use std::io::Write;

// tampilkan keterangan jika suatu angka adalah kelipatan 7

fn main() {
    print!("masukkan angka: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let angka: i32 = input.trim().parse().unwrap();

    let kelipatan = angka % 7;
    if kelipatan == 0 {
        println!("{} adalah kelipatan 7", angka);
    }else{
        println!("{} bukan kelipatan 7", angka);
    }
}