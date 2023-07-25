use std::fs;

pub fn get_inputs() -> Vec<(String, String)> {
    let input =
        fs::read_to_string("res/_2_rock_paper_sissor.txt").expect("unable to open input file");

    input
        .split("\r\n")
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

/**
 * --- Day 2: Rock Paper Scissors ---
 * The Elves begin to set up camp on the beach. To decide whose tent gets to be
 * closest to the snack storage, a giant Rock Paper Scissors tournament is
 * already in progress.
 *
 * Rock Paper Scissors is a game between two players. Each game contains many
 * rounds; in each round, the players each simultaneously choose one of Rock,
 * Paper, or Scissors using a hand shape. Then, a winner for that round is
 * selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats
 * Rock. If both players choose the same shape, the round instead ends in a
 * draw.
 *
 * Appreciative of your help yesterday, one Elf gives you an encrypted strategy
 * guide (your puzzle input) that they say will be sure to help you win. "The
 * first column is what your opponent is going to play: A for Rock, B for Paper,
 * and C for Scissors. The second column--" Suddenly, the Elf is called away to
 * help with someone's tent.
 *
 * The second column, you reason, must be what you should play in response: X
 * for Rock, Y for Paper, and Z for Scissors. Winning every time would be
 * suspicious, so the responses must have been carefully chosen.
 *
 * The winner of the whole tournament is the player with the highest score. Your
 * total score is the sum of your scores for each round. The score for a single
 * round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3
 * for Scissors) plus the score for the outcome of the round (0 if you lost, 3
 * if the round was a draw, and 6 if you won).
 *
 * Since you can't be sure if the Elf is trying to help you or trick you, you
 * should calculate the score you would get if you were to follow the strategy
 * guide.
 *
 * For example, suppose you were given the following strategy guide:
 *
 * A Y
 * B X
 * C Z
 *
 * This strategy guide predicts and recommends the following:
 *
 * In the first round, your opponent will choose Rock (A), and you should choose
 * Paper (Y). This ends in a win for you with a score of 8 (2 because you chose
 * Paper + 6 because you won).
 *
 * In the second round, your opponent will choose Paper (B), and you should
 * choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
 *
 * The third round is a draw with both players choosing Scissors, giving you a
 * score of 3 + 3 = 6.
 *
 * In this example, if you were to follow the strategy guide, you would get a
 * total score of 15 (8 + 1 + 6).
 *
 * What would your total score be if everything goes exactly according to your
 * strategy guide?
*/
pub fn score(inp: &Vec<(String, String)>) -> i32 {
    let mut score = 0;

    for (player_1_shape, player_2_shape) in inp.into_iter() {
        score += RPS::new(&player_2_shape).fight(&RPS::new(&player_1_shape));
    }

    score
}

/**
 * --- Part Two ---
 * The Elf finishes helping with the tent and sneaks back over to you. "Anyway,
 * the second column says how the round needs to end: X means you need to lose,
 * Y means you need to end the round in a draw, and Z means you need to win.
 * Good luck!"
 *
 * The total score is still calculated in the same way, but now you need to
 * figure out what shape to choose so the round ends as indicated. The example
 * above now goes like this:
 *
 * In the first round, your opponent will choose Rock (A), and you need the
 * round to end in a draw (Y), so you also choose Rock. This gives you a score
 * of 1 + 3 = 4.
 *
 * In the second round, your opponent will choose Paper (B), and you choose Rock
 * so you lose (X) with a score of 1 + 0 = 1.
 *
 * In the third round, you will defeat your opponent's Scissors with Rock for a
 * score of 1 + 6 = 7.
 *
 * Now that you're correctly decrypting the ultra top secret strategy guide, you
 * would get a total score of 12.
 *
 * Following the Elf's instructions for the second column, what would your total
 * score be if everything goes exactly according to your strategy guide?
*/
pub fn score_v2(inp: &Vec<(String, String)>) -> i32 {
    inp.into_iter().fold(0, |score, (opp_shape, res_type)| {
        let (my_shape, resolution) =
            RPS::new(&opp_shape).shape_base_on_resolution(
                Resolution::new(&res_type),
            );
        score + resolution.calc_score(&my_shape)
    })
}
