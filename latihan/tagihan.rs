use std::io;
use std::io::Write;

/* Ketentuan PDAM
    "... setiap pelanggan dikenakan pemakaian minimal 10m3 ..."
    "... tarif harga per m3: Rp 5 ribu rupiah ..."

    buatlah program dengan 1 argument : berapa m3 pemakaian air
    output: berapa ribu rupiah tagihannya
    contoh2:
    - pemakaian 4 m3 : tagihan 50 ribu
    - pemakaian 9 m3 : tagihan masih sama 50 rb (pemakaian minimal)
    - pemakaian 17 m3 : 17 * 5 = 85 ribu rupiah
*/

fn main () {
    print!("masukkan jumlah pemakaian(m3): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    let pemakaian: i32 = input.trim().parse().unwrap(); //konversi ke tipe integer

    let tagihan = pemakaian * 5;
    if pemakaian < 10 {
        println!("Tagihan anda: {} ribu rupiah (pemakaian minimal)", 10 * 5);
    }else{
        println!("Tagihan anda: {} * 5 = {} ribu rupiah", pemakaian, tagihan);
    }
}