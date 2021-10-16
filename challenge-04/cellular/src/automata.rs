use crate::space::Space;

impl Space {
	pub fn advance(&mut self) {
		let mut next = Self::blank(self.x, self.y);

		for (position, cell) in self.map.chars().enumerate() {
			let density = self.evaluate_neighbors(position % self.x, position / self.x);
			let new_state = Self::conway(cell, density);

			next.assign(position, new_state)

		}

		self.map = next.map;
	}

	fn evaluate_neighbors(&self, x: usize, y: usize) -> usize {
		let neighbors: Vec<(i8, i8)> = vec![(-1, -1),  (-1, 0),  (-1,  1), 
										    ( 0, -1),            ( 0,  1), 
									        ( 1, -1),  ( 1, 0),  ( 1,  1)];		
		let mut density = 0;

		for adjacent in &neighbors {
			let offset_x = x as i8 - adjacent.0;
			let offset_y = y as i8 - adjacent.1;

			if offset_x < 0 || offset_x >= self.x as i8 { continue }
			if offset_y < 0 || offset_y >= self.y as i8 { continue }			

			let cell_pos = (y * self.y + x) as i8;
			let adj_cell_pos = adjacent.1 * self.y as i8 + adjacent.0;
			let offset = cell_pos - adj_cell_pos;

			density += match self.map.chars().nth(offset as usize) {
				Some('1') => 1,
				_ => 0,
			}
		}

		density							
	}

	fn conway(cell: char, density: usize) -> char {
		match density {
			3 => '1',
			2 => if cell == '1' { '1' } else { '0' },
			_ => '0',
		}		
	}
}