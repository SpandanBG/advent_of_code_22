pub fn get_inputs(inp_raw: &str) -> Vec<(String, String)> {
    inp_raw
        .split("\n")
        .map(|each| {
            let contestent_inputs: Vec<String> = String::from(each)
                .split_whitespace()
                .map(String::from)
                .collect();

            let input_1 = contestent_inputs.get(0).unwrap().clone();
            let input_2 = contestent_inputs.get(1).unwrap().clone();
            (input_1, input_2)
        })
        .collect()
}

#[derive(PartialEq, Clone)]
enum Resolution {
    Win,
    Draw,
    Loss,
}

impl Resolution {
    fn new(round_end_type: &str) -> Resolution {
        match round_end_type {
            "X" => Resolution::Loss,
            "Y" => Resolution::Draw,
            _ => Resolution::Win,
        }
    }

    fn calc_score(&self, played_shape: &RPS) -> i32 {
        match self {
            Resolution::Loss => played_shape.0,
            Resolution::Draw => 3 + played_shape.0,
            Resolution::Win => 6 + played_shape.0,
        }
    }
}

#[derive(Clone)]
struct RPS(i32);

impl RPS {
    fn new(shape: &str) -> RPS {
        match shape {
            "A" | "X" => RPS(1),
            "B" | "Y" => RPS(2),
            _ => RPS(3),
        }
    }

    fn fight(&self, other: &Self) -> i32 {
        self.fight_resolution(other).calc_score(self)
    }

    fn fight_resolution(&self, other: &Self) -> Resolution {
        if self.0 == other.0 {
            return Resolution::Draw;
        }

        if self.0 == 3 && other.0 == 1 {
            return Resolution::Loss;
        }

        if (self.0 == 1 && other.0 == 3) || self.0 > other.0 {
            return Resolution::Win;
        }

        Resolution::Loss
    }

    fn get_other_shapes(&self) -> (RPS, RPS) {
        let shape_1 = (self.0 + 1) % 3;
        let shape_1 = RPS(if shape_1 == 0 { 3 } else { shape_1 });

        let shape_2 = (self.0 + 2) % 3;
        let shape_2 = RPS(if shape_2 == 0 { 3 } else { shape_2 });

        return (shape_1, shape_2);
    }

    fn shape_base_on_resolution(&self, resolution: Resolution) -> (Self, Resolution) {
        if resolution == Resolution::Draw {
            return (self.clone(), resolution);
        }

        let (shape_1, shape_2) = self.get_other_shapes();
        if shape_1.fight_resolution(self) == resolution {
            return (shape_1, resolution);
        }
        (shape_2, resolution)
    }
}

pub fn score(inp: &Vec<(String, String)>) -> i32 {
    let mut score = 0;

    for (player_1_shape, player_2_shape) in inp.into_iter() {
        score += RPS::new(&player_2_shape).fight(&RPS::new(&player_1_shape));
    }

    score
}

pub fn score_v2(inp: &Vec<(String, String)>) -> i32 {
    inp.into_iter().fold(0, |score, (opp_shape, res_type)| {
        let (my_shape, resolution) =
            RPS::new(&opp_shape).shape_base_on_resolution(Resolution::new(&res_type));
        score + resolution.calc_score(&my_shape)
    })
}
