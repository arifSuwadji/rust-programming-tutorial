use std::io;
use std::io::Write;

// harga tiket masuk suatu tempat wisata alam adalah Rp 7 rb per pengunjung
// jumlah pemasukan = jumlah pengunjung dikalikan Rp 7 rb
fn main () {
    print!("masukkan jumlah pengunjung: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");

    //konversi data ke tipe integer
    let pengunjung: i32 = input.trim().parse().unwrap();
    println!("pemasukan: {} ribu rupiah", pengunjung * 7);
}