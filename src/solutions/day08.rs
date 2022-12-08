use std::collections::HashMap;
use std::fmt;
use std::ops::{Add, Sub};

const NORTH: Point = Point { x: 0, y: 1};
const SOUTH: Point = Point { x: 0, y: -1};
const EAST:Point = Point { x: 1, y: 0 };
const WEST:Point = Point { x: -1, y: 0 };

pub fn part_one(input: String) -> i32 { 
    let grid = Grid::new(input);

    return grid.get_visible_trees().len() as i32;
}

pub fn part_two(input: String) -> i32 { 
    let grid = Grid::new(input);

    let result = grid.get_max_scenic_score().unwrap();
    println!("The tree is at {} and has a score {}", result.1, result.0);
    return result.0;

}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y}
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

struct Tree {
    height: i32
}

struct Grid {
    trees: HashMap<Point, Tree>,
    size: Point
}

impl Grid {
    pub fn new(grid_string: String) -> Self { 
        const RADIX: u32 = 10;
        let mut grid_map: HashMap<Point, Tree> = HashMap::new();
        let clone = grid_string.trim().clone();
        let lines = grid_string.trim().lines();

        let mut y_index = (clone.lines().count() as i32) - 1;
        lines.for_each(|line| {
            let mut x_index = 0;
            line.chars().for_each(|tree_height| {
                grid_map.insert(Point { x: x_index, y: y_index }, Tree { height: tree_height.to_digit(RADIX).unwrap() as i32} );
                x_index = x_index + 1;
            });
            y_index = y_index - 1;
        });
        let max_x = grid_map.keys().map(|pos| pos.x).max().unwrap();
        let max_y = grid_map.keys().map(|pos| pos.y).max().unwrap();

        return Grid { trees: grid_map, size: Point { x: max_x, y: max_y } }
    }

    fn check_if_tree_is_visible_in_direction(&self, position: Point, direction: Point) -> bool {
        
        let tree = self.trees.get(&position).unwrap();

        let mut current_position: Point = Point { x: position.x, y: position.y };
        current_position = current_position + direction;

        while current_position.x <= self.size.x && current_position.y <= self.size.y && current_position.x >= 0 && current_position.y >= 0 {
            let result = self.trees.get(&current_position);
            if result.is_some() && result.unwrap().height >= tree.height {
                return false;
            }

            current_position = current_position + direction;
        }
        return true;
    }

    fn get_trees_visible_in_direction(&self, position: Point, direction: Point) -> Vec<&Tree> { 
        let tree = self.trees.get(&position).unwrap();
        let mut visible_trees:Vec<&Tree> = Vec::new();

        let mut current_position: Point = Point { x: position.x, y: position.y };
        current_position = current_position + direction;

        while current_position.x <= self.size.x && current_position.y <= self.size.y && current_position.x >= 0 && current_position.y >= 0 {
            let result = self.trees.get(&current_position);
            if result.is_some()  {
                let visible_tree = result.unwrap();
                visible_trees.push(visible_tree);
                if visible_tree.height >= tree.height {
                    return visible_trees;
                }
            }

            current_position = current_position + direction;
        }

        return visible_trees;
    }

    fn check_if_tree_is_visible(&self, position: Point) -> bool {

        let directions = [NORTH, SOUTH, EAST, WEST];

        return directions
            .map(|dir| self.check_if_tree_is_visible_in_direction(position, dir))
            .into_iter()
            .reduce(|accum, value| {
                return accum || value;
            }).unwrap();
    }

    pub fn get_visible_trees(&self) -> Vec<&Tree> {
        let mut visible_trees:Vec<&Tree> = Vec::new();
        
        for x in 0..self.size.x + 1 {
            for y in 0..self.size.y + 1 {
                if self.check_if_tree_is_visible(Point { x, y}) {
                    visible_trees.push(self.trees.get(&Point { x, y }).unwrap());
                }
            }
        }

        return visible_trees;
    }

