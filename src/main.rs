mod _1_calorie_elf;

use crate::_1_calorie_elf::*;

fn main() {
    let inp = get_inputs();
    let ans_1 = get_elf_with_most_calories(&inp);
    let ans_2 = get_top_three_elfs_with_most_calories(&inp);
    println!("{ans_1} {ans_2}")
}