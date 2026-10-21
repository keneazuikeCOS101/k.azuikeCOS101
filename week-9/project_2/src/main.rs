use std::io::Write;
use std::fs::File;


fn main() {
    //entering data into an array 
    let student_name :[&str; 5] = ["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Bianca Edemoh"];
    let matric_number :[&str; 5] = ["ACC10211111", "EC010110101", "CSC10328828","EEE11020202", "MEE10202001"];
    let department :[&str; 5] = ["Accounting", "Economics", "Computer", "Electrical", "Mechanical"];
    let level:[&str; 5] = ["300", "100", "200", "200", "100"];

    let mut pau_smis = File::create("PAU-SMIS.csv").expect("failed to create");
    println!("Welcome to the PAU-SMIS file");
    
    //writing data into file using loop
    pau_smis.write_all("Stdent Name, Matric. Number, Department, Level\n".as_bytes()).expect("failed to write");
    for i in 0..level.len(){
        pau_smis.write_all(student_name[i].as_bytes()).expect("failed to write");
        pau_smis.write_all(",".as_bytes()).expect("failed to write");
        pau_smis.write_all(matric_number[i].as_bytes()).expect("failed to write");
        pau_smis.write_all(",".as_bytes()).expect("failed to write");
        pau_smis.write_all(department[i].as_bytes()).expect("failed to write");
        pau_smis.write_all(",".as_bytes()).expect("failed to write");
        pau_smis.write_all(level[i].as_bytes()).expect("failed to write");
        pau_smis.write_all("\n".as_bytes()).expect("failed to write");
    }

    //complementary message at the end
    println!("\nThe PAU Student Management Information System has been created");

}