use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Bienvenido a mi juego de adivinar el numero");

    let numero_adivinado = rand::thread_rng().gen_range(0..=100);

    loop {
        println!("Dime un numero entre 0 y 100");
        
        let mut numero = String::new();
        io::stdin()
            .read_line(&mut numero)
            .expect("Failed to read line");

        let numero: u8 = match numero.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match numero.cmp(&numero_adivinado) {
            Ordering::Less => println!("Muy peque"),
            Ordering::Greater => println!("Muy grande"),
            Ordering::Equal => {
                println!("correcto, Has Ganado!");
                break;
            },
            
        }

    }
}