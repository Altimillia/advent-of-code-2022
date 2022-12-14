use std::{collections::{HashMap, VecDeque}, ops::Index, cell::Cell, borrow::Borrow};

use priority_queue::PriorityQueue;

use crate::domain::point::{NORTH, Point, EAST, WEST, SOUTH};

pub fn part_one(input: String) -> i32 {
    let grid = &mut Grid::new(input.as_str());
    return path_find(grid.start, grid.end, grid);
}

pub fn part_two(input: String) -> i32 {
    let grid = &mut Grid::new(input.as_str());
    let points:Vec<Point> = grid.height_map.values().filter(|x| x.height == 1).map(|g| g.position).collect();

    println!("{}", points.len());
    let shortest_paths:Vec<i32> = points.into_iter().map(|p| {
        return path_find(p, grid.end, &mut Grid::new(input.as_str()));
    }).filter(|p| p > &0).collect();

    return shortest_paths.into_iter().min().unwrap();
}

fn path_find(start_point: Point, end_point: Point, grid: &mut Grid) -> i32 {
    let mut frontier:PriorityQueue<Point, i32> = PriorityQueue::new();
    let mut closed:Vec<Point> = Vec::new();

    let start = grid.get_node(start_point);
    frontier.push(start.position, 0);

    while frontier.len() > 0 { 

        let popped = frontier.pop().unwrap();

        let current_pos = popped.0;
        let current_copy;
        {
            let current = &mut grid.get_node(current_pos);
            current_copy = current.clone();
        }

        closed.push(current_copy.position);

        if current_copy.position == end_point {
            break;
        }

        
        let neighbors = grid.get_neighbors(current_copy.position);

        let neighbors_count = neighbors.len();
        for x in 0..neighbors_count {

            let neighbor_copy;
            {
                let neighbor = &mut grid.get_node(neighbors[x]);
                neighbor_copy = neighbor.clone();
            }

            if (current_copy.height - neighbor_copy.height) < -1 {
                continue;
            }

            let cost = current_copy.cost + 1;

            if frontier.get(&neighbor_copy.position).is_some() && cost < neighbor_copy.cost {
            
                frontier.remove(&neighbor_copy.position);
            }

            if closed.contains(&neighbor_copy.position) && cost < neighbor_copy.cost {
                closed.retain(|&x| x != neighbor_copy.position);
            }

            if !frontier.get(&neighbor_copy.position).is_some() && !closed.contains(&neighbor_copy.position) {

                grid.update_node(cost, current_copy.position, neighbor_copy.position);
                grid.height_map.get_mut(&current_copy.position).unwrap().cost = cost;

                let priority = cost + heuristic(neighbor_copy.position, end_point);
                frontier.push(neighbor_copy.position, -priority);
            }
        }

    }

    let mut current_goal = grid.get_node(end_point);

    let mut counter = 0;
    while current_goal.prev.is_some() && counter < 2000 {
        current_goal = grid.get_node(current_goal.prev.unwrap());
        counter = counter + 1;
    }
    return counter;
}

fn heuristic(a:Point, b:Point) -> i32 {
    return (a.x - b.x).abs() + (a.y - b.y).abs();
}

fn height_to_int(height_char: char) -> i32 {
    if(height_char == 'S')
    {
        return 1;
    }

    if(height_char == 'E') {
        return 26;
    }
    let num:u32 = height_char as u32;

    // Lower Case Unicode, starting with decimal 97 for 'a'
    return (num - 96) as i32;
    
}


struct Grid {
    height_map: HashMap<Point, GridNode>,
    size: Point,
    start: Point,
    end: Point
}


impl Grid {
    fn new(grid_def: &str) -> Grid { 
        let mut grid_map: HashMap<Point, GridNode> = HashMap::new();
        let clone = grid_def.trim().clone();
        let lines = grid_def.trim().lines();
        let mut start:Option<Point> = Option::None;
        let mut end:Option<Point> = Option::None;

        let mut y_index = (clone.lines().count() as i32) - 1;
        lines.for_each(|line| {
            let mut x_index = 0;
            line.chars().for_each(|mountain_height| {
            if mountain_height == 'S' {
                start = Option::Some(Point { x: x_index, y: y_index });
                println!("Start is at {}", Point { x: x_index, y: y_index });
            }
            if mountain_height == 'E' {
                end = Option::Some(Point { x: x_index, y: y_index });
                println!("End is at {}", Point { x: x_index, y: y_index });
            }
                grid_map.insert(Point { x: x_index, y: y_index }, GridNode { position: Point { x: x_index, y: y_index }, height: height_to_int(mountain_height), cost: 0, prev: Option::None, closed: Cell::new(false) });
                x_index = x_index + 1;
            });
            y_index = y_index - 1;
        });
        let max_x = grid_map.keys().map(|pos| pos.x).max().unwrap();
        let max_y = grid_map.keys().map(|pos| pos.y).max().unwrap();
        return Grid { height_map: grid_map, size: Point { x: max_x, y: max_y }, start: start.unwrap(), end: end.unwrap() }
    }


    fn get_neighbors(&self, pos: Point) -> Vec<Point> {
        let directions = [NORTH, EAST, WEST, SOUTH];
        let mut neighbor_points:Vec<Point> = Vec::new();
        directions.iter().for_each(|dir| {
            if self.height_map.contains_key(&(*dir + pos)) {
                neighbor_points.push(*dir + pos);
            }
        });

        return neighbor_points;
    }

    fn get_node_mut(&mut self, pos: Point) -> &GridNode {
        return self.height_map.get(&pos).unwrap();
    }
    fn get_node(&self, pos: Point) -> &GridNode {
        return self.height_map.get(&pos).unwrap();
    }

    fn update_node(&mut self, cost:i32, prev: Point, pos: Point) {
        self.height_map.get_mut(&pos).unwrap().cost = cost;
        self.height_map.get_mut(&pos).unwrap().prev = Option::Some(prev);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
struct GridNode {
    position: Point,
    height: i32,
    cost: i32,
    prev: Option<Point>,
    closed: Cell<bool>
}


#[cfg(test)]
mod tests {
    use super::{Grid, path_find};

    #[test]
    fn path_finding_can_match_example() {
        let input = r#"
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;


        let grid = &mut Grid::new(input);

        let result = path_find(grid.start, grid.end, grid);

        assert_eq!(result, 31);
    }

}