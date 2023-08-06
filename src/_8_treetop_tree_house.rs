use std::fs;

#[derive(Debug)]
pub struct Quadrent {
    width: usize,
    height: usize,
    landscape: Vec<i8>,
}

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
    height: usize,
    width: usize,
}

impl Coord {
    fn new(idx: &usize, width: usize, height: usize) -> Coord {
        let x = idx % width;
        let y = idx / height;
        return Coord {
            x,
            y,
            height,
            width,
        };
    }

    fn up(&self) -> Option<Self> {
        if self.y == 0 {
            return None;
        }

        Some(Self {
            x: self.x,
            y: self.y - 1,
            width: self.width,
            height: self.height,
        })
    }

    fn down(&self) -> Option<Self> {
        if self.y == self.height - 1 {
            return None;
        }

        Some(Self {
            x: self.x,
            y: self.y + 1,
            width: self.width,
            height: self.height,
        })
    }

    fn left(&self) -> Option<Self> {
        if self.x == 0 {
            return None;
        }

        Some(Self {
            x: self.x - 1,
            y: self.y,
            width: self.width,
            height: self.height,
        })
    }

    fn right(&self) -> Option<Self> {
        if self.x == self.width - 1 {
            return None;
        }

        Some(Self {
            x: self.x + 1,
            y: self.y,
            width: self.width,
            height: self.height,
        })
    }

    fn to_index(&self) -> usize {
        self.y * self.height + self.x
    }
}

#[derive(Debug, Clone)]
struct CellDistM(i8, i32);

// up:0, left:1, down:2, right:3, it:4
#[derive(Debug, Clone)] 
struct HeatCell(CellDistM, CellDistM, CellDistM, CellDistM, i8);

impl HeatCell {
    fn new(it: i8) -> HeatCell {
        let cdm = CellDistM(-1, 0);
        HeatCell(cdm.clone(), cdm.clone(), cdm.clone(), cdm.clone(), it)
    }
}

pub fn get_inputs() -> Quadrent {
    let input: Vec<String> = fs::read_to_string("res/_8_treetop_tree_house.txt")
        .unwrap()
        .split("\r\n")
        .map(|each| String::from(each))
        .collect();

    let width = input.get(0).unwrap().len();
    let height = input.len();

    let mut landscape = vec![];
    for row in input.iter() {
        // ASCII 0 to 9 => 48 to 57
        let mut row_ints: Vec<i8> = row.chars().map(|each| (each as i8) - 48).collect();
        landscape.append(&mut row_ints);
    }

    Quadrent {
        width,
        height,
        landscape,
    }
}

// sol 1
pub fn get_visibility_count(inp: &Quadrent) -> u32 {    
    let mut count = 0;
    for cell in get_heat_map(inp).iter() {
        if cell.0.0 < cell.4 || cell.1.0 < cell.4 || cell.2.0 < cell.4 || cell.3.0 < cell.4 {
            count += 1;
        }
    }

    // for (idx, cell) in heat_map.iter().enumerate() {
    //     if idx % inp.width == 0 {
    //         print!("\n");
    //     }
    //     print!("({}, {})({}, {}){}\t", cell.0, cell.1, cell.2, cell.3, cell.4);
    // }

    count
}

fn get_heat_map(inp: &Quadrent) -> Vec<HeatCell> {
    let mut heat_map = vec![HeatCell::new(0); inp.landscape.len()];

    // Setup Up-Left heat map
    for (idx, cell) in inp.landscape.iter().enumerate() {
        let center = Coord::new(&idx, inp.width, inp.height);
        let mut heat_cell = HeatCell::new(*cell);

        if let Some(up) = center.up() {
            let prev_up = heat_map.get(up.to_index()).unwrap();
            let higher = if prev_up.0.0 < prev_up.4 {
                prev_up.4
            } else {
                prev_up.0.0
            };
            heat_cell.0.0 = higher;
            if heat_cell.4 <= prev_up.4 {
                heat_cell.0.1 = 1;
            } else {
                heat_cell.0.1 = prev_up.0.1 + 1
            }
        } else {
            heat_cell.0 = CellDistM(-1, 0);
        }

        if let Some(left) = center.left() {
            let prev_lf = heat_map.get(left.to_index()).unwrap();
            let higher = if prev_lf.1.0 < prev_lf.4 {
                prev_lf.4
            } else {
                prev_lf.1.0
            };
            heat_cell.1.0 = higher;
            if heat_cell.4 <= prev_lf.4 {
                heat_cell.1.1 = 1;
            } else {
                heat_cell.1.1 = prev_lf.1.1 + 1
            }
        } else {
            heat_cell.1 = CellDistM(-1, 0);
        }

        heat_map[idx] = heat_cell;
    }

    let last_idx = inp.landscape.len() - 1;
    for idx_f in 0..=last_idx {
        let idx = last_idx - idx_f;
        let center = Coord::new(&idx, inp.width, inp.height);

        if let Some(down) = center.down() {
            let higher = if heat_map[down.to_index()].2.0 < heat_map[down.to_index()].4 {
                heat_map[down.to_index()].4
            } else {
                heat_map[down.to_index()].2.0
            };
            heat_map[idx].2.0 = higher;
            if heat_map[idx].4 <= heat_map[down.to_index()].4 {
                heat_map[idx].2.1 = 1;
            } else {
                heat_map[idx].2.1 = heat_map[down.to_index()].2.1 + 1;
            }
        } else {
            heat_map[idx].2 = CellDistM(-1, 0);
        }

        if let Some(right) = center.right() {
            let higher = if heat_map[right.to_index()].3.0 < heat_map[right.to_index()].4 {
                heat_map[right.to_index()].4
            } else {
                heat_map[right.to_index()].3.0
            };
            heat_map[idx].3.0 = higher;
            if heat_map[idx].4 <= heat_map[right.to_index()].4 {
                heat_map[idx].3.1 = 1;
            } else {
                heat_map[idx].3.1 = heat_map[right.to_index()].3.1 + 1;
            }
        } else {
            heat_map[idx].3 = CellDistM(-1, 0);
        }
    }

    heat_map
}