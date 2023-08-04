#[derive(Debug, Clone)]
pub struct Pair<T>(T, T);

pub type Input = Vec<Pair<Pair<i32>>>;

pub fn get_inputs(inp_raw: &str) -> Input {
    inp_raw
        .split("\n")
        .map(|each| each.split(",").take(2))
        .map(|each_pair| {
            let pairs: Vec<Pair<i32>> = each_pair
                .map(|section| {
                    let section_set: Vec<&str> = section.split("-").take(2).collect();
                    let section_start: i32 = section_set.get(0).unwrap().parse().unwrap();
                    let section_end: i32 = section_set.get(1).unwrap().parse().unwrap();
                    Pair(section_start, section_end)
                })
                .take(2)
                .collect();
            Pair(
                pairs.get(0).unwrap().to_owned(),
                pairs.get(1).unwrap().to_owned(),
            )
        })
        .collect()
}

// part 1 sol
pub fn get_highest_recommendation(inp: Input) -> usize {
    let high_priority_secs: Input = inp.into_iter().filter(is_contains).collect();
    high_priority_secs.len()
}

// part 2 sol
pub fn get_overlaps(inp: Input) -> usize {
    let high_priority_secs: Input = inp.into_iter().filter(is_overlap).collect();
    high_priority_secs.len()
}

fn is_contains(sections: &Pair<Pair<i32>>) -> bool {
    let outer_covers = sections.0 .0 <= sections.1 .0 && sections.0 .1 >= sections.1 .1;
    let inner_covers = sections.1 .0 <= sections.0 .0 && sections.1 .1 >= sections.0 .1;
    outer_covers || inner_covers
}

fn is_overlap(sections: &Pair<Pair<i32>>) -> bool {
    let outer_overlap = (sections.0 .0 >= sections.1 .0 && sections.0 .0 <= sections.1 .1)
        || (sections.0 .1 >= sections.1 .0 && sections.0 .1 <= sections.1 .1);

    let inner_overlap = (sections.1 .0 >= sections.0 .0 && sections.1 .0 <= sections.0 .1)
        || (sections.1 .1 >= sections.0 .0 && sections.1 .1 <= sections.0 .1);

    return outer_overlap || inner_overlap;
}
