use std::fs;
use std::io;

fn show_table(file: &str) {
	let contents = fs::read_to_string(file);
	if contents.is_ok() {
		println!("{}", contents.unwrap());
	} else {
		println!("Could not read the file");
	}
}
fn administrator() {
	println!("Administrator: Whole database structure");
	show_table("administrator.sql");
}
fn project_manager() {
	println!("Project Manager: Project Table");
	show_table("project.sql");
}
fn employee() {
	println!("Employee: Staff Table");
	show_table("staff.sql");
	}
fn customer() {
	println!("Customer: Customer Table");
	show_table("customer.sql");
}
fn vendor() {
	println!("Vendor: Dataplan Table");
	show_table("dataplan.sql");
}
fn main() {
	println!("=========GLOBACOM DATABASE SYSTEM=========");
	println!("1. Administrator");
	println!("2. Project Manager");
	println!("3. Employee");
	println!("4. Customer");
	println!("5. Vendor");
	println!("Choose your role:");
	let mut choice = String::new();
	io::stdin().read_line(&mut choice).unwrap();
	match choice.trim() {
		"1" => administrator(),
		"2" => project_manager(),
		"3" => employee(),
		"4" => customer(),
		"5" => vendor(),
		_ => println!("Invalid choice"),
	}
}