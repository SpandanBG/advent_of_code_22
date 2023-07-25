/**
 * --- Day 5: Supply Stacks ---
 * The expedition can depart as soon as the final supplies have been unloaded
 * from the ships. Supplies are stored in stacks of marked crates, but because
 * the needed supplies are buried under many other crates, the crates need to be
 * rearranged.
 *
 * The ship has a giant cargo crane capable of moving crates between stacks. To
 * ensure none of the crates get crushed or fall over, the crane operator will
 * rearrange them in a series of carefully-planned steps. After the crates are
 * rearranged, the desired crates will be at the top of each stack.
 *
 * The Elves don't want to interrupt the crane operator during this delicate
 * procedure, but they forgot to ask her which crate will end up where, and they
 * want to be ready to unload them as soon as possible so they can embark.
 *
 * They do, however, have a drawing of the starting stacks of crates and the
 * rearrangement procedure (your puzzle input). For example:
 *
 *     [D]    
 * [N] [C]    
 * [Z] [M] [P]
 * 1   2   3
 * move 1 from 2 to 1
 * move 3 from 1 to 3
 * move 2 from 2 to 1
 * move 1 from 1 to 2
 *
 * In this example, there are three stacks of crates. Stack 1 contains two
 * crates: crate Z is on the bottom, and crate N is on top. Stack 2 contains
 * three crates; from bottom to top, they are crates M, C, and D. Finally, stack
 * 3 contains a single crate, P.
 *
 * Then, the rearrangement procedure is given. In each step of the procedure, a
 * quantity of crates is moved from one stack to a different stack. In the first
 * step of the above rearrangement procedure, one crate is moved from stack 2 to
 * stack 1, resulting in this configuration:
 * [D]        
 * [N] [C]    
 * [Z] [M] [P]
 *  1   2   3
 * In the second step, three crates are moved from stack 1 to stack 3. Crates
 * are moved one at a time, so the first crate to be moved (D) ends up below the
 * second and third crates:
 *
 *         [Z]
 *         [N]
 *     [C] [D]
 *     [M] [P]
 *  1   2   3
 * Then, both crates are moved from stack 2 to stack 1. Again, because crates
 * are moved one at a time, crate C ends up below crate M:
 *
 *         [Z]
 *         [N]
 * [M]     [D]
 * [C]     [P]
 *  1   2   3
 * Finally, one crate is moved from stack 1 to stack 2:
 *
 *         [Z]
 *         [N]
 *         [D]
 * [C] [M] [P]
 *  1   2   3
 * The Elves just need to know which crate will end up on top of each stack; in
 * this example, the top crates are C in stack 1, M in stack 2, and Z in stack
 * 3, so you should combine these together and give the Elves the message CMZ.
 *
 * After the rearrangement procedure completes, what crate ends up on top of
 * each stack?
 *
 * --- Part Two ---
 * As you watch the crane operator expertly rearrange the crates, you notice the
 * process isn't following your prediction.
 *
 * Some mud was covering the writing on the side of the crane, and you quickly
 * wipe it away. The crane isn't a CrateMover 9000 - it's a CrateMover 9001.
 *
 * The CrateMover 9001 is notable for many new and exciting features: air
 * conditioning, leather seats, an extra cup holder, and the ability to pick up
 * and move multiple crates at once.
 *
 * Again considering the example above, the crates begin in the same
 * configuration:
 *
 *     [D]    
 * [N] [C]    
 * [Z] [M] [P]
 *  1   2   3
 * Moving a single crate from stack 2 to stack 1 behaves the same as before:
 *
 * [D]        
 * [N] [C]    
 * [Z] [M] [P]
 *  1   2   3
 * However, the action of moving three crates from stack 1 to stack 3 means that
 * those three moved crates stay in the same order, resulting in this new
 * configuration:
 *
 *         [D]
 *         [N]
 *     [C] [Z]
 *     [M] [P]
 *  1   2   3
 * Next, as both crates are moved from stack 2 to stack 1, they retain their
 * order as well:
 *
 *         [D]
 *         [N]
 * [C]     [Z]
 * [M]     [P]
 *  1   2   3
 * Finally, a single crate is still moved from stack 1 to stack 2, but now it's
 * crate C that gets moved:
 *
 *         [D]
 *         [N]
 *         [Z]
 * [M] [C] [P]
 *  1   2   3
 * In this example, the CrateMover 9001 has put the crates in a totally
 * different order: MCD.
 *
 * Before the rearrangement process finishes, update your simulation so that the
 * Elves know where they should stand to be ready to unload the final supplies.
 * After the rearrangement procedure completes, what crate ends up on top of
 * each stack?
*/
use std::fs;

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
        let mut cargo_rows: Vec<&str> = stacks_str.split("\r\n").collect();
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

pub fn get_inputs() -> Input {
    let file = fs::read_to_string("res/_5_supply_stacks.txt").unwrap();

    let inputs: Vec<&str> = file.split("\r\n\r\n").collect();
    let stacks_str = inputs.get(0).unwrap().to_owned();
    let commands_str = inputs.get(1).unwrap().to_owned();

    let cmds: CommandList = commands_str.split("\r\n").map(Command::new).collect();
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
