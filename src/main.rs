use std::{io, u32};
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Adivinheo o número");

    let secret_number:u32 = rand::thread_rng().gen_range(1..=100);
    let mut guesses: u32 = 0;
    loop{
        println!("Digite um número: ");

        let mut guess:String = String::new();
        io::stdin().read_line(&mut guess).expect("Falha ao ler o número");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        guesses += 1;
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("Voce acertou!");
                break;
            },
            Ordering::Less => println!("Número digitado é Menor"),
            Ordering::Greater => println!("Número digitado é Maior")
        }
    }
    println!("Voce tentou {guesses} vezes.");

}
