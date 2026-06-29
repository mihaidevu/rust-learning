// Varianta 1, folosind iterators
#[allow(dead_code)]
fn process_items_old(items: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in 0..items.len() {
        if items[i] % 2 == 0 {
            sum += items[i];
        }
    }
    sum
}

// Varianta 2, imbunatatita, folosind iterators
fn process_items(items: &[i32]) -> i32 {
    let mut sum = 0;
    for &item in items {
        if item % 2 == 0 {
            sum += item;
        }
    }
    sum
}

//Varianta 3, fara &item
#[allow(dead_code)]
fn process_items_3(items: &[i32]) -> i32 {
    let mut sum = 0;

    for item in items {
        if item % 2 == 0 {
            sum += item;
        }
    }

    sum
}

// Aceasta este cea mai idiomatică și „Rust-correct” alegere, atât din punct de vedere al ownership-ului, cât și al iterării.
// De ce aceasta e cea mai bună?
// ✔️ items: &[i32]
// este un slice (view peste date)
// nu presupune ownership
// este mai general decât &Vec<i32>
// acceptă:
// Vec<i32>
// array-uri [i32; N]

// 👉 În Rust, preferi mereu „mai general” decât „mai specific” când nu ai nevoie de ownership.
// ✔️ for &item in items.iter()
// .iter() dă referințe &i32
// &item face destructuring → obții valoarea i32
// Deci:
// for &item in items.iter()
// înseamnă:
// „ia fiecare referință și copiază valoarea în item”


fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let result = process_items(&numbers);
    println!("Suma numerelor pare este: {}", result);
}