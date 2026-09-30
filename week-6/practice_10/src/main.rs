fn main() {
    let a = 28;
    let b = 42;

    if (a > 10) && (b > 10) {
        println!("true");
    }
    let c = 8;
    let d = 37;

    if (c > 10) || (d > 10) {
        println!("true");
    }
    let is_elder = false;

    if !is_elder {
        println!("Not Elder");
    }
}
