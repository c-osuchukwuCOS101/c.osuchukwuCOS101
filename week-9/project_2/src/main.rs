use std::io::Write;

fn main() {
	let students = [
	("\nOluchi Mardi","ACC10211111","Accounting",300),
	("\nAdams Aliyu","ECO10110101","Economics",100),
	("\nShania Bolade","CSC10328828","Computer",200),
	("\nAdekunle Gold","EEE11020202","Electrical",200),
	("\nBlanca Edemah","MEE10202001","Mechanical",100),
	];

	let mut file = std::fs::File::create("student details.txt").expect("create failed");
	file.write_all("PAU SMIS\n".as_bytes()).expect("write failed");
	for (student_name, matric_number, department, level) in students {
		let students = format!("{} | {} | {} | {}", student_name, matric_number,department,level);
		file.write_all(students.as_bytes()).expect("write failed");
	}
	println!("\nDetails written to file.");
}