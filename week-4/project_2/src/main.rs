use std::io;
fn main() {
	// Create one variable to hold all text input
	let mut a_str = String::new();

	// Ask if the employee is experienced
	println!("Experienced? (yes/no):");
	io::stdin().read_line(&mut a_str).expect("Failed to read input");

	// Check if they typed "yes"
	if a_str.trim() == "yes" {
		a_str.clear();
	}
	// Ask for the employees age
	println!("Enter age:");
	io::stdin().read_line(&mut a_str).expect("Failed to read input");
	let age: u8 =
	a_str.trim().parse().expect("Please enter a valid number");
	
	// Check age and print the matching incentive
	if age >= 40 {
		println!("Incentive: N1_560_000");
	}
	else if age >= 30 {
		println!("Incentive: N1_480_000");
	}
	else if age < 28 {
		println!("Incentive: N1_300_000");
	} else {
		println!("Incentive: N100_000");
	}
}                                 