mod _1_calorie_counting;
mod _2_rock_paper_scissor;
mod _3_rucksack_reorg;
mod _4_camp_cleanup;
mod _5_supply_stacks;
mod _6_tuning_trouble;
mod _7_no_space;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn hello_world() -> String {
    log("hello, world!");
    String::from("Hello, world")
}

#[wasm_bindgen]
pub fn day_1_1(inp_raw: &str) -> i32 {
    let input = _1_calorie_counting::get_inputs(inp_raw);
    _1_calorie_counting::get_elf_with_most_calories(&input)
}

#[wasm_bindgen]
pub fn day_1_2(inp_raw: &str) -> i32 {
    let input = _1_calorie_counting::get_inputs(inp_raw);
    _1_calorie_counting::get_top_three_elfs_with_most_calories(&input)
}

#[wasm_bindgen]
pub fn day_2_1(inp_raw: &str) -> i32 {
    let input = _2_rock_paper_scissor::get_inputs(inp_raw);
    _2_rock_paper_scissor::score(&input)
}

#[wasm_bindgen]
pub fn day_2_2(inp_raw: &str) -> i32 {
    let input = _2_rock_paper_scissor::get_inputs(inp_raw);
    _2_rock_paper_scissor::score_v2(&input)
}

#[wasm_bindgen]
pub fn day_3_1(inp_raw: &str) -> i32 {
    let input = _3_rucksack_reorg::get_inputs(inp_raw);
    _3_rucksack_reorg::reorg(&input)
}

#[wasm_bindgen]
pub fn day_3_2(inp_raw: &str) -> i32 {
    let input = _3_rucksack_reorg::get_inputs(inp_raw);
    _3_rucksack_reorg::stick_sticker(&input)
}

#[wasm_bindgen]
pub fn day_4_1(inp_raw: &str) -> i64 {
    let input = _4_camp_cleanup::get_inputs(inp_raw);
    _4_camp_cleanup::get_highest_recommendation(input) as i64
}

#[wasm_bindgen]
pub fn day_4_2(inp_raw: &str) -> i64 {
    let input = _4_camp_cleanup::get_inputs(inp_raw);
    _4_camp_cleanup::get_overlaps(input) as i64
}

#[wasm_bindgen]
pub fn day_5_1(inp_raw: &str) -> String {
    let mut input = _5_supply_stacks::get_inputs(inp_raw);
    _5_supply_stacks::rearrange(&mut input)
}

#[wasm_bindgen]
pub fn day_5_2(inp_raw: &str) -> String {
    let mut input = _5_supply_stacks::get_inputs(inp_raw);
    _5_supply_stacks::rearrange_v2(&mut input)
}

#[wasm_bindgen]
pub fn day_6_1(inp_raw: &str) -> i64 {
    _6_tuning_trouble::find_index(String::from(inp_raw), 4) as i64
}

#[wasm_bindgen]
pub fn day_6_2(inp_raw: &str) -> i64 {
    _6_tuning_trouble::find_index(String::from(inp_raw), 14) as i64
}

#[wasm_bindgen]
pub fn day_7_1(inp_raw: &str) -> i32 {
    let input = _7_no_space::get_inputs(inp_raw);
    let disk = &_7_no_space::setup_ssd(&input);
    _7_no_space::get_cleanable_space(disk)
}

#[wasm_bindgen]
pub fn day_7_2(inp_raw: &str) -> i32 {
    let input = _7_no_space::get_inputs(inp_raw);
    let disk = &_7_no_space::setup_ssd(&input);
    _7_no_space::get_cleanable_space_v2(disk)
}