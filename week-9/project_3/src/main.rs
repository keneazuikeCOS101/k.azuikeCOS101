use std::io::Write;
use std::fs::File;


fn main() {
    //entering data into an array 
    let commissioner :[&str; 5] = ["Aigbogun Alamba Daudu", "Murtala Afeez Bendu",
         "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
    let ministry:[&str; 5] = ["Internal Affairs", "Justice", "Defense","Power & Steel", "Petroleum"];
    let zone :[&str; 5] = ["South West", "North East", "South South", "South West", "South East"];

    let mut efcc_file = File::create("The EFCC Merge.csv").expect("failed to create");
    println!("The EFCC Merge");
    
    //writing data into file using loop
    efcc_file.write_all("Commissioner, Ministry, Zone\n".as_bytes()).expect("failed to write");
    for i in 0..zone.len(){
        efcc_file.write_all(commissioner[i].as_bytes()).expect("failed to write");
        efcc_file.write_all(",".as_bytes()).expect("failed to write");
        efcc_file.write_all(ministry[i].as_bytes()).expect("failed to write");
        efcc_file.write_all(",".as_bytes()).expect("failed to write");
        efcc_file.write_all(zone[i].as_bytes()).expect("failed to write");
        efcc_file.write_all("\n".as_bytes()).expect("failed to write");
    }

    //complementary message at the end
    println!("\nThe datasets have been marged into 1 file.")
}