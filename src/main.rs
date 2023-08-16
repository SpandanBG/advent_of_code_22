// mod _1_calorie_elf;
// mod _2_rock_paper_sissor;
// mod _3_rucksack_reorg;
// mod _4_camp_cleanup;
// mod _5_supply_stacks;
// mod _6_tuning_trouble;
// mod _7_no_space;
mod _8_treetop_tree_house;

// use crate::_1_calorie_elf::*;
// use crate::_2_rock_paper_sissor::*;
// use crate::_3_rucksack_reorg::*;
// use crate::_4_camp_cleanup::*;
// use crate::_5_supply_stacks::*;
// use crate::_6_tuning_trouble::*;
// use crate::_7_no_space::*;
use crate::_8_treetop_tree_house::*;

fn main() {
    let inp = get_inputs();

    // # _1_calori_elf
    // 71934 211447(X)
    // let ans_1 = get_elf_with_most_calories(&inp);
    // let ans_2 = get_top_three_elfs_with_most_calories(&inp);

    // # _2_rock_paper_sissor
    // 9177 12111
    // let ans_1 = score(&inp);
    // let ans_2 = score_v2(&inp);

    // # _3_rucksack_reorg
    // 7553 2758
    // let ans_1 = reorg(&inp);
    // let ans_2 = stick_sticker(&inp);

    // # _4_camp_cleanup
    // 509 870
    // let ans_1 = get_highest_recommendation(inp);
    // let ans_1 = get_overlaps(inp);

    // # _5_supply_stacks
    // TWSGQHNHL JNRSCDWPP
    // let ans_1 = rearrange(&mut inp);
    // let ans_1 = rearrange_v2(&mut inp);

    // # _6_tuning_trouble
    // 1723 3708
    // let ans_1 = find_index(inp, 4);
    // let ans_1 = find_index(inp, 14);

    // # _7_no_space
    // Elapsed: 231
    // 1325919
    // let ssd = setup_ssd(&inp);
    // let ans_1 = get_cleanable_space(&ssd);
    // let ans_2 = get_cleanable_space_v2(&ssd);

    // # _8_treetop_tree_house
    //
    // let ans_1 = get_visibility_count(&inp);
    let ans_2 = get_largest_viewing_scene(&inp);

    // print!("{ans_1}");
    println!(" {ans_2}");
}