    fn calculate_tree_scenic_score(&self, position: Point) -> i32 {
        let directions = [NORTH, SOUTH, EAST, WEST];

        return directions
            .map(|dir| self.get_trees_visible_in_direction(position, dir).len())
            .into_iter()
            .reduce(|accum, value| {
                return accum * value;
            }).unwrap() as i32;
    }

    fn get_max_scenic_score(&self) -> Option<(i32, Point)> {
        let result = self
            .trees
            .keys()
            .map(|pos| {
                return (self.calculate_tree_scenic_score(*pos), *pos);
            })
            .reduce(|accum, value| { 
                if value.0 > accum.0 {
                    return value;
                } 
                return accum;
            });

        return result;
    }
}



#[cfg(test)]
mod tests {
    use crate::solutions::day08::{EAST, WEST};

    use super::{Grid, Point};



    #[test]
    fn grid_can_be_initialized_from_string() {
        let input = r#"
30373
25512
65332
33549
35390"#;

        let grid = Grid::new(input.to_string());

        assert_eq!(grid.trees.len(), 25);
    }

    #[test]
    fn grid_can_check_visibility_in_a_direction_for_a_tree() {
        let input = r#"
30373
25512
65332
33549
35390"#;

        let grid = Grid::new(input.to_string());

        let west_visibile = grid.check_if_tree_is_visible_in_direction(Point { x: 1, y: 3 }, Point { x: -1, y: 0 });
        let east_visible = grid.check_if_tree_is_visible_in_direction(Point { x: 1, y: 3 }, Point { x: 1, y: 0 });
        let south_visible = grid.check_if_tree_is_visible_in_direction(Point { x: 1, y: 3 }, Point { x: 0, y: -1 });
        let north_visible = grid.check_if_tree_is_visible_in_direction(Point { x: 1, y: 3 }, Point { x: 0, y: 1 });
        
        assert_eq!(west_visibile, true);
        assert_eq!(east_visible, false);
        assert_eq!(south_visible, false);
        assert_eq!(north_visible, true);
    }

    
    #[test]
    fn grid_can_check_visibility_in_all_directions_for_a_tree() {
        let input = r#"
30373
25512
65332
33549
35390"#;

        let grid = Grid::new(input.to_string());
        let visible = grid.check_if_tree_is_visible(Point { x: 1, y: 3 });
        assert_eq!(visible, true);
        let visible = grid.check_if_tree_is_visible(Point { x: 2, y: 2 });
        assert_eq!(visible, false);
    }

    #[test]
    fn grid_can_get_correct_amount_of_trees_visible() {
        let input = r#"
30373
25512
65332
33549
35390"#;

        let grid = Grid::new(input.to_string());
        let trees = grid.get_visible_trees();

        assert_eq!(trees.len(), 21);

    }

    #[test]
    fn grid_can_find_visible_trees_in_direction() {
        let input = r#"
30373
25512
65332
33549
35390"#;
        
                let grid = Grid::new(input.to_string());
                let east_trees = grid.get_trees_visible_in_direction(Point { x: 2, y: 3}, EAST);
                let west_trees = grid.get_trees_visible_in_direction(Point { x: 2, y: 3}, WEST);
        
                assert_eq!(east_trees.len(), 2); 
                assert_eq!(west_trees.len(), 1); 
    }

    #[test]
    fn trees_on_edge_have_no_trees_visible_in_direction() {
        let input = r#"
30373
25512
65332
33549
35390"#;
        
                let grid = Grid::new(input.to_string());
                let east_trees = grid.get_trees_visible_in_direction(Point { x: 4, y: 3}, EAST);
        
                assert_eq!(east_trees.len(), 0); 
    }

    #[test]
    fn grid_can_calculate_a_trees_scenic_score() {
        let input = r#"
30373
25512
65332
33549
35390"#;
        
                let grid = Grid::new(input.to_string());
                let scenic_score = grid.calculate_tree_scenic_score(Point { x: 2, y: 1});
        
                assert_eq!(scenic_score, 8); 
    }
}