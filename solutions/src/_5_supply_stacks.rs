#[derive(Debug, Clone)]
pub struct Command(usize, usize, usize);

pub type CommandList = Vec<Command>;

impl Command {
    fn new(cmd_str: &str) -> Command {
        let cmd_list: Vec<&str> = cmd_str.split_whitespace().collect();
        let number_of_items: usize = cmd_list.get(1).unwrap().parse().unwrap();
        let from_column: usize = cmd_list.get(3).unwrap().parse().unwrap();
        let to_column: usize = cmd_list.get(5).unwrap().parse().unwrap();

        Command(number_of_items, from_column, to_column)
    }

    fn get_number_of_items(&self) -> usize {
        self.0
    }

    fn get_from_column(&self) -> usize {
        self.1
    }

    fn get_to_column(&self) -> usize {
        self.2
    }
}

pub type Cell = char;
pub type Stack = Vec<Cell>;

pub struct Cargo {
    stacks: Vec<Stack>,
}

impl Cargo {
    fn new(stacks_str: &str) -> Cargo {
        let mut cargo_rows: Vec<&str> = stacks_str.split("\n").collect();
        cargo_rows.reverse();
        let number_of_columns = Cargo::get_number_of_stacks(&cargo_rows);

        let cargo_rows = cargo_rows.into_iter().skip(1).collect::<Vec<&str>>();
        let cargo_rows = Cargo::create_rows_in_cells(cargo_rows, number_of_columns);
        let cargo_stack = Cargo::transform_to_neg_90_deg(&cargo_rows, number_of_columns);

        Cargo {
            stacks: cargo_stack,
        }
    }

    fn exec_cmd(&mut self, cmd: &Command) {
        let from_stack = cmd.get_from_column() - 1;
        let to_stack = cmd.get_to_column() - 1;
        for _ in 0..cmd.get_number_of_items() {
            let item = self.stacks.get_mut(from_stack).unwrap().pop().unwrap();
            self.stacks.get_mut(to_stack).unwrap().push(item);
        }
    }

    fn exec_cmd_v2(&mut self, cmd: &Command) {
        let mut temp_stack: Stack = vec![];

        let from_stack = cmd.get_from_column() - 1;
        let to_stack = cmd.get_to_column() - 1;

        for _ in 0..cmd.get_number_of_items() {
            let item = self.stacks.get_mut(from_stack).unwrap().pop().unwrap();
            temp_stack.push(item);
        }

        for _ in 0..cmd.get_number_of_items() {
            let item = temp_stack.pop().unwrap().to_owned();
            self.stacks.get_mut(to_stack).unwrap().push(item);
        }
    }

    fn top_to_string(&self) -> String {
        let mut base_chars = vec![];

        for stack in self.stacks.iter() {
            base_chars.push(stack.last().unwrap().to_owned())
        }

        base_chars.iter().collect()
    }

    fn get_number_of_stacks(cargo_rows: &Vec<&str>) -> usize {
        cargo_rows
            .get(0)
            .unwrap()
            .split_whitespace()
            .filter(|each| !each.is_empty())
            .collect::<Vec<&str>>()
            .len()
    }

    fn create_rows_in_cells(cargo_rows: Vec<&str>, number_of_columns: usize) -> Vec<Vec<Cell>> {
        let mut rows = vec![];

        for row in cargo_rows.into_iter() {
            let mut row_cells = vec![];

            let row = row.chars().collect::<Vec<char>>();
            for i in 0..number_of_columns {
                let start_idx = i * 4;
                let content = row.get(start_idx + 1).unwrap().to_owned();
                row_cells.push(content);
            }

            rows.push(row_cells);
        }

        rows
    }

    fn transform_to_neg_90_deg(
        cargo_rows: &Vec<Vec<Cell>>,
        number_of_columns: usize,
    ) -> Vec<Stack> {
        let mut stack_list = vec![vec![]; number_of_columns];

        for row in cargo_rows.iter() {
            for (i, cell) in row.iter().enumerate() {
                if *cell == ' ' {
                    continue;
                }
                stack_list[i].push(cell.to_owned());
            }
        }

        return stack_list;
    }
}

pub type Input = (CommandList, Cargo);

pub fn get_inputs(inp_raw: &str) -> Input {
    let inputs: Vec<&str> = inp_raw.split("\n\n").collect();
    let stacks_str = inputs.get(0).unwrap().to_owned();
    let commands_str = inputs.get(1).unwrap().to_owned();
    let cmds: CommandList = commands_str.split("\n").map(Command::new).collect();
    let cargo = Cargo::new(stacks_str);

    (cmds, cargo)
}

// part 1 sol
pub fn rearrange(inp: &mut Input) -> String {
    let (cmds, cargo) = inp;

    for cmd in cmds.iter() {
        cargo.exec_cmd(cmd);
    }

    String::from(cargo.top_to_string())
}

// part 2 sol
pub fn rearrange_v2(inp: &mut Input) -> String {
    let (cmds, cargo) = inp;

    for cmd in cmds.iter() {
        cargo.exec_cmd_v2(cmd);
    }

    String::from(cargo.top_to_string())
}
