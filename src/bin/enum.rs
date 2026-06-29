
use std::collections::btree_map::Values;
use std::fs;
use std::fmt;

#[allow(dead_code)]
enum Directions {
    North,
    South,
    East,
    West,
}

enum Contact {
    Email(String),
    Phone(String),
    Age(u32),
}

// Implement Default pentru Contact
impl Default for Contact {
    fn default() -> Self {
        Contact::Email(String::from("default@example.com"))
    }
}

impl fmt::Display for Contact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Contact::Email(email) => write!(f, "Email: {}", email),
            Contact::Phone(phone) => write!(f, "Phone: {}", phone),
            Contact::Age(age) => write!(f, "Age: {}", age),
        }
    }
}

fn main() {
    let c = Contact::default(); 
    println!("Contactul implicit este: {}", c);

    let numbers = vec![10, 20, 30];

    let result = numbers.get(1);

    let age = Contact::Age(25);
    println!("Contactul este: {}", age);
    match age {
        Contact::Age(age) if age < 18 => println!("Minor"),
        Contact::Age(age) if age > 80 => println!("Senior"),
        Contact::Age(_) => println!("Adult"),   
        _ => println!("Alt tip de contact"),
    }
            
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

    let heading = Directions::North;

    match heading {
        Directions::North => println!("Mergem spre nord"),
        Directions::South => println!("Mergem spre sud"),
        _ => println!("Est sau vest"),
    }


    //let six = five + 1; // Acest lucru nu va funcționa deoarece five este de tip Option<i32>, nu i32
    /*   Compiling learning v0.1.0 (/home/mihai/projects/learning)
error[E0369]: cannot add `{integer}` to `Option<i32>` */

    // *** With If let ***/
    // let five: Option<i32> = Some(5);
    // if let Some(value) = five {
    //     let six = value + 1;
    //     println!("Six is: {}", six);
    // }

    // *** Rusty style with map ***/?
    let five: Option<i32> = Some(5);
    let six = five.map(|v| v + 1);
    println!("Six is: {:?}", six);
}