type Age = u32;

fn print_age(age: Age) {
    println!("Age is: {}", age);
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    match &numbers[..] {
        [first, rest @ ..] => {
            println!("First number: {}", first);
            println!("Rest of the numbers: {:?}", rest);
        },
        _ => println!("The vector is empty."),
    }

    let person_age: Age = 25;
    print_age(person_age);    
}
