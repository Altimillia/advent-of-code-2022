use std::collections::HashSet;

pub fn part_one(input: String) -> usize {

    let overlap_assignment_pairs:Vec<AssignmentPair> = input
        .lines()
        .map(|f| AssignmentPair::new(f.to_string()))
        .filter(|assignment_pair| assignment_pair.do_assignments_completely_overlap())
        .collect();

    return overlap_assignment_pairs.len();

}

pub fn part_two(input: String) -> usize {
    let intersect_assignment_pairs:Vec<AssignmentPair> = input
    .lines()
    .map(|f| AssignmentPair::new(f.to_string()))
    .filter(|assignment_pair| assignment_pair.do_assignments_intersect())
    .collect();

    return intersect_assignment_pairs.len();
}


struct Elf {
    assignments: HashSet<i32>,
    low_end: i32,
    high_end: i32
}

 impl Elf {
     pub fn new(assignment_string: String) -> Self {
        let splits:Vec<&str> = assignment_string.split("-").into_iter().collect();
        let (low_end, high_end) = (splits[0].parse::<i32>().unwrap(), splits[1].parse::<i32>().unwrap());
        return Elf { 
            assignments: { (low_end)..(high_end + 1) }.into_iter().collect(),
            low_end: low_end,
            high_end: high_end
        }
     }
 }

struct AssignmentPair {
    elves: Vec<Elf>
}

impl AssignmentPair {
    pub fn new(item_string: String) -> Self {
        let splits = item_string.split(',');
        

        return AssignmentPair { 
            elves: splits.map(|f| Elf::new(f.to_string())).collect()
        };
    }

    pub fn do_assignments_completely_overlap(&self) -> bool {
        let elf_1 = &self.elves[0];
        let elf_2 = &self.elves[1];

        fn full_contain(lhs: &Elf, rhs: &Elf) -> bool {
            return lhs.low_end <= rhs.low_end && lhs.high_end >= rhs.high_end;
        }

        return full_contain(elf_1, elf_2) || full_contain(elf_2, elf_1);
    }

    pub fn do_assignments_intersect(&self) -> bool {
        let elf_1 = &self.elves[0];
        let elf_2 = &self.elves[1];

        let intersection: HashSet<i32> = elf_1.assignments.intersection(&elf_2.assignments).copied().collect();
        let count = intersection.len();

        return count > 0;
    }

}

#[cfg(test)]
mod tests {
    use super::AssignmentPair;

    #[test]
    fn assignment_pair_initializes_elves() {
        let input = r#"2-4,6-8"#;

        let assignment_pair = AssignmentPair::new(input.to_string());

        assert_eq!(assignment_pair.elves.len(), 2);
    }
    

    #[test]
    fn assignment_pair_can_find_overlap() {
        let input = r#"2-8,3-7"#;

        let assignment_pair = AssignmentPair::new(input.to_string());

        assert_eq!(assignment_pair.do_assignments_completely_overlap(), true);
    }

    
    #[test]
    fn assignment_pair_can_find_intersect() {
        let input = r#"5-7,7-9"#;

        let assignment_pair = AssignmentPair::new(input.to_string());

        assert_eq!(assignment_pair.do_assignments_intersect(), true);
    }

    #[test]
    fn assignment_pair_can_find_no_intersect() {
        let input = r#"5-6,7-9"#;

        let assignment_pair = AssignmentPair::new(input.to_string());

        assert_eq!(assignment_pair.do_assignments_intersect(), false);
    }



    #[test]
    fn assignment_pair_can_find_no_overlap() {
        let input = r#"2-4,6-8"#;

        let assignment_pair = AssignmentPair::new(input.to_string());

        assert_eq!(assignment_pair.do_assignments_completely_overlap(), false);
    }
}