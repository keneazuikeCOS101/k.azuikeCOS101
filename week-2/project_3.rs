fn main() {
	let p: f64 = 210_000.00;
	let r: f64 = 5.0;
	let n: i32 = 3;

	//value after depreciation
	let a : f64 = p * (1.0 - (r / 100.0)).powi(n);
	println!("The value after depreciation is {}", a)
	

}