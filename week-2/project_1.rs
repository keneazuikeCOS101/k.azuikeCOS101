fn main() {
	let p: f64 = 520_000_000.00;
	let r: f64 = 10.0;
	let n: i32 = 5;

	//total amount
	let a : f64 = p * (1.0 + (r / 100.0)).powi(n);
	println!("Amount is {}", a);
	//compound interest
	let ci: f64 = a - p;
	println!("Compound interest is {}", ci);
}