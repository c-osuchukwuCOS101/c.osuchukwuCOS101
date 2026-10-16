fn main() {
	// Name, years of experience
	let candidates = vec![
	(String::from("Ade"), 4),
	(String::from("Aisha"), 9),
	(String::from("Emeka"), 12),
	(String::from("John"), 7),
	(String::from("Crystal"), 17),
	(String::from("Funke"), 11),
	];
	println!("EY Interview Experience Checker");
	println!("Analyzing {} candidates...\n", candidates.len());

	let mut top_candidate_name = String::new();
	let mut max_years = 0;
	for (name, years) in candidates {
		if years > max_years {
			max_years = years;

			top_candidate_name = name;
		}
	}
	println!("The developer with the highest experience is: {}", top_candidate_name);
	println!(" With {} Years of Experience.", max_years);
}