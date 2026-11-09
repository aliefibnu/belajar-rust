mod input_output;

fn main() {
    println!("\n");
    let nama = "Alief Ibnu Hamdani";
    println!("Hello, world!\n");
    println!("Selamat Datang : {}", nama);

    let umur_awal = 18;
    let mut umur = umur_awal;
    println!("Umur kamu sekarang : {}", umur);
    
    umur += 1;
    println!("Umur kamu {} tahun lagi : {}", umur-umur_awal, umur);
    
    // input_output.rs
    input_output::start();
}
