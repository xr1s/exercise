#![feature(type_name_of_val)]

#[derive(Clone, Copy)]
enum Dir {
    R,
    D,
    L,
    U,
}

impl Dir {
    fn turn(self) -> Self {
        return match self {
            Self::U => Self::R,
            Self::R => Self::D,
            Self::D => Self::L,
            Self::L => Self::U,
        };
    }
}

#[derive(Clone, Copy)]
struct Cursor {
    row: i8,
    col: i8,
    dir: Dir,
}

impl Cursor {
    fn new() -> Self {
        return Self {
            row: 0,
            col: 0,
            dir: Dir::R,
        };
    }

    fn next(&self) -> Self {
        return match self.dir {
            Dir::R => Self {
                row: self.row,
                col: self.col + 1,
                dir: self.dir,
            },
            Dir::D => Self {
                row: self.row + 1,
                col: self.col,
                dir: self.dir,
            },
            Dir::L => Self {
                row: self.row,
                col: self.col - 1,
                dir: self.dir,
            },
            Dir::U => Self {
                row: self.row - 1,
                col: self.col,
                dir: self.dir,
            },
        };
    }

    fn turn(&self) -> Self {
        return Self {
            row: self.row,
            col: self.col,
            dir: self.dir.turn(),
        };
    }

    fn hit(&self, square: &Vec<Vec<i8>>) -> bool {
        let size = square.len() as i8;
        let next = self.next();
        if next.row == size || next.row == -1 || next.col == size || next.col == -1 {
            return true;
        }
        let next = next.next();
        if next.row == size || next.row == -1 || next.col == size || next.col == -1 {
            return false;
        }
        return square[next.row as usize][next.col as usize] == 1;
    }
}

fn length(size: usize) -> usize {
    return (size + 1) * (size + 1) / 2 - 1;
}

fn spiralize(size: usize) -> Vec<Vec<i8>> {
    let mut square = vec![vec![0; size]; size];
    let mut cursor = Cursor::new();
    for _ in 0..length(size) {
        square[cursor.row as usize][cursor.col as usize] = 1;
        if cursor.hit(&square) {
            cursor = cursor.turn();
        }
        cursor = cursor.next();
    }
    return square;
}

fn debug(inside: fn(size: usize, row: usize, col: usize) -> bool) -> Box<dyn Fn(usize) + Sync> {
    Box::new(move |size: usize| {
        let square = spiralize(size);
        for row in 0..size {
            for col in 0..size {
                let draw = if inside(size, row, col) {
                    match square[row][col] {
                        0 => ".",
                        1 => "#",
                        _ => unreachable!(),
                    }
                } else {
                    " "
                };
                print!("{}", draw);
            }
            println!();
        }
        println!();
    })
}

lazy_static::lazy_static! {
    static ref DEBUG_FULL: Box<dyn Fn(usize) + Sync> = debug(|_, _, _| true); // 打印完整的方阵
    static ref DEBUG_U: Box<dyn Fn(usize) + Sync> = debug(|size, row, col| {
        return row + col <= size - 1 && row <= col; // 打印上三角形
    });
    static ref DEBUG_R: Box<dyn Fn(usize) + Sync> = debug(|size, row, col| {
        return row + col >= size - 1 && row <= col; // 打印右三角形
    });
    static ref DEBUG_D: Box<dyn Fn(usize) + Sync> = debug(|size, row, col| {
        return row + col >= size - 1 && row >= col; // 打印下三角形
    });
    static ref DEBUG_L: Box<dyn Fn(usize) + Sync> = debug(|size, row, col| {
        return row + col <= size - 1 && row >= col; // 打印左三角形
    });
    static ref DEBUG_L2: Box<dyn Fn(usize) + Sync> = debug(|size, row, col| {
        return row + col <= size - 1 && row >= col && row != col + 1; // 打印无转折的左三角形
    });
}

fn main() {
    // DEBUG_FULL(9);

    // DEBUG_U(9);
    // DEBUG_R(9);
    // DEBUG_D(9);
    // DEBUG_L(9);
    // DEBUG_L2(9);

    assert_eq!(
        spiralize(5),
        [
            [1, 1, 1, 1, 1],
            [0, 0, 0, 0, 1],
            [1, 1, 1, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 1, 1, 1, 1],
        ],
    );
    assert_eq!(
        spiralize(8),
        [
            [1, 1, 1, 1, 1, 1, 1, 1],
            [0, 0, 0, 0, 0, 0, 0, 1],
            [1, 1, 1, 1, 1, 1, 0, 1],
            [1, 0, 0, 0, 0, 1, 0, 1],
            [1, 0, 1, 0, 0, 1, 0, 1],
            [1, 0, 1, 1, 1, 1, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 1],
            [1, 1, 1, 1, 1, 1, 1, 1],
        ],
    );
}
