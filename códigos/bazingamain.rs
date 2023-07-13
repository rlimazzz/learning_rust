use std::cmp::Ordering;
use std::io;

fn main() {
    let mut entrada = String::new();
    let bazinga = 444;

    io::stdin().read_line(&mut entrada)
        .expect("NAO FOI INSERIDO");

    let entrada : u32 = entrada.trim().parse()
        .expect("INCORRETO");
    
    match entrada.cmp(&bazinga) {
        Ordering::Greater =>{
            println!("NAO SERA BAZINGA");
        }
        Ordering::Equal =>{
            println!("BAZINGA");
        }
        Ordering::Less =>{
            println!("BOOM");
        }
    }
}
