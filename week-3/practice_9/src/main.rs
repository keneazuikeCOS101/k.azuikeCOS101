fn main() {
    let mut cost = 500_000;
    println!("cost is {} ", cost);

    cost = 900_000;
    println!("cost changed to {}", cost);  // will run because the "fees" variable is now mutable
}
