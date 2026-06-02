
use std::fs;
fn main() {
    let numbers = vec![10, 20, 30];

    let result = numbers.get(1);
            
    // Folosim un match pentru a verifica dacă am găsit elementul
    // .get nu ia ownership, ci intoarce o referinta &i32 Option<&i32>
    // merge sa pun let result = numbers.get(1); deoarece Rust il deduce automat ca fiind Option<&i32>
    match result {
        Some(value) => println!("Am găsit: {}", value),
        None => println!("Nu există acel element"),
    }

    let content = fs::read_to_string("file.txt");

    match content {
        Ok(text) => println!("Fișierul conține: {}", text),
        Err(error) => println!("Eroare la citire: {}", error),
    }
}