#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    West,
    South,
}

fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
    return arr
        .into_iter()
        .fold(Vec::with_capacity(arr.len()), |mut v, &next| {
            match (v.last(), next) {
                (Some(Direction::North), Direction::South) => v.pop(),
                (Some(Direction::South), Direction::North) => v.pop(),
                (Some(Direction::East), Direction::West) => v.pop(),
                (Some(Direction::West), Direction::East) => v.pop(),
                _ => {
                    v.push(next);
                    None
                }
            };
            return v;
        });
}

fn main() {
    assert_eq!(
        dir_reduc(&[
            Direction::North,
            Direction::South,
            Direction::South,
            Direction::East,
            Direction::West,
            Direction::North,
            Direction::West
        ]),
        [Direction::West]
    );
    assert_eq!(
        dir_reduc(&[
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East
        ]),
        [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East
        ]
    );
}
