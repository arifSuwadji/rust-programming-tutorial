fn main (){
    let a = 14;
    let mut b = 10; //bisa dimutasikan nilai / isinya
    let c = a * b - 100;
    println!("a: {}, b: {}, c: {}", a, b, c);

    //mutable variable b (let mut b = 10)
    b = b + 10;
    println!("b menjadi: {}", b);

    //dengan menyebutkan tipe
    let d: i32 = 14;
    println!("d: {}", d);
}