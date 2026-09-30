fn main() {
    let name = "Kene Azuike";
    let uni:&str = "Pan-Atlantic University";
    let addr:&str = "Km 52 Lekki-Epe Expressway, Ibeju-Lekki, Lagos";
    println!("Name: {}", name);
    println!("University: {}, \nSchool: {}", uni, addr);

    let department:&'static str = "Software Engineering";
    let school:&'static str = "School of Science and Technology";
    println!("Department : {}, \nSchool: {}", department, school);
}
