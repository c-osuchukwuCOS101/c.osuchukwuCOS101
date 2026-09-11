fn main() {
	// Calculate totals for each item 
	let toshiba_total = 2 * 450000;
	let mac_total = 1 *1500000;
	let hp_total = 3 * 750000;
	let dell_total = 3 * 2850000;
	let acer_total = 1 * 250000;

	// Calculate sum 
	let sum = toshiba_total + mac_total + hp_total + dell_total + acer_total;

	// Calculate average 
	let average = sum as f64 / 5.0;

	// Display results
	println!("Toshiba: {}", toshiba_total);
	println!("Mac: {}", mac_total);
	println!("Hp: {}", hp_total);
	println!("Dell: {}", dell_total);
	println!("Acer: {}", acer_total);
	println!("\nTotal Sum: {}", sum);
	println!("Average: {:.2}", average);
}