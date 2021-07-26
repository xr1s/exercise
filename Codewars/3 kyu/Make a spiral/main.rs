fn draw(size: usize, row: usize, col: usize) -> i8 {
    if col + row <= size - 1 && row <= col {
        return row as i8 % 2 ^ 1;
    }
    if col + row >= size - 1 && row <= col {
        return (size - col) as i8 % 2;
    }
    if col + row >= size - 1 && row >= col {
        return (size - row) as i8 % 2
            ^ (size % 4 == 2 && (row, col) == (size / 2, size / 2 - 1)) as i8;
    }
    if col + row <= size - 1 && row > col {
        return col as i8 % 2 ^ (row != col + 1) as i8;
    }
    unreachable!();
}

fn spiralize(size: usize) -> Vec<Vec<i8>> {
    (0..size)
        .map(|row| (0..size).map(|col| draw(size, row, col)).collect())
        .collect()
}

fn main() {
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
