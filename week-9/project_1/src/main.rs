use std::io::Write;
fn main() {
	let drinks = "Lager: 33 Export, Desperados, Goldberg, Guilder, Heineken, Star
	\nStout: Legend, Turbo King, Williams
	\nNon-Alcoholic: Maltina, Amstel Malta, Malta Gold, Fayrouz";

	let mut file = std::fs::File::create("Nigerian Breweries plc.txt").expect("create failed");
	file.write_all("Welcome to Nigerian Breweries\n".as_bytes()).expect("write failed");
	file.write_all(drinks.as_bytes()).expect("write failed");
	println!("");
}