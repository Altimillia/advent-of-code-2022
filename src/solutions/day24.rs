use std::{fmt::Display, collections::{HashSet, HashMap}, time::Instant};

use itertools::Itertools;
use priority_queue::PriorityQueue;

use crate::domain::point::Point;

pub fn part_one(input: String) -> impl Display {
    let grid_state = GridState::parse(input);

    // println!("{:?} - {:?}", grid_state.start, grid_state.end);
    // println!("{}", grid_state.print_grid());
    // let grid_state = grid_state.add_minutes(1);
    // println!("{}", grid_state.print_grid());
    // let grid_state = grid_state.add_minutes(1);
    // println!("{}", grid_state.print_grid());
    let grid_states = get_grid_states(&grid_state, 1000);
    // let result = find_path_djistrkas(grid_state.start, grid_state.end, &grid_state, &Vec::new(), Player { position: grid_state.start }, &grid_states, 0);
    // println!("{:?}", result);

    // for x in grid_states.values() {
    //     println!("{}", x.print_grid());
    // }

    let res = find_path_a_star(grid_state.start, grid_state.end, &grid_state, &grid_states, 0);
    res
}

pub fn part_two(input: String) -> impl Display {
    let grid_state = GridState::parse(input);
    let grid_states = get_grid_states(&grid_state, 1000);    
    let trip_1 = find_path_a_star(grid_state.start, grid_state.end, &grid_state, &grid_states, 0);
    let trip_2 = find_path_a_star(grid_state.end, grid_state.start, &grid_state, &grid_states, trip_1);
    let trip_3 = find_path_a_star(grid_state.start, grid_state.end, &grid_state, &grid_states, trip_2);
    
    trip_3
}


fn get_grid_states(grid_state: &GridState, minutes_to_map: i32) -> HashMap<i32, GridState> {

    let mut grid_states = HashMap::new();
    grid_states.insert(0, grid_state.clone());
    let mut previous_state = grid_state.clone();
    for x in 1..=minutes_to_map {
        let new_state = previous_state.add_minutes(1);
        grid_states.insert(x, new_state.clone());
        previous_state = new_state;
    }

    grid_states
}


fn find_path_a_star(start_point: Point, end_point: Point, grid_state: &GridState, grid_states: &HashMap<i32, GridState>, start_minute: i32) -> i32 {
    let mut frontier:PriorityQueue<(Point, i32), i32> = PriorityQueue::new();
    let mut closed:Vec<(Point, i32)> = Vec::new();
    let possible_moves = vec![Direction::North.to_point(), Direction::East.to_point(), Direction::West.to_point(), Direction:: South.to_point(), Point::new(0, 0)];
    let mut path: HashMap<(Point, i32), (Point, i32)> = HashMap::new();
    let mut cost_map: HashMap<(Point, i32), i32> = HashMap::new();
    let mut timed = i32::MAX;
    frontier.push((start_point, start_minute), 0);
    
    while frontier.len() > 0 { 
        //let timer = Instant::now();
        let popped = frontier.pop().unwrap();
        let (current_pos, current_minutes) = popped.0;
        // let current_minutes = popped.0.1;
        closed.push((current_pos, current_minutes));
        if current_pos == end_point {
            timed = current_minutes;
            break;
        }

        // if current_minutes % 10 == 0 {
        //     println!("{} {}", current_pos, current_minutes);
        // }
        let next_grid_state = &grid_states[&(current_minutes + 1)];
    
        for possible in possible_moves.to_vec() { 
            let next_pos = current_pos + possible;
            if next_pos.x < 0 || next_pos.x > next_grid_state.size.x || next_pos.y < 0 || next_pos.y > next_grid_state.size.y {
                //println!("Out of bounds");
                continue;
            }
            if next_grid_state.walls.contains(&next_pos) {
                //println!("Wall");
                continue;
            }
            if next_grid_state.blizzard_positions.contains(&next_pos) {
                //println!("Blizzard");
                continue;
            }
    
            let cost = *cost_map.get(&(current_pos, current_minutes)).unwrap_or(&0) + 1;
            let next_minutes = current_minutes + 1;
            let next_key = &(next_pos, next_minutes);
            if frontier.get(&(next_pos, next_minutes)).is_some() && cost < *cost_map.get(next_key).unwrap_or(&0) {
                frontier.remove(&(next_pos, next_minutes));
            }

            if closed.contains(next_key) && cost < *cost_map.get(next_key).unwrap_or(&0) {
                closed.retain(|&x| x != *next_key);
            }

            if !frontier.get(next_key).is_some() && !closed.contains(next_key) {

                // let value = path.entry(next_pos).or_insert(current_pos);
                path.insert(*next_key, (current_pos, current_minutes));
                cost_map.insert(*next_key, cost);
                
                let priority = cost + heuristic(next_pos, end_point);
                frontier.push(*next_key, -priority);
            }
        }


        //let time = timer.elapsed();
        //println!("{:.2?}", time);
        // let current_copy;
        // {
        //     let current = &mut grid.get_node(current_pos);
        //     current_copy = current.clone();
        // }
    }

    println!("Path Length: {}", path.len());

    println!("TIME! {}", timed);

    timed
    // println!("Start: {}", path[&start_point]);
    // for x in path.iter() {
    //     println!("{} -> {}", x.0, x.1);
    // }
    // //println!("path length: {:?}", path);
}

