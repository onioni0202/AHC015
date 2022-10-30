use std::collections::VecDeque;
use std::io::{stdin, stdout, BufReader, Write};
use proconio::{input, source::line::LineSource};

const BOX_SIZE: usize = 10;
const DCHARS: [char; 4] = ['F', 'B', 'R', 'L'];

#[derive(Debug, Clone)]
struct Box {
    grid: [[usize; BOX_SIZE]; BOX_SIZE],
    score_coeff: f32
}

impl Box {
    fn new(candy_kinds_order: &Vec<usize>) -> Box {
        let mut candy_kinds_count = [0; 4];
        let mut score_coeff = 0.0;
        for &candy_kind in candy_kinds_order {
            candy_kinds_count[candy_kind] += 1;
        }
        for i in 1..4 {
            score_coeff += (candy_kinds_count[i] as f32) * (candy_kinds_count[i] as f32);
        }
        score_coeff = 1e6 / score_coeff;
        Box {
            grid: [[0; BOX_SIZE]; BOX_SIZE],
            score_coeff,
        }
    }

    fn tilt(&mut self, direction: char) {
        match direction {
            'F' => self.tilt_front_side(),
            'B' => self.tilt_back_side(),
            'L' => self.tilt_left_side(),
            'R' => self.tilt_right_side(),
            _ => unreachable!(),
        }
    }

    fn tilt_front_side(&mut self) {
        for w in 0..BOX_SIZE {
            let mut candy_cnt = 0;
            for h in 0..BOX_SIZE {
                if self.grid[h][w] > 0 {
                    self.grid[candy_cnt][w] = self.grid[h][w];
                    if candy_cnt != h {
                        self.grid[h][w] = 0;
                    }
                    candy_cnt += 1;
                }
            }
        }
    }

    fn tilt_back_side(&mut self) {
        for w in 0..BOX_SIZE {
            let mut candy_cnt = 0;
            for h in (0..BOX_SIZE).rev() {
                if self.grid[h][w] > 0 {
                    self.grid[BOX_SIZE - 1 - candy_cnt][w] = self.grid[h][w];
                    if (BOX_SIZE - 1 - candy_cnt) != h {
                        self.grid[h][w] = 0;
                    }
                    candy_cnt += 1;
                }
            }
        }
    }

    fn tilt_left_side(&mut self) {
        for h in 0..BOX_SIZE {
            let mut candy_cnt = 0;
            for w in 0..BOX_SIZE {
                if self.grid[h][w] > 0 {
                    self.grid[h][candy_cnt] = self.grid[h][w];
                    if candy_cnt != w {
                        self.grid[h][w] = 0;
                    }
                    candy_cnt += 1;
                }
            }
        }
    }

    fn tilt_right_side(&mut self) {
        for h in 0..BOX_SIZE {
            let mut candy_cnt = 0;
            for w in (0..BOX_SIZE).rev() {
                if self.grid[h][w] > 0 {
                    self.grid[h][BOX_SIZE - 1 - candy_cnt] = self.grid[h][w];
                    if (BOX_SIZE - 1 - candy_cnt) != w {
                        self.grid[h][w] = 0;
                    }
                    candy_cnt += 1;
                }
            }
        }
    }

    fn put_candy(&mut self, pos_idx: usize, candy_kind: usize) {
        let mut empty_cnt = 0;
        'mainloop: for h in 0..BOX_SIZE {
            for w in 0..BOX_SIZE {
                if self.grid[h][w] == 0 {
                    empty_cnt += 1;
                    if pos_idx == empty_cnt {
                        self.grid[h][w] = candy_kind;
                        break 'mainloop;
                    }
                }
            }
        }
    }

    fn calc_score(&self) -> i64 {
        const DIRECTION: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];
        let mut seen = [[false; BOX_SIZE]; BOX_SIZE];
        let mut score = 0.0;
        for h in 0..BOX_SIZE {
            for w in 0..BOX_SIZE {
                if self.grid[h][w] == 0 {
                    seen[h][w] = true;
                }
                if !seen[h][w] {
                    let candy_kind = self.grid[h][w];
                    let mut candy_cnt = 1;
                    let mut que = VecDeque::new();
                    seen[h][w] = true;
                    que.push_back((h, w));
                    while let Some((fh, fw)) = que.pop_front() {
                        for &(dh, dw) in DIRECTION.iter() {
                            let th = fh.wrapping_add(dh);
                            let tw = fw.wrapping_add(dw);
                            if th >= BOX_SIZE || tw >= BOX_SIZE {
                                continue;
                            }
                            if !seen[th][tw] && self.grid[th][tw] == candy_kind {
                                seen[th][tw] = true;
                                candy_cnt += 1;
                                que.push_back((th, tw));
                            }
                        }
                    }
                    score += self.score_coeff * ((candy_cnt * candy_cnt * candy_cnt) as f32);
                }
            }
        }
        score.round() as i64
    }

    fn print_box(&self) {
        for h in 0..BOX_SIZE {
            for w in 0..BOX_SIZE {
                eprint!("{} ", self.grid[h][w]);
            }
            eprintln!();
        }
    } 
}

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        candy_kinds_order: [usize; BOX_SIZE*BOX_SIZE]
    }
    let mut candy_box = Box::new(&candy_kinds_order);
    for step in 0..100 {
        input! {
            from &mut source,
            pos_idx: usize
        }
        candy_box.put_candy(pos_idx, candy_kinds_order[step]);
        
        let mut best_direction = 'R';
        let mut best_score = -1;
        for &d1 in DCHARS.iter() {
            let mut copied_candy_box = candy_box.clone();
            copied_candy_box.tilt(d1);
            let score1 = copied_candy_box.calc_score();
            for &d2 in DCHARS.iter() {
                copied_candy_box.tilt(d2);
                let score2 = copied_candy_box.calc_score();
                if score1+score2 > best_score {
                    best_score = score1+score2;
                    best_direction = d1;
                }
            }
        }
        candy_box.tilt(best_direction);

        println!("{}", best_direction);
        stdout().flush().unwrap();

        //candy_box.print_box();
        //eprintln!("{}", candy_box.calc_score());
    }
    eprintln!("{}", candy_box.calc_score());
}

