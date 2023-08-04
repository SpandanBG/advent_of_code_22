#[derive(Debug, Clone)]
struct Buff {
    empty_idx: usize,
    chars: Vec<char>,
    size: usize,
}

impl Buff {
    fn new(size: usize) -> Buff {
        Buff {
            empty_idx: 0,
            chars: vec![' '; size],
            size,
        }
    }
    fn clean_buff(&mut self) {
        self.chars = vec![' '; self.size];
        self.empty_idx = 0;
    }

    fn is_buff_full(&self) -> bool {
        return self.empty_idx == self.size;
    }

    fn is_in_buff(&self, chr: char) -> (bool, usize) {
        for i in 0..self.size {
            if self.chars[i] == chr {
                let jump_by = self.empty_idx - i - 1;
                return (true, jump_by);
            }
        }
        (false, 0)
    }

    fn push_to_buff(&mut self, chr: char) {
        if self.empty_idx >= self.size {
            return;
        }
        self.chars[self.empty_idx] = chr;
        self.empty_idx += 1;
    }
}

// part 1 & 2 sol
pub fn find_index(inp: String, size: usize) -> usize {
    let mut i: usize = 0;
    let chars = inp.chars().collect::<Vec<char>>();
    let mut bff = Buff::new(size);

    while i < chars.len() {
        // The buff is full with all unique, so current index is marker
        if bff.is_buff_full() {
            return i;
        }

        let (found, jump_back_by) = bff.is_in_buff(chars[i]);
        if found {
            i -= jump_back_by;
            bff.clean_buff();
        }

        bff.push_to_buff(chars[i]);
        i += 1;
    }

    0
}