fn heuristic(a:Point, b:Point) -> i32 {
    return (a.x - b.x).abs() + (a.y - b.y).abs();
}


fn find_path_djistrkas(start_point: Point, end_point: Point, grid_state: &GridState, path: &Vec<Point>, player: Player, grid_states: &HashMap<i32, GridState>, minutes: i32) -> PathFit {
    let mut paths: Vec<PathFit> = Vec::new();
    let possible_moves = vec![Direction::North.to_point(), Direction::East.to_point(), Direction::West.to_point(), Direction:: South.to_point(), Point::new(0, 0)];

    let next_grid_state = &grid_states[&minutes];
    let blizzard_positions = next_grid_state.blizzards.iter().map(|b| b.position).collect_vec();

    
    for possible in possible_moves {
        if minutes > 25 {
            continue;
        }
        else if minutes < 5 {
        println!("{}", minutes);
        }
        let next_pos = player.position + possible;

        if next_pos.x < 0 || next_pos.x >= next_grid_state.size.x || next_pos.y < 0 || next_pos.y >= next_grid_state.size.y {
            continue;
        }
        if next_grid_state.walls.contains(&next_pos) {
            continue;
        }
        if blizzard_positions.contains(&next_pos) {
            continue;
        }

        let elapsed = minutes + 1;
        let mut branch = path.clone();
        branch.push(next_pos);

        let full_path = find_path_djistrkas(start_point, end_point, &next_grid_state, path, Player { position: next_pos }, grid_states, elapsed);

        let mut add_path = path.clone();
        add_path.extend(full_path.path);

        if add_path.contains(&end_point) {
            paths.push(PathFit { path: add_path, minutes: full_path.minutes + 1 });
            break;
        }
    }

    let mut best_path = PathFit { path: Vec::new(), minutes: i32::MAX };
    for path_fit in paths {
        if path_fit.minutes < best_path.minutes {
            best_path = path_fit;
        }
    }
    best_path
    
}

#[derive(Debug, Clone, Copy)]
struct Player {
    position: Point
}

#[derive(Debug, Clone)]
struct PathFit {
    path: Vec<Point>,
    minutes: i32
}

// fn path_find(start_point: Point, end_point: Point, grid: &mut Grid) -> i32 {
//     let mut frontier:PriorityQueue<Point, i32> = PriorityQueue::new();
//     let mut closed:Vec<Point> = Vec::new();

//     let start = grid.get_node(start_point);
//     frontier.push(start.position, 0);

//     while frontier.len() > 0 { 

//         let popped = frontier.pop().unwrap();

//         let current_pos = popped.0;
//         let current_copy;
//         {
//             let current = &mut grid.get_node(current_pos);
//             current_copy = current.clone();
//         }

//         closed.push(current_copy.position);

//         if current_copy.position == end_point {
//             break;
//         }

        
//         let neighbors = grid.get_neighbors(current_copy.position);

//         let neighbors_count = neighbors.len();
//         for x in 0..neighbors_count {

//             let neighbor_copy;
//             {
//                 let neighbor = &mut grid.get_node(neighbors[x]);
//                 neighbor_copy = neighbor.clone();
//             }

//             if (current_copy.height - neighbor_copy.height) < -1 {
//                 continue;
//             }

//             let cost = current_copy.cost + 1;

//             if frontier.get(&neighbor_copy.position).is_some() && cost < neighbor_copy.cost {
            
//                 frontier.remove(&neighbor_copy.position);
//             }

//             if closed.contains(&neighbor_copy.position) && cost < neighbor_copy.cost {
//                 closed.retain(|&x| x != neighbor_copy.position);
//             }

//             if !frontier.get(&neighbor_copy.position).is_some() && !closed.contains(&neighbor_copy.position) {

//                 grid.update_node(cost, current_copy.position, neighbor_copy.position);
//                 grid.height_map.get_mut(&current_copy.position).unwrap().cost = cost;

//                 let priority = cost + heuristic(neighbor_copy.position, end_point);
//                 frontier.push(neighbor_copy.position, -priority);
//             }
//         }

//     }

