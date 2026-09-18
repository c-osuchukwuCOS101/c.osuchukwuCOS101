use std::io;
fn main(){
	//Create a string to hold user input
	let mut a_str = String::new();

	println!("Enter a:");
	io::stdin().read_line(&mut a_str).expect("Failed to read input");
	let a: f32 = a_str.trim().parse().expect("please enter a number");
	a_str.clear();

	println!("Enter b");
	io::stdin().read_line(&mut a_str).expect("Failed to read input");
	let b: f32 = a_str.trim().parse().expect("please enter a number");
	a_str.clear();

	println!("Enter c");
	io::stdin().read_line(&mut a_str).expect("Failed to read input");
	let c: f32 = a_str.trim().parse().expect("please enter a number");
	a_str.clear();

	// Calculate the discriminant
	let d = b * b - 4.0 * a * c;

	// Check d to find the roots
	if d > 0.0 {
		println!("Roots: {} and {}", (-b + d.sqrt()) / (2.0 * a), (-b - d.sqrt()) / (2.0 * a));
	} else if d == 0.0 {
		println!("Root: {}", -b / (2.0 * a));
	} else {
		println!("No real roots");
	}
}