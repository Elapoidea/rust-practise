pub struct Pascal {
    pub triangle: Vec<Vec<u32>>,
}

impl Pascal {
    pub fn new(first_row: Vec<u32>) -> Pascal {
        Pascal {
            triangle: vec![first_row],
        }
    }

    pub fn extend(&mut self, surplus: u32) {
        for _ in 0..surplus {
            self.next_line()
        }
    }

    pub fn next_line(&mut self) {
        let top_row = &self.triangle[(self.triangle.len() - 1) as usize];
        let halfway =  (top_row.len() as f32 / 2_f32).ceil() as u32;
        let mut new_row: Vec<u32> = Vec::with_capacity(top_row.len());

        for i in 1..halfway {
            new_row.push(top_row[i as usize] + top_row[(i - 1) as usize])
        }

        for i in 1..halfway {
            new_row.push(new_row[(halfway - i - 1) as usize])
        }

        if top_row[halfway as usize] == top_row[(halfway - 1) as usize] {
            new_row.insert(halfway as usize - 1, top_row[halfway as usize] * 2);
        }

        new_row.insert(0, 0);
        new_row.push(0);

        self.triangle.push(new_row.to_vec());
    }
}

pub fn display(x: Pascal) {
    let cell_size: u32 = x.triangle[x.triangle.len() - 1][x.triangle.len() / 2].to_string().len() as u32;

    for i in x.triangle {
        for j in i {
            if j == 0 { continue }

            print!("{}", {
                let mut cell = String::from("");

                cell.push_str(&j.to_string());

                for _ in 0..cell_size - j.to_string().len() as u32 + 2 {
                    cell.push(' ')
                }

                cell
            })
        }

        println!()
    }
}
