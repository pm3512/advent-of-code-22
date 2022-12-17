use std::env;
use std::fs;

const CHAMBER_WIDTH: usize = 7;
const CHAMBER_MIN_HEIGHT: usize = 7;
const NUM_ITER: u64 = 1_000_000_000_000;
enum Shape {
    HORIZONTAL,
    PLUS,
    L,
    VERTICAL,
    SQUARE
}

struct Rock {
    shape: Shape,
    x: usize,
    y: usize,
}

impl Rock {
    fn is_valid_pos(&self, x: i64, y: i64, chamber: &Vec<[bool; CHAMBER_WIDTH as usize]>) -> bool {
        if y < 0 || x < 0 {
            return false;
        }

        let x = x as usize;
        let y = y as usize;
        assert!(y <= chamber.len() - 4);

        match self.shape {
            Shape::HORIZONTAL => {
                x <= CHAMBER_WIDTH - 4 &&
                chamber[y][x..=(x + 3)].into_iter().all(|a| *a)
            },
            Shape::PLUS => {
                x <= CHAMBER_WIDTH - 3 &&
                chamber[y][x + 1] &&
                chamber[y + 1][x..=(x + 2)].into_iter().all(|a| *a) &&
                chamber[y + 2][x + 1]
            },
            Shape::L => {
                x <= CHAMBER_WIDTH - 3 &&
                chamber[y][x..=(x + 2)].into_iter().all(|a| *a) &&
                chamber[y + 1][x + 2] &&
                chamber[y + 2][x + 2]
            },
            Shape::VERTICAL => {
                x <= CHAMBER_WIDTH - 1 &&
                chamber[y][x] &&
                chamber[y + 1][x] &&
                chamber[y + 2][x] &&
                chamber[y + 3][x]
            },
            Shape::SQUARE => {
                x <= CHAMBER_WIDTH - 2 &&
                chamber[y][x] &&
                chamber[y][x + 1] &&
                chamber[y + 1][x] &&
                chamber[y + 1][x + 1]
            }
        }
    }

    fn jet_step(&mut self, dir: u8, chamber: &Vec<[bool; CHAMBER_WIDTH]>) {
        if dir == b'<' {
            if self.is_valid_pos(self.x as i64 - 1, self.y as i64, chamber) {
                self.x -= 1;
            }
        } else {
            assert!(dir == b'>');
            if self.is_valid_pos(self.x as i64 + 1, self.y as i64, chamber) {
                self.x += 1;
            }
        }
    }

    fn fall_step(&mut self, chamber: &mut Vec<[bool; CHAMBER_WIDTH as usize]>) -> bool {
        if self.is_valid_pos(self.x as i64, self.y as i64 - 1, chamber) {
            self.y -= 1;
            true
        } else {
            let y = self.y;
            let x = self.x;
            match self.shape {
                Shape::HORIZONTAL => {
                    for dx in 0..=3 {
                        chamber[y][x + dx] = false;
                    }
                },
                Shape::PLUS => {
                    chamber[y][x + 1] = false;
                    chamber[y + 1][x] = false;
                    chamber[y + 1][x + 1] = false;
                    chamber[y + 1][x + 2] = false;
                    chamber[y + 2][x + 1] = false;
                },
                Shape::L => {
                    for dx in 0..=2 {
                        chamber[y][x + dx] = false;
                    }
                    chamber[y + 1][x + 2] = false;
                    chamber[y + 2][x + 2] = false;
                },
                Shape::VERTICAL => {
                    for dy in 0..=3 {
                        chamber[y + dy][x] = false;
                    }
                },
                Shape::SQUARE => {
                    chamber[y][x] = false;
                    chamber[y][x + 1] = false;
                    chamber[y + 1][x] = false;
                    chamber[y + 1][x + 1] = false;
                },
            };
            false
        }
    }
    
    fn max_height(&self) -> usize {
        self.y + match self.shape {
            Shape::HORIZONTAL => 1,
            Shape::PLUS => 3,
            Shape::L => 3,
            Shape::VERTICAL => 4,
            Shape::SQUARE => 2
        }
    }

    fn new(max_height: usize, count: usize) -> Self {
        Rock {
            shape: match count % 5 {
                0 => Shape::HORIZONTAL,
                1 => Shape::PLUS,
                2 => Shape::L,
                3 => Shape::VERTICAL,
                4 => Shape::SQUARE,
                _ => panic!()
            },
            x: 2,
            y: max_height + 3,
        }
    }
    
}

pub fn solve() {
    let args: Vec<String> = env::args().collect();
    let default_filename = String::from("inputs/in1.txt");
    let in_filename = &args.get(1).unwrap_or(&default_filename);

    let jet_pattern = fs::read_to_string(in_filename).unwrap().trim().as_bytes().to_vec();
    let mut chamber: Vec<[bool; CHAMBER_WIDTH]> = Vec::from([[true; CHAMBER_WIDTH]; CHAMBER_MIN_HEIGHT]);
    let mut max_height = 0;
    let mut jet_num = 0;
    
    const CYCLE_LENGTH: u64 = 1750;
    const GAIN_PER_CYCLE: u64 = 2796;
    const REMAINDER: u64 = NUM_ITER % CYCLE_LENGTH;

    for i in 0..REMAINDER {
        let mut rock = Rock::new(max_height, i as usize);
        let mut can_move = true;
        while can_move {
            rock.jet_step(jet_pattern[jet_num], &chamber);
            jet_num += 1;
            jet_num %= jet_pattern.len();
            can_move = rock.fall_step(&mut chamber);
        }
        max_height = max_height.max(rock.max_height());
        while chamber.len() <= max_height + CHAMBER_MIN_HEIGHT {
            chamber.push([true; CHAMBER_WIDTH]);
        }
    }

    println!("{}", NUM_ITER / CYCLE_LENGTH * GAIN_PER_CYCLE + max_height as u64);
}