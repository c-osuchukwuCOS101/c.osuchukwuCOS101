fn main() {
	let original_price = 210000.00;
	let year1 = original_price - (original_price * 5.0 / 100.0);
	println!("Year 1: {:.2}", year1);

	let year2 = year1 - (year1 * 5.0 / 100.0);
	println!("Year 2: {:.2}", year2);

	let year3 = year2 - (year2 * 5.0 / 100.0);
	println!("Year 3: {:.2}", year3);

	let _depreciation = original_price - year3;
	println!("");
	println!("Value after 3 years: {:.2}", year3) 
}