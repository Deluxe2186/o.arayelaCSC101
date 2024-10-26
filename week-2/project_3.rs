fn main() {
	let p:f64 = 510_100.0;
	let r:f64 = 5.0;
	let n:f64 = 3.0;
	let a:f64 = p * (1.0 - (r / 100.0)).powf(n);
	let ci:f64 = a - p;
	println!("Compound interest is {}", ci);
}