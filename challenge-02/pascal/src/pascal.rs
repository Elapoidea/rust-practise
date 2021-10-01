// define the pascal structure.
pub struct Pascal {
    pub triangle: Vec<Vec<u32>>,
}

impl Pascal {
    pub fn new(first_row: Vec<u32>) -> Pascal {
        Pascal {
            triangle: vec![first_row],
        }
    }

    pub fn extend(&mut self, growth: u32) {
        for _ in 0..growth {
            self.next_line()
        }
    }

    pub fn next_line(&mut self) {
        let latest_row = &self.triangle[(self.triangle.len() - 1) as usize];
        let mut new_row: Vec<u32> = Vec::with_capacity(latest_row.len());

        // the point in the latest row which should be considered the halfway
        let halfway =  (latest_row.len() as f32 / 2_f32).ceil() as u32;

        // make half of the new row
        for i in 1..halfway {
            new_row.push(latest_row[i as usize] + latest_row[(i - 1) as usize])
        }

        // copy the row on the other side
        for i in 1..halfway {
            new_row.push(new_row[(halfway - i - 1) as usize])
        }

        // if the row is even, add one more value into the center of the row
        if latest_row.len() % 2 == 0 {
            new_row.insert(halfway as usize - 1, latest_row[halfway as usize] * 2);
        }

        // add the zeroes on the sides
        new_row.insert(0, 0);
        new_row.push(0);

        // add the line to the triangle
        self.triangle.push(new_row.to_vec());
    }
}

// this displays the triangle in a proportioned way
pub fn display(x: Pascal) {
    let cell_size: u32 = x.triangle[x.triangle.len() - 1][x.triangle.len() / 2].to_string().len() as u32;

    for row in x.triangle {
        for value in row {
            if value == 0 { continue }

            print!("{}", {
                let mut cell = String::from("");

                cell.push_str(&value.to_string());

                for _ in 0..cell_size - value.to_string().len() as u32 + 2 {
                    cell.push(' ')
                }

                cell
            })
        }

        println!()
    }
}
