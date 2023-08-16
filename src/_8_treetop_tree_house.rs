use std::fs;

#[derive(Debug)]
pub struct Quadrent {
    width: usize,
    height: usize,
    landscape: Vec<i8>,
}

#[derive(Debug)]
pub struct ViewMap {
    width: usize,
    height: usize,
    map: Vec<Option<i32>>,
}

impl ViewMap {
    fn get(&self, x: usize, y: usize) -> Option<i32> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let idx = Coord::new_raw(x, y, self.width, self.height).to_index();

        if let Some(value) = self.map.get(idx) {
            match value {
                Some(map_value) => Some(*map_value),
                _ => None,
            }
        } else {
            None
        }
    }
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

    fn new_raw(x: usize, y: usize, width: usize, height: usize) -> Coord {
        Coord {
            x,
            y,
            width,
            height,
        }
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

// up:0, left:1, down:2, right:3, it:4
#[derive(Debug, Clone)]
struct HeatCell(i8, i8, i8, i8, i8);

impl HeatCell {
    fn new(it: i8) -> HeatCell {
        HeatCell(-1, -1, -1, -1, it)
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
        if cell.0 < cell.4 || cell.1 < cell.4 || cell.2 < cell.4 || cell.3 < cell.4 {
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
            let higher = if prev_up.0 < prev_up.4 {
                prev_up.4
            } else {
                prev_up.0
            };
            heat_cell.0 = higher;
        } else {
            heat_cell.0 = -1;
        }

        if let Some(left) = center.left() {
            let prev_lf = heat_map.get(left.to_index()).unwrap();
            let higher = if prev_lf.1 < prev_lf.4 {
                prev_lf.4
            } else {
                prev_lf.1
            };
            heat_cell.1 = higher;
        } else {
            heat_cell.1 = -1;
        }

        heat_map[idx] = heat_cell;
    }

    let last_idx = inp.landscape.len() - 1;
    for idx_f in 0..=last_idx {
        let idx = last_idx - idx_f;
        let center = Coord::new(&idx, inp.width, inp.height);

        if let Some(down) = center.down() {
            let prev_dn = heat_map.get(down.to_index()).unwrap();
            let higher = if prev_dn.2 < prev_dn.4 {
                prev_dn.4
            } else {
                prev_dn.2
            };
            heat_map[idx].2 = higher;
        } else {
            heat_map[idx].2 = -1;
        }

        if let Some(right) = center.right() {
            let prev_rt = heat_map.get(right.to_index()).unwrap();
            let higher = if prev_rt.3 < prev_rt.4 {
                prev_rt.4
            } else {
                prev_rt.3
            };
            heat_map[idx].3 = higher;
        } else {
            heat_map[idx].3 = -1;
        }
    }

    heat_map
}

// sol 2
pub fn get_largest_viewing_scene(inp: &Quadrent) -> u32 {
    let x_view_map = generate_x_axis_view_map(inp);
    let y_view_map = generate_y_axis_view_map(inp);

    for (idx, cell) in inp.landscape.iter().enumerate() {
        let center = Coord::new(&idx, inp.width, inp.height);

        let right = fetch_larger_tree_index(
            *cell,
            &x_view_map,
            &center,
            (inp.width - 1 - center.x) as i32,
            |tree_idx, selected_tree_idx| tree_idx > -1 && tree_idx < selected_tree_idx,
        );

        let left = fetch_larger_tree_index(
            *cell, 
            &x_view_map, 
            &center, 
            () as i32,
            | tree_idx, selected_tree_idx| tree_idx < 
        )
    }

    0
}

// fn display_view_map(view_map: &ViewMap) {
//     for (idx, cell) in view_map.map.iter().enumerate() {
//         if idx % view_map.width == 0 {
//             println!()
//         }
//         print!("{}:{}\t", idx % view_map.width, if cell.is_none() { -1 } else { cell.unwrap() as i32 });
//     }
// }

fn fetch_larger_tree_index(
    start_tree_size: i8,
    view_map: &ViewMap,
    start_coord: &Coord,
    base_tree_idx: i32,
    selector_fn: impl Fn(i32, i32) -> bool,
) -> i32 {
    (start_tree_size..10).fold(base_tree_idx, |selected_tree_idx, tree_size| {
        if let Some(tree_index) = view_map.get(tree_size as usize, start_coord.y) {
            if selector_fn(tree_index, selected_tree_idx) {
                return tree_index;
            }
        }
        selected_tree_idx
    })
}

fn generate_x_axis_view_map(inp: &Quadrent) -> ViewMap {
    let mut x_view_map = ViewMap {
        width: 10,
        height: inp.height,
        map: vec![None; 10 * inp.height],
    };

    for (idx, cell) in inp.landscape.iter().enumerate() {
        if idx % inp.width == 0 {
            // skip the first index as it will be our starting looking cell
            continue;
        }

        let coord = Coord::new(&idx, inp.width, inp.height);
        let view_map_index = *cell as usize + coord.y * x_view_map.width;

        // We ignore if it has some value, cause from the left,
        // the repeated value will be furthest
        if x_view_map.map.get(view_map_index).unwrap().is_none() {
            x_view_map.map[view_map_index] = Some(idx as i32 % inp.width as i32);
        }
    }

    x_view_map
}

fn generate_y_axis_view_map(inp: &Quadrent) -> ViewMap {
    let mut y_view_map = ViewMap {
        width: 10,
        height: inp.height,
        map: vec![None; 10 * inp.height],
    };

    for (idx, cell) in inp.landscape.iter().enumerate() {
        if idx / inp.height == 0 {
            // skip the first index as it will be our starting looking cell
            continue;
        }

        let coord = Coord::new(&idx, inp.width, inp.height);
        let view_map_index = *cell as usize + coord.x * y_view_map.width;

        // We ignore if it has some value, cause from the left,
        // the repeated value will be furthest
        if y_view_map.map.get(view_map_index).unwrap().is_none() {
            y_view_map.map[view_map_index] = Some(idx as i32 / inp.height as i32);
        }
    }

    y_view_map
}
