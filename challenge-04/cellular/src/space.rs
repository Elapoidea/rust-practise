pub struct Space {
	pub map: String,
	pub x: usize,
	pub y: usize,
}

impl Space {
	pub fn new(x: usize, y: usize, map: &'static str) -> Self {
		Space { x, y, map: map.to_string()}
	}

	pub fn blank(x: usize, y: usize) -> Self {
		Space { x, y, map: { 
			let mut blank_map = "".to_string();

			for _ in 0..x*y { blank_map.push('0') };

			blank_map
		}}
	}

	pub fn display(&self) {
		for (i, j) in self.map.chars().enumerate() {
			if i % self.x == 0 { println!(); }

			print!("{}", {
				match j {
					'1' => '█',
					_ => '.',
				}
			});
		}
	}

	pub fn assign(&mut self, position: usize, state: char) {
		self.map = { 
			let mut new = "".to_string();

			for (i, j) in self.map.chars().enumerate() {
				if i == position { new.push(state) }
				else { new.push(j) }
			}

			new
		}
	}
}


