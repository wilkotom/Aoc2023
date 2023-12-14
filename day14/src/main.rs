use std::{error::Error, collections::{HashMap, hash_map::DefaultHasher}, hash::{Hash, Hasher}};
use aochelpers::{get_daily_input, Coordinate, Direction};

#[derive(Debug,Clone,Copy, PartialEq, Eq)]
enum Rock {
    Round,
    Square
}

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(14,2023)?;
    let mut arena: HashMap<Coordinate<i64>, Rock> = parse_data(&data);
    let boundary = Coordinate{ 
        x: arena.keys().map(|c| c.x).max().unwrap(),
        y: arena.keys().map(|c| c.y).max().unwrap()
    }; 
    
    roll_rocks(&mut arena, &boundary, Direction::North);
    println!("Part 1: {}",support_beam_load(&arena, &boundary, Direction::North));

    let mut cycles = 0;
    let mut seen : HashMap<u64, i128> = HashMap::new(); 
    let mut state = hash_arena(&arena, &boundary);
    while !seen.contains_key(&state) && cycles < 200{
        seen.insert(state.clone(), cycles);
        cycle_rocks(&mut arena, &boundary);
        cycles +=1;
        state = hash_arena(&arena, &boundary);
    }

    let cycles_remaining = (1_000_000 - cycles) %(cycles -seen.get(&state).unwrap());
     for _ in 0..cycles_remaining {
        cycle_rocks(&mut arena, &boundary);
    }
    println!("Part 2: {}",support_beam_load(&arena, &boundary, Direction::North));

    Ok(())
}

fn parse_data(data: &str) -> HashMap<Coordinate<i64>,Rock> {
    let mut arena = HashMap::new();
    for (y, line) in data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'O' => {arena.insert(Coordinate{x:x as i64,y:y as i64}, Rock::Round);},
                '#' => {arena.insert(Coordinate{x:x as i64,y:y as i64}, Rock::Square);},
                '.' => {},
                _ => unimplemented!()
            }
        }
    }
    arena
}

fn cycle_rocks(arena: &mut HashMap<Coordinate<i64>, Rock>, boundary: &Coordinate<i64>) {
    roll_rocks(arena, boundary, Direction::North);
    roll_rocks(arena, boundary, Direction::West);
    roll_rocks(arena, boundary, Direction::South);
    roll_rocks(arena, boundary, Direction::East);
}

fn roll_rocks(arena: &mut HashMap<Coordinate<i64>, Rock>, boundary: &Coordinate<i64>, direction: Direction) {


    match direction {
        Direction::North => {
            for y in 0..=boundary.y {
                for x in 0..=boundary.x {
                    if let Some(&Rock::Round) = arena.get(&Coordinate {x, y}) {
                        let mut target_loc = Coordinate{x,y};
                        while target_loc.y > 0 && !arena.contains_key(&target_loc.neighbour(direction)) {
                            target_loc = target_loc.neighbour(direction)
                        }
                        if target_loc != (Coordinate{x,y}) {
                            arena.insert(target_loc, Rock::Round);
                            arena.remove(&Coordinate{x,y});
                        }
                    }
                }
            }
        },
        Direction::South => { 
            for y in (0..=boundary.y).rev() {
                for x in 0..=boundary.x {
                    if let Some(&Rock::Round) = arena.get(&Coordinate {x, y}) {
                        let mut target_loc = Coordinate{x,y};
                        while target_loc.y < boundary.y && !arena.contains_key(&target_loc.neighbour(direction)) {
                            target_loc = target_loc.neighbour(direction)
                        }
                        if target_loc != (Coordinate{x,y}) {
                            arena.insert(target_loc, Rock::Round);
                            arena.remove(&Coordinate{x,y});
                        }
                    }
                }
            }
        },
        Direction::East => {
            for x in (0..=boundary.x).rev() {
                for y in 0..=boundary.y {
                    if let Some(&Rock::Round) = arena.get(&Coordinate {x, y}) {
                        let mut target_loc = Coordinate{x,y};
                        while target_loc.x < boundary.x && !arena.contains_key(&target_loc.neighbour(direction)) {
                            target_loc = target_loc.neighbour(direction)
                        }
                        if target_loc != (Coordinate{x,y}) {
                            arena.insert(target_loc, Rock::Round);
                            arena.remove(&Coordinate{x,y});
                        }
                    }
                }
            }

        },
        Direction::West => {
            for x in 0..=boundary.x {
                for y in 0..=boundary.y {
                    if let Some(&Rock::Round) = arena.get(&Coordinate {x, y}) {
                        let mut target_loc = Coordinate{x,y};
                        while target_loc.x > 0 && !arena.contains_key(&target_loc.neighbour(direction)) {
                            target_loc = target_loc.neighbour(direction)
                        }
                        if target_loc != (Coordinate{x,y}) {
                            arena.insert(target_loc, Rock::Round);
                            arena.remove(&Coordinate{x,y});
                        }
                    }
                }
            }


        },
        _ => unimplemented!()
    }
}

fn support_beam_load(arena: &HashMap<Coordinate<i64>, Rock>, boundary: &Coordinate<i64>, direction: Direction) -> i64 {
    let mut result = 0;
    match direction {
        Direction::North => {
            for (loc, rock) in arena.iter() {
                if *rock == Rock::Round {
                    result += (boundary.y +1) - loc.y;
                }
            }
        },
        Direction::East => unimplemented!(),
        Direction::South => unimplemented!(),
        Direction::West => unimplemented!(),
        _ => unimplemented!()
    }
    result
}

fn hash_arena(arena: &HashMap<Coordinate<i64>, Rock>,  boundary: &Coordinate<i64>) -> u64 {
    let mut result = Vec::new();
    for y in 0..=boundary.y {
        let mut row: i128 = 0;
        for x in 0..=boundary.x {
            row *= 2;
            if arena.contains_key(&Coordinate { x, y}) {
                row +=1;
            }
        } 
        result.push(row);
    }
    let mut hasher = DefaultHasher::new();
    result.hash(&mut hasher);
    hasher.finish()
    
}

#[cfg(test)]
mod tests {

    use super::*; 

    const EXAMPLE : &str = 
"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

const EXAMPLE_ROLLED: &str = 
".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....";

    #[test]
    fn test_roller() {
        let mut arena: HashMap<Coordinate<i64>, Rock> = parse_data(EXAMPLE);
        let boundary = Coordinate{ x: arena.keys().map(|c| c.x).max().unwrap(),
            y : arena.keys().map(|c| c.y).max().unwrap()}; 
        cycle_rocks(&mut arena, &boundary);
        let cycle_one = parse_data(EXAMPLE_ROLLED);
        assert_eq!(arena, cycle_one);
        for key in arena.keys() {
            assert!(cycle_one.contains_key(key))
        }
    }
}