//     let mut current_goal = grid.get_node(end_point);

//     let mut counter = 0;
//     while current_goal.prev.is_some() && counter < 2000 {
//         current_goal = grid.get_node(current_goal.prev.unwrap());
//         counter = counter + 1;
//     }
//     return counter;
// }

#[derive(Debug, Clone)]
struct GridState {
    minutes: i32,
    blizzards: Vec<Blizzard>,
    walls: HashSet<Point>,
    blizzard_positions: Vec<Point>,
    size: Point,
    start: Point,
    end: Point
}

impl GridState {
    fn add_minutes(&self, minutes_to_add: i32) -> Self {
        let mut next_state = self.clone();

        // move blizzards
        let mut count = next_state.minutes;
        let target = next_state.minutes + minutes_to_add;
        
        while count < target {
            for blizzard in next_state.blizzards.iter_mut() {
                let mut next_pos = blizzard.position + blizzard.direction.to_point();
                if self.walls.contains(&next_pos) {
                    if next_pos.x == self.size.x {
                        next_pos = Point::new(1, next_pos.y);
                    }
                    else if next_pos.x == 0 {
                        next_pos = Point::new(self.size.x - 1, next_pos.y);
                    }
                    else if next_pos.y == self.size.y {
                        next_pos = Point::new(next_pos.x, 1);
                    }
                    else if next_pos.y == 0 {
                        next_pos = Point::new(next_pos.x, self.size.y - 1);
                    }
                }

                blizzard.position = next_pos;
                
            }

            count += 1;
        }

        next_state.blizzard_positions = next_state.blizzards.iter().map(|b| b.position).collect_vec();
        next_state.minutes = target;

        return next_state;
    }
    fn print_grid(&self) -> String {

        let mut map = String::new();
        map.push_str("\n");
        map.push_str("   ");

        for y in 0..=self.size.y {
            map.push_str("\n");
            
            for x in 0..=self.size.x {
                if self.walls.contains(&Point::new(x, y)) {
                    map.push_str("#");
                    continue;
                }

                let blizzards = self.blizzards.iter().filter(|b| b.position == Point::new(x, y)).collect_vec();
                if blizzards.len() == 1 {
                    match blizzards[0].direction {
                        Direction::North => map.push_str("^"),
                        Direction::East => map.push_str(">"),
                        Direction::South => map.push_str("v"),
                        Direction::West => map.push_str("<"),
                    }
                }
                else if blizzards.len() > 1 {
                    map.push_str(blizzards.len().to_string().as_str());
                }
                else {
                    map.push_str(".")
                }
            }
        }

        return map;
    }
    fn parse(input: String) -> Self {
        let mut blizzards = Vec::new();
        let mut walls = HashSet::new();

        let line_total = input.lines().count();
        let mut y_index = 0;
        let mut start:Option<Point> = None;
        let mut end:Option<Point> = None;
        input
            .lines()
            .enumerate()
            .for_each(|(n, l)| {
                let mut x_index = 0;
                l.chars().enumerate().for_each(|(_,c)| {
                    match c {
                        '#' => { 
                            walls.insert(Point::new(x_index, y_index));
                            },
                        '>' => blizzards.push(Blizzard { direction: Direction::East, position: Point::new(x_index, y_index)}),
                        '<' => blizzards.push(Blizzard { direction: Direction::West, position: Point::new(x_index, y_index)}),
                        '^' => blizzards.push(Blizzard { direction: Direction::North, position: Point::new(x_index, y_index)}),
                        'v' => blizzards.push(Blizzard { direction: Direction::South, position: Point::new(x_index, y_index)}),
                        _ => ()
                    }
                    if n == 0 && c == '.' {
                        start = Some(Point::new(x_index, y_index));
                    }
                    if n == line_total - 1 && c == '.' {
                        end = Some(Point::new(x_index, y_index));   
                    }
                    x_index += 1;
                });
                y_index += 1;
            });

        let max_x = walls.iter().map(|point| point.x).max().unwrap();
        let max_y = walls.iter().map(|point| point.y).max().unwrap();
    
        GridState { minutes: 0, blizzards: blizzards, walls: walls, size: Point::new(max_x,max_y), start: start.unwrap(), end: end.unwrap(), blizzard_positions: Vec::new() }
    }
}

#[derive(Debug, Clone)]
struct Blizzard {
    position: Point,
    direction: Direction
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North = 3,
    East = 0,
    South = 1,
    West = 2
}

impl Direction {
    fn to_point(&self) -> Point {
        match self {
            Direction::North => Point { x: 0, y: -1 },
            Direction::East => Point { x: 1, y: 0},
            Direction::South => Point { x: 0, y: 1 },
            Direction::West =>  Point { x: -1, y: 0},
        }
    }
}
