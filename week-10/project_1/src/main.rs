struct Laptop {
	brand:String,
	price:u32,
	
}

	impl Laptop {
		fn unit_cost(&self, qty: u32)->u32 {
			self.price * qty
		}
	}
	fn main() {
		let laptops = vec![
		Laptop {brand: String::from("HP"), price: 650_000},
		Laptop {brand: String::from("IBM"), price: 755_000},
		Laptop {brand: String::from("Toshiba"), price: 550_000},
		Laptop {brand: String::from("Dell"), price: 850_000},
		];
		let mut total = 0;
		for laptops in &laptops {
			let unit_cost = laptops.unit_cost(3);
			println!("{}: N{}", laptops.brand, unit_cost);
			total += unit_cost;
		}
		println!("Total unit_cost: N{}", total);
	}

