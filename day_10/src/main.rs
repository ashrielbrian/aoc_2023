use std::fs;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct Connection {
    inlet: Direction,
    outlet: Direction,
}

#[derive(Debug)]
enum Tile {
    Pipe(Connection),
    Ground, // .
    Start,  // S
}

const OFFSETS: &[(i32, i32)] = &[
    (-1, 0), // above
    (1, 0),  // below
    (0, -1), // left
    (0, 1),  // right
];

fn char_to_tile(c: char) -> Tile {
    match c {
        '|' => Tile::Pipe(Connection {
            inlet: Direction::North,
            outlet: Direction::South,
        }),
        '-' => Tile::Pipe(Connection {
            inlet: Direction::East,
            outlet: Direction::West,
        }),
        'J' => Tile::Pipe(Connection {
            inlet: Direction::North,
            outlet: Direction::West,
        }),
        'L' => Tile::Pipe(Connection {
            inlet: Direction::North,
            outlet: Direction::East,
        }),
        'F' => Tile::Pipe(Connection {
            inlet: Direction::South,
            outlet: Direction::East,
        }),
        '7' => Tile::Pipe(Connection {
            inlet: Direction::South,
            outlet: Direction::West,
        }),
        '.' => Tile::Ground,
        _ => Tile::Start,
    }
}

fn parse_loop(s: String) -> (Vec<Vec<Tile>>, (i32, i32)) {
    let mut start: (i32, i32) = (0, 0);
    (
        s.lines()
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, c)| {
                        if c == 'S' {
                            start = (i as i32, j as i32);
                        }
                        char_to_tile(c)
                    })
                    .collect::<Vec<Tile>>()
            })
            .collect::<Vec<Vec<Tile>>>(),
        start,
    )
}

fn main() {
    let loop_contents =
        fs::read_to_string("loop.txt").expect("Should have been able to open file.");
    let (tiles, start) = parse_loop(loop_contents);
    // println!("{:?}, {:?}", tiles, start);

    let mut curr_tile = &tiles[start.0 as usize][start.1 as usize];
    let mut curr_position = (start.0 as usize, start.1 as usize);
    let mut from: Direction = Direction::North;

    for offset in OFFSETS {
        let potential_position = (
            (start.0 + offset.0) as usize,
            (start.1 + &offset.1) as usize,
        );
        let first_node = &tiles[potential_position.0][potential_position.1];

        match first_node {
            Tile::Pipe(pipe) => match offset {
                (-1, 0) => {
                    // look at the node above S
                    if pipe.inlet == Direction::South || pipe.outlet == Direction::South {
                        curr_tile = first_node;
                        curr_position = potential_position;
                        from = Direction::South;
                        break;
                    }
                }
                (1, 0) => {
                    // look at the node below S
                    if pipe.inlet == Direction::North || pipe.outlet == Direction::North {
                        curr_tile = first_node;
                        curr_position = potential_position;
                        from = Direction::North;
                        break;
                    }
                }
                (0, -1) => {
                    // look at the node to the left of S
                    if pipe.inlet == Direction::East || pipe.outlet == Direction::East {
                        curr_tile = first_node;
                        curr_position = potential_position;
                        from = Direction::East;
                        break;
                    }
                }
                (0, 1) => {
                    // look at the node to the right of S
                    if pipe.inlet == Direction::West || pipe.outlet == Direction::West {
                        curr_tile = first_node;
                        curr_position = potential_position;
                        from = Direction::West;
                        break;
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    println!(
        "Starting first node after S: {:?} {:?}. From the direction: {:?}",
        curr_position, curr_tile, from
    );

    let mut steps = 1;
    while !(curr_position.0 == start.0 as usize && curr_position.1 == start.1 as usize) {
        match curr_tile {
            Tile::Pipe(pipe) => {
                let to: Direction;
                if from == pipe.inlet {
                    to = pipe.outlet;
                } else {
                    to = pipe.inlet;
                }

                match to {
                    Direction::North => {
                        curr_position = (curr_position.0 - 1, curr_position.1);
                        from = Direction::South;
                    }
                    Direction::South => {
                        curr_position = (curr_position.0 + 1, curr_position.1);
                        from = Direction::North;
                    }
                    Direction::East => {
                        curr_position = (curr_position.0, curr_position.1 + 1);
                        from = Direction::West;
                    }
                    Direction::West => {
                        curr_position = (curr_position.0, curr_position.1 - 1);
                        from = Direction::East;
                    }
                }
                curr_tile = &tiles[curr_position.0][curr_position.1];
                steps += 1;
            }
            _ => {}
        }
    }

    println!("{}", ((steps / 2) as f64).ceil());
}
