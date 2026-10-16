fn main() {
	// category, Title, APS level
	let roles_db = vec![
	// Office Admin
	("Office Admin", "Intern", "APS 1-2"),
	("Office Admin", "Administrator", "APS 3-5"),
	("Office Admin", "Senior Administrator", "APS 5-8"),
	("Office Admin", "Office Manager", "EL1 8-10"),

	//Lawyer
	("Lawyer", "Paralegal", "APS 1-2"),
	("Lawyer", "Junior Associate", "APS 3-5"),
	("Lawyer", "Associate", "APS 5-8"),
	("Lawyer", "Senior Assiocate", "EL1 8-10"),

	//Academic
	("Academic", "Research Assistant", "APS 3-5"),
	("Academic", "Phd Candidate", "APS 5-8"),
	("Academic", "Post-Doc Researcher","EL1 8-10"),

	//Teacher
	("Teacher", "Classroom Teacher", "APS 3-5"),
	("Teacher", "Senior Teacher", "APS 5-8"),
	];

	let search_category = "Lawyer";
	let search_title = "Associate";
	println!("Checking APS level for: {} - {}", search_category, search_title);

	let mut found = false;
	for (role_category, role_title, role_aps_level) in roles_db {
		if 	role_category == search_category && role_title== search_title {
			println!("Match found. APS level: {}", role_aps_level);
			found = true;
			break;
		}
	}
	if found == false {
		println!("No matching role found in the table.");
	}

}