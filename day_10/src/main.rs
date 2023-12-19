use core::panic;
use std::{
    collections::{HashSet, VecDeque},
    fs::{self, File},
    io::Write,
};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct Connection {
    pipe_type: char,
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
            pipe_type: '|',
            inlet: Direction::North,
            outlet: Direction::South,
        }),
        '-' => Tile::Pipe(Connection {
            pipe_type: '-',
            inlet: Direction::East,
            outlet: Direction::West,
        }),
        'J' => Tile::Pipe(Connection {
            pipe_type: 'J',
            inlet: Direction::North,
            outlet: Direction::West,
        }),
        'L' => Tile::Pipe(Connection {
            pipe_type: 'L',
            inlet: Direction::North,
            outlet: Direction::East,
        }),
        'F' => Tile::Pipe(Connection {
            pipe_type: 'F',
            inlet: Direction::South,
            outlet: Direction::East,
        }),
        '7' => Tile::Pipe(Connection {
            pipe_type: '7',
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

fn flood_fill(start: (usize, usize), walls: &HashSet<(usize, usize)>, tiles: &Vec<Vec<Tile>>) {
    // implement BFS
    // EDIT: This won't work as is, because the start position is highly dependent and requires
    // more additional processing of the grid.
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited = HashSet::new();
    let mut enclosed_tiles = 0;
    queue.push_back(start);

    while queue.len() > 0 {
        let curr = queue.pop_front().unwrap();

        if walls.contains(&curr)
            || visited.contains(&curr)
            || (curr.0 >= tiles.len() || curr.0 < 0)
            || (curr.1 >= tiles[0].len() || curr.1 < 0)
        {
            continue;
        }

        visited.insert(curr);
        enclosed_tiles += 1;

        for offset in OFFSETS {
            queue.push_back((
                (curr.0 as i32 + offset.0) as usize,
                (curr.1 as i32 + offset.1) as usize,
            ));
        }
    }

    println!("Num enclosed tiles: {}", enclosed_tiles);
}

fn get_start_pipe(
    start: (i32, i32),
    tiles: &Vec<Vec<Tile>>,
    curr_position: &mut (usize, usize),
    from: &mut Direction,
) -> Connection {
    // Since we don't know what kind of pipe S is, we need to first figure out what it is by
    // looking at all neighbourhood pipes (up, down, left, right), and seeing which ones connect
    // to the S tile.
    // IMPORTANT: This does NOT check if the position is potentially out-of-bounds.

    let mut north = false;
    let mut south = false;
    let mut east = false;
    let mut west = false;

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
                        *curr_position = potential_position;
                        *from = Direction::South;
                        north = true;
                    }
                }
                (1, 0) => {
                    // look at the node below S
                    if pipe.inlet == Direction::North || pipe.outlet == Direction::North {
                        *curr_position = potential_position;
                        *from = Direction::North;
                        south = true;
                    }
                }
                (0, -1) => {
                    // look at the node to the left of S
                    if pipe.inlet == Direction::East || pipe.outlet == Direction::East {
                        *curr_position = potential_position;
                        *from = Direction::East;
                        west = true;
                    }
                }
                (0, 1) => {
                    // look at the node to the right of S
                    if pipe.inlet == Direction::West || pipe.outlet == Direction::West {
                        *curr_position = potential_position;
                        *from = Direction::West;
                        east = true;
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    match (north, south, east, west) {
        (true, true, _, _) => Connection {
            pipe_type: '|',
            inlet: Direction::North,
            outlet: Direction::South,
        },
        (true, _, true, _) => Connection {
            pipe_type: 'L',
            inlet: Direction::North,
            outlet: Direction::East,
        },
        (true, _, _, true) => Connection {
            pipe_type: 'J',
            inlet: Direction::North,
            outlet: Direction::West,
        },
        (_, true, true, _) => Connection {
            pipe_type: 'F',
            inlet: Direction::South,
            outlet: Direction::East,
        },
        (_, true, _, true) => Connection {
            pipe_type: '7',
            inlet: Direction::South,
            outlet: Direction::West,
        },
        (_, _, true, true) => Connection {
            pipe_type: '-',
            inlet: Direction::East,
            outlet: Direction::West,
        },
        _ => panic!("No such combination"),
    }
}

fn clean_tiles(
    start: Connection,
    walls: &HashSet<(usize, usize)>,
    tiles: Vec<Vec<Tile>>,
) -> Vec<Vec<Tile>> {
    tiles
        .into_iter()
        .enumerate()
        .map(|(row, tile_line)| {
            tile_line
                .into_iter()
                .enumerate()
                .map(|(col, tile)| match tile {
                    Tile::Start => Tile::Pipe(Connection {
                        pipe_type: start.pipe_type,
                        inlet: start.inlet,
                        outlet: start.outlet,
                    }),
                    Tile::Pipe(_) if walls.contains(&(row, col)) => tile,
                    _ => Tile::Ground,
                })
                .collect()
        })
        .collect()
}

fn scan_line(walls: HashSet<(usize, usize)>, tiles: &Vec<Vec<Tile>>) {
    let (rows, cols) = (tiles.len(), tiles[0].len());
    let mut enclosed_tiles: usize = 0;

    for row in 0..rows {
        let mut is_in_loop = false;
        let mut seen_wall_pipes = 0;
        let mut prior_pipe = '-';

        for col in 0..cols {
            match &tiles[row][col] {
                Tile::Pipe(pipe) => {
                    if (pipe.pipe_type == '|')
                        || (prior_pipe == 'F' && pipe.pipe_type == 'J')
                        || (prior_pipe == 'L' && pipe.pipe_type == '7')
                    {
                        is_in_loop = !is_in_loop;
                    }

                    prior_pipe = pipe.pipe_type;
                }
                _ => {}
            }
            is_in_loop = false;

            // if walls.contains(&(row, col)) {
            //     seen_wall_pipes += 1;
            // } else {
            //     if seen_wall_pipes % 2 != 0 {
            //         // inside the pipe loop
            //         enclosed_tiles += 1;
            //     }
            // }
        }
        println!(
            "After {} rows, enclosed tiles: {}. Number of wall pipes seen: {}",
            row, enclosed_tiles, seen_wall_pipes
        );
    }
    println!("Enclosed tiles: {}", enclosed_tiles);
}

fn visualize_walls(walls: &HashSet<(usize, usize)>, tiles: &Vec<Vec<Tile>>) {
    let mut array = vec![vec!['.'; tiles[0].len()]; tiles.len()];

    for (row, col) in walls {
        // array[*row][*col] = 'X';
        array[*row][*col] = if let Tile::Pipe(pipe) = &tiles[*row][*col] {
            pipe.pipe_type
        } else {
            panic!("Noooo!");
        };
    }

    // Write the array to a text file
    if let Ok(mut file) = File::create("output.txt") {
        for row in &array {
            writeln!(file, "{}", row.iter().collect::<String>()).unwrap();
        }

        println!("Visualization written to 'output.txt'");
    } else {
        eprintln!("Error creating/opening the file.");
    }
}

fn main() {
    // Solution assumes there is no dead-ends and the pipes work in a single direction.
    let loop_contents =
        fs::read_to_string("loop.txt").expect("Should have been able to open file.");
    let (tiles, start) = parse_loop(loop_contents);

    let mut curr_position = (start.0 as usize, start.1 as usize);
    let mut from: Direction = Direction::North;

    let start_pipe = get_start_pipe(start, &tiles, &mut curr_position, &mut from);

    println!(
        "Starting first node after S: {:?}. From the direction: {:?}. Start pipe is a {:?}",
        curr_position, from, start_pipe
    );

    let mut walls = HashSet::new();
    walls.insert((curr_position.0 as usize, curr_position.1 as usize));

    let mut steps = 1;
    while !(curr_position.0 == start.0 as usize && curr_position.1 == start.1 as usize) {
        let curr_tile = &tiles[curr_position.0][curr_position.1];
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

                walls.insert((curr_position.0, curr_position.1));
                steps += 1;
            }
            _ => {}
        }
    }

    println!(
        "Furthest number of steps is exactly half way away: {}",
        ((steps / 2) as f64).ceil()
    );

    let tiles = clean_tiles(start_pipe, &walls, tiles);
    visualize_walls(&walls, &tiles);
    // scan_line(walls, &tiles);
}
