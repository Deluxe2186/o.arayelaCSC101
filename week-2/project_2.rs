fn main() {
	let toshiba:f64 = 450_000.0;
	let mac:f64 = 1_500_000.0;
	let hp:f64 = 750_000.0;
	let dell:f64 = 2_850_000.0;
	let acer:f64 = 250_000.0;
	let amount = 2.0 * toshiba + mac + 3.0 * hp + 3.0 * dell + acer;
	let avg = amount / (2.0 + 1.0 + 3.0 + 3.0 + 1.0);
    println!("The average of the sales record is {}", avg);
}