fn main() {
    let cost = 500_000;
    println!("cost is {} ", cost);

    cost = 900_000;
    println!("cost changed is {}", cost);  // won't run because the "fees" variable is immutable
}
