use std::io::Write;
fn main() {
	let commissioners = ["Aigbogun Alamba Daudu","Murtala Afeez Bendu","Okorocha Calistus Ogbona",
		"Adewale Jimoh Akanbi","Osazuwa Faith Etieye"];
	let ministries = ["Internal affairs","Justice","Defense","Power & Steel","Petroleum"];
	let zones = ["South West","North East","South South","South West","South East"];

	for i in 0..commissioners.len() {
		println!("{} | {} | {}", commissioners[i], ministries[i], zones[i]);
	}
}