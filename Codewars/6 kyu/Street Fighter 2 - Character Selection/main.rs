#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const FIGHTERS: [[&str; 6]; 2] = [
    ["Ryu", "E.Honda", "Blanka", "Guile", "Balrog", "Vega"],
    ["Ken", "Chun Li", "Zangief", "Dhalsim", "Sagat", "M.Bison"],
];

fn street_fighter_selection(
    fighters: &[[&str; 6]; 2],
    position: &[i64; 2],
    moves: &[Direction],
) -> Vec<String> {
    use Direction::*;
    return moves
        .iter()
        .fold(
            vec![(position[0] as usize + 1, position[1] as usize + 1)],
            |mut v, next| {
                v.push(match (next, v.last().unwrap()) {
                    (Up, &(1, c)) => (1, c),
                    (Up, &(r, c)) => (r - 1, c),
                    (Down, &(2, c)) => (2, c),
                    (Down, &(r, c)) => (r + 1, c),
                    (Left, &(r, 1)) => (r, 6),
                    (Left, &(r, c)) => (r, c - 1),
                    (Right, &(r, 6)) => (r, 1),
                    (Right, &(r, c)) => (r, c + 1),
                });
                return v;
            },
        )
        .into_iter()
        .skip(1)
        .map(|(r, c)| fighters[r - 1][c - 1].to_string())
        .collect::<Vec<_>>();
}

fn main() {
    use Direction::*;
    assert_eq!(
        street_fighter_selection(&FIGHTERS, &[0, 0], &[Up, Left, Right, Left, Left]),
        ["Ryu", "Vega", "Ryu", "Vega", "Balrog"],
    );
    assert_eq!(
        street_fighter_selection(&FIGHTERS, &[0, 0], &[]),
        [] as [String; 0]
    );
    assert_eq!(
        street_fighter_selection(
            &FIGHTERS,
            &[0, 0],
            &[Left, Left, Left, Left, Left, Left, Left, Left]
        ),
        ["Vega", "Balrog", "Guile", "Blanka", "E.Honda", "Ryu", "Vega", "Balrog"],
    );
    assert_eq!(
        street_fighter_selection(
            &FIGHTERS,
            &[0, 0],
            &[Right, Right, Right, Right, Right, Right, Right, Right]
        ),
        ["E.Honda", "Blanka", "Guile", "Balrog", "Vega", "Ryu", "E.Honda", "Blanka"],
    );
    assert_eq!(
        street_fighter_selection(
            &FIGHTERS,
            &[0, 0],
            &[Up, Left, Down, Right, Up, Left, Down, Right]
        ),
        ["Ryu", "Vega", "M.Bison", "Ken", "Ryu", "Vega", "M.Bison", "Ken"],
    );
    assert_eq!(
        street_fighter_selection(&FIGHTERS, &[0, 0], &[Down, Down, Down, Down]),
        ["Ken", "Ken", "Ken", "Ken"],
    );
    assert_eq!(
        street_fighter_selection(&FIGHTERS, &[0, 0], &[Up, Up, Up, Up]),
        ["Ryu", "Ryu", "Ryu", "Ryu"],
    );
}
