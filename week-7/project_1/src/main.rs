use std::io;
// MTH 101 Project - Shape calculator


fn main() {
	loop {
		println!("\n Shape Caluclator");
		println!("1. Trapezium Area");
		println!("2. Rhombus Area");
		println!("Parallelogram Area");
		println!("Cube Surface Area");
		println!("Cylinder Volume");
		println!("Enter your choice (1-5):");

		let mut choice = String::new();
		io::stdin().read_line(&mut choice).expect("Failed");
		let choice: u32 = choice.trim().parse().expect("Please enter a number");
		match choice {
			1 => println!("Area: {}", trapezium()),
			2 => println!("Area: {}", rhombus()),
			3 => println!("Area: {}", parallelogram()),
			4 => println!("Surface Area: {}", cube()),
			5 => println!("Volume: {}", cylinder()),

		
	 _=>println!("Invalid choice! Please pick 1-5."),
		}
	}
}
fn get_num() -> f64 {
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Failed");
	input.trim().parse().expect("Enter a number")
}

// Shape functions
fn trapezium() -> f64 {
	println!("Enter height:");
	let h = get_num();
	println!("Enter base 1:");
	let b1 = get_num();
	println!("Enter base 2:");
	let b2 = get_num();
	h / 2.0 * (b1 + b2)
}

fn rhombus() -> f64 {
	println!("Enter diagonal 1:");
	let d1 = get_num();
	println!("Enter diagonal 2:");
	let d2 = get_num();
	0.5 * d1 * d2
}

fn parallelogram() -> f64 {
	println!("Enter base:");
	let b = get_num();
	println!("Enter altitude:");
	let a = get_num();
	b * a
}

fn cube() -> f64 {
	println!("Enter side length:");
	let s = get_num();
	6.0 * s * s
}
fn cylinder() -> f64 {
	println!("Enter radius:");
	let r = get_num();
	println!("Enter height:");
	let h = get_num();
	3.142 * r * r * h
}