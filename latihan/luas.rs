use std::io;
use std::io::Write;

//Luas tandah dari 2 argument: panjang (meter), lebar (meter).
//tampilkan info jika luas lebih besar dari 100 meter persegi

fn main() {
    //input panjang
    print!("masukkan panjang(m): ");
    io::stdout().flush().unwrap();

    let mut inputpanjang = String::new();
    io::stdin().read_line(&mut inputpanjang).expect("failed to read panjang");
    let panjang: i32 = inputpanjang.trim().parse().unwrap();//konversi ke tipe integer

    //input lebar
    print!("masukkan lebar(m): ");
    io::stdout().flush().unwrap();

    let mut inputlebar = String::new();
    io::stdin().read_line(&mut inputlebar).expect("failed to read lebar");
    let lebar: i32 = inputlebar.trim().parse().unwrap();//konversi ke tipe integer

    //menampilkan info
    let luas = panjang * lebar;
    if luas > 100 {
        println!("Luas {} meter persegi Lebih besar dari 100 meter persegi", luas);
    }else{
        println!("Luas {}", luas);
    }
}