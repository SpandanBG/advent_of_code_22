pub fn get_inputs(inp_raw: &str) -> Vec<Vec<i32>> {
    let input = inp_raw.split("\n\n");
    let input = input.map(|cals| cals.split("\n"));
    let input = input.map(|cals| {
        cals.map(|cal| cal.parse::<i32>().expect("unable to parse str to i32"))
            .collect()
    });

    input.collect()
}

pub fn get_elf_with_most_calories(input: &Vec<Vec<i32>>) -> i32 {
    let mut selected_elf_s_calorie = 0;

    for food_cals in  input.iter() {
        let cals = total_cal(food_cals);
        if cals >= selected_elf_s_calorie {
            selected_elf_s_calorie = cals;
        }
    }

    selected_elf_s_calorie
}

pub fn get_top_three_elfs_with_most_calories(input: &Vec<Vec<i32>>) -> i32 {
    let mut thrid_highest = 0;
    let mut second_highest = 0;
    let mut first_hightest = 0;

    for food_cals in  input.iter() {
        let cals = total_cal(food_cals);
        if cals >= first_hightest {
            thrid_highest = second_highest;
            second_highest = first_hightest;
            first_hightest = cals;
        }
        else if cals >= second_highest {
            thrid_highest = second_highest;
            second_highest = cals
        }
        else if cals >= thrid_highest {
            thrid_highest = cals
        }
    }

    first_hightest + second_highest + thrid_highest
}

fn total_cal(food_cals: &Vec<i32>) -> i32 {
    food_cals.iter().sum()
}