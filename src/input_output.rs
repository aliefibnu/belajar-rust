use std::io;
use rand::prelude::*;
use std::cmp::Ordering;

pub fn start(){
  println!("\nInput Output OK");
  let secret = rand::rng().random_range(1..=10);
  println!("Angka rahasia : {secret}");

  loop {
    println!("Masukkan Angka : ");

    let mut tebakan = String::new();

    io::stdin()
      .read_line(&mut tebakan)
      .expect("Gagal baca");

    let tebakan:u32 = match tebakan.trim().parse() {
      Ok(num)=>num,
      Err(_)=>{println!("Input number kocak"); continue},
    }; 

    println!("Tebakan kamu : {tebakan}");

    match tebakan.cmp(&secret){
      Ordering::Less=>println!("Terlalu kecil"),
      Ordering::Greater=>println!("Terlalu Besar"),
      Ordering::Equal=>{println!("Kamu Menang"); break;},
    }
  }
  
}
