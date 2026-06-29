use std::fs::File;
#[allow(unused_variables)]
fn main() {
    // Varianta 1: folosind expect
    //let file1 = File::open("file.txt").expect("Cant find the file");
    //println!("File opened successfully: {:?}", file1);

    //Varianta 2: folosind match
    let file1 = match File::open("file.txt") {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error);
        }
    };

    // Varianta 3, when to use ?
    

}