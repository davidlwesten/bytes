use std::cmp;

#[derive(Debug)]
pub struct Viewer {
	pub columns: usize,
	pub width: usize,

	current_column: usize
}

impl Viewer {
	pub fn new(columns: usize, width: usize) -> Viewer {
		Viewer {
			columns,
			width,

			current_column: 0
		}
	}

	pub fn draw(&mut self, data: &[u8]) {
		// continue current row
		let max = cmp::min(data.len(), self.columns - self.current_column);
		for c in 0..max {
			self.draw_character(data[c]);
		}

		// complete row if we're done with it
		if self.current_column + max == self.columns {
			print!("\n");

			// if there's data left, draw it line-by-line
			if max < data.len() {
				let new_slice = &data[max..];
				self.draw_lines(new_slice);
				self.current_column = ((new_slice.len() + self.columns - 1) % self.columns) + 1;
			}
			else {
				self.current_column = 0;
			}
		}
		else {
			self.current_column += max;
		}
	}
}

impl Viewer {
	fn draw_lines(&self, data: &[u8]) {
		// print rows 0 to n-2
		let n = (data.len() + self.columns - 1) / self.columns;
		for r in 0 .. n - 1 {
			let i = r * self.columns;
			for c in 0..self.columns - 1 {
				self.draw_character(data[i + c]);
			}
			self.draw_character(data[i + self.columns - 1]);
			print!("\n");
		}

		// print n-1 row
		let i = (n - 1) * self.columns;
		let last_column = ((data.len() + self.columns - 1) % self.columns) + 1;
		for c in 0..last_column {
			self.draw_character(data[i + c]);
		}
	}

	fn complete_row(&self) {
		if self.current_column > 0 {
			for _ in self.current_column..self.columns {
				print!("{:width$}", "**", width = self.width);
			}
			print!("\n");
		}
	}

	fn draw_character(&self, c: u8) {
		print!("{:width$}", format!("{:#04x}", c), width=self.width);
	}
}

impl Drop for Viewer {
	fn drop(&mut self) {
		self.complete_row();
	}
}
