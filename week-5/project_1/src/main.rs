use std::io;

fn main() {
	// Display the menu
	println!("=========== RESTAURANT MENU ==========");
	println!("P - Pounded Yam / Edinkaiko Soup - N3200");
	println!("F - Fried Rice & Chicken - N3000");
	println!("A - Amala & Ewedu Soup - N2500");
	println!("E - Eba & Egusi Soup - N2000");
	println!("W - White Rice & Stew - N2500");

	// Input reading
	let mut food_type = String::new();
	println!("Enter the food type (P, F, A, E, W):");
	io::stdin().read_line(&mut food_type).expect("Failed to read line");

	//Convert to uppercase to handle both lowercase and uppercase inputs
	let food_type = food_type.trim().to_uppercase();

	let mut quantity_str = String::new();
	println!("Enter the quantity:");
	io::stdin().read_line(&mut quantity_str).expect("Failed to read line");
	let quantity:u32 = quantity_str.trim().parse().expect("Please enter a valid number");

	let price: f64 = match food_type.as_str()
	{
		"P" => 3200.0,
		"F" => 3000.0,
		"A" => 2500.0,
		"E" => 2000.0,
		"W" => 2500.0,
		 _ => {
			println!("Invalid food type entered. Please run the program again.");
			0.0 }
	};
	  // Arithmetic for the total
	  let total_charge = price * (quantity as f64);
	  // if for the discount
	  let _final_charge = if total_charge > 10000.0 {
	  	println!("Your total is greater than N10000. A 5% discount has been given!");
	  	total_charge * 0.95 // Apply 5% discount
	  } else {
	  	total_charge
	  };
	  println!("Final Total Charge: N{:.2}", _final_charge);

}