mod _1_calorie_elf;
// mod _2_rock_paper_sissor;
// mod _3_rucksack_reorg;

use crate::_1_calorie_elf::*;
// use crate::_2_rock_paper_sissor::*;
// use crate::_3_rucksack_reorg::*;

fn main() {
    let inp = get_inputs();

    // # _1_calori_elf
    // 71934 211447(X)
    let ans_1 = get_elf_with_most_calories(&inp);
    let ans_2 = get_top_three_elfs_with_most_calories(&inp);


    // # _2_rock_paper_sissor
    // 9177 12111
    // let ans_1 = score(&inp);
    // let ans_2 = score_v2(&inp);
    

    // # _3_rucksack_reorg
    // 7553
    // let ans_1 = reorg(&inp);
    // let ans_2 = stick_sticker(&inp);

    print!("{ans_1}");
    println!(" {ans_2}");
}