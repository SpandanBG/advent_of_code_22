#[derive(Debug)]
struct Group(String, String, String);

pub fn get_inputs(inp_raw: &str) -> Vec<String> {
        inp_raw
        .split("\n")
        .map(|each| String::from(each))
        .collect()
}

// Part 1 sol
pub fn reorg(inp: &Vec<String>) -> i32 {
    inp.into_iter()
        .map(|each| each.split_at(each.len() / 2))
        .map(get_common_item)
        .flatten()
        .map(convert_to_priority)
        .sum()
}

// Part 2 sol
pub fn stick_sticker(inp: &Vec<String>) -> i32 {
    group_into_threes(inp)
        .into_iter()
        .map(get_common_item_in_group)
        .flatten()
        .map(convert_to_priority)
        .sum()
}

fn group_into_threes(rucksacks: &Vec<String>) -> Vec<Group> {
    let mut group: Vec<Group> = vec![];

    for i in 0..(rucksacks.len() / 3) {
        group.push(Group(
            rucksacks.get(i * 3).unwrap().clone(),
            rucksacks.get(i * 3 + 1).unwrap().clone(),
            rucksacks.get(i * 3 + 2).unwrap().clone(),
        ))
    }

    group
}

fn get_common_item_in_group(group: Group) -> Vec<char> {
    let (str_1, str_2, str_3) = (group.0, group.1, group.2);
    let mut range = [-1; 52];
    let mut collected = vec![];

    for chr in str_1.chars().into_iter() {
        range[get_index_from_char(&chr)] = 0;
    }

    for chr in str_2.chars().into_iter() {
        let idx = get_index_from_char(&chr);
        if range[idx] == 0 {
            range[idx] = 1;
        }
    }

    for chr in str_3.chars().into_iter() {
        let idx = get_index_from_char(&chr);
        if range[idx] == 1 {
            collected.push(chr);
            range[idx] = 0;
        }
    }

    return collected;
}

/**
 * ASCII Map:
 * A-Z => 65-90
 * a-z => 97-122
*/
fn get_common_item((str_1, str_2): (&str, &str)) -> Vec<char> {
    let mut range = [0; 52]; // [A-Za-z] range
    let mut collected_commons: Vec<char> = vec![];

    // Get count of all chars in the first pocket
    for chr in str_1.chars().into_iter() {
        range[get_index_from_char(&chr)] += 2;
    }

    // Get cont of all chars in the second pocket
    for chr in str_2.chars().into_iter() {
        let idx = get_index_from_char(&chr);
        if range[idx] > 0 {
            collected_commons.push(chr);
            range[idx] = 0; // To prevent re-entring the same char
        }
    }

    collected_commons
}

fn convert_to_priority(chr: char) -> i32 {
    let ascii = chr as u8;
    if ascii >= 65 && ascii <= 90 {
        (ascii - 65 + 27) as i32 // A-Z
    } else {
        (ascii - 96) as i32 // a-z
    }
}

fn get_index_from_char(chr: &char) -> usize {
    let ascii = *chr as u8;
    if ascii >= 65 && ascii <= 90 {
        (ascii - 65) as usize // A-Z
    } else {
        (ascii - 97 + 26) as usize // a-z
    }
}
