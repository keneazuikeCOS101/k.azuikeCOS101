use std::fs;

fn main() {
    fs::remove_file("data.txt").expect("couldn't remove file");
    println!("File is removed");
}
