//File saving program for Nigerian Breweries


use std::io::Write;
use std::fs::File;


fn main() {
    //entering data into an array 
    let lager :[&str; 6] = ["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let stout :[&str; 6] = ["Legend", "Turbo King", "Williams","", "", ""];
    let non_alcoholic :[&str; 6] = ["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz", "", ""];

    let mut brewery_file = File::create("Nigeria Breweries.csv").expect("failed to create");
    println!("Welcome to the Nigeria Brewery file");
    
    //writing data into file using loop
    brewery_file.write_all("Lager, Stout, Non-Alcoholic\n".as_bytes()).expect("failed to write");
    for i in 0..lager.len(){
        brewery_file.write_all(lager[i].as_bytes()).expect("failed to write");
        brewery_file.write_all(",".as_bytes()).expect("failed to write");
        brewery_file.write_all(stout[i].as_bytes()).expect("failed to write");
        brewery_file.write_all(",".as_bytes()).expect("failed to write");
        brewery_file.write_all(non_alcoholic[i].as_bytes()).expect("failed to write");
        brewery_file.write_all("\n".as_bytes()).expect("failed to write");
    }

    //complementary message at the end
    println!("\n Data has been put into the file")

}