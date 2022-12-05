pub fn part_one(input: String) -> i32 {

    let rucksacks:Vec<Rucksack> = input.lines().map(|f| Rucksack::new(f.to_string())).collect();

    let total = rucksacks.into_iter().map(|f| {
        let supply = f.find_shared_item();
        return supply.get_priority();
    }).sum();

    return total;
}

pub fn part_two(input: String) -> i32 {

    let rucksacks:Vec<Rucksack> = input.lines().map(|f| Rucksack::new(f.to_string())).collect();

    return rucksacks
        .chunks(3)
        .map(|rucksacks| SecurityGroup { rucksacks: rucksacks.to_vec() })
        .map(|security_group| security_group.get_badge().get_priority())
        .sum();
}

struct SecurityGroup {
    rucksacks: Vec<Rucksack>
}

impl SecurityGroup {
    fn get_badge(&self) -> Supply {
        let first_rucksack_items = self.rucksacks[0].all_supplies();
        let second_rucksack_items = self.rucksacks[1].all_supplies();
        let third_rucksack_items = self.rucksacks[2].all_supplies();

        let result = first_rucksack_items.into_iter().reduce(|accum, item| {
            if second_rucksack_items.contains(&item) && third_rucksack_items.contains(&item)
            {
                return item;
            }
            return accum;
        });

        return result.unwrap();
    }
}


#[derive(PartialEq, Eq, Clone)]
struct Rucksack {
    left_compartment: Vec<Supply>,
    right_compartment: Vec<Supply>
}

impl Rucksack {
    pub fn new(item_string: String) -> Self {
        let (split1, split2) = item_string.split_at(item_string.len() / 2);

        return Rucksack { 
            left_compartment: split1.chars().map(|f| Supply::new(f)).collect(),
            right_compartment: split2.chars().map(|f| Supply::new(f)).collect(),
        };
    }

    fn find_shared_item(&self) -> &Supply {

        let mut intersect:Option<&Supply> = Option::None;
        self.left_compartment.iter().for_each(|f| {
            if self.right_compartment.contains(f) {
                intersect = Option::Some(f);
            }
        });

        return intersect.unwrap();
    }

    fn all_supplies(&self) -> Vec<Supply> {
        let mut all:Vec<Supply> = Vec::new();
        all.append(&mut self.left_compartment.to_vec());
        all.append(&mut self.right_compartment.to_vec());

        return all;
    }
}

#[derive(PartialEq, Eq, Clone)]
struct Supply {
    item_type: char
}

impl Supply {
    pub fn new(item_type: char) -> Self {

        return Supply { item_type: item_type };
    }
    pub fn get_priority(&self) -> i32 { 
        let num:u32 = self.item_type as u32;

        // Lower Case Unicode, starting with decimal 97 for 'a'
        if num > 91  {
            return (num - 96) as i32;
        }
        // Upper Case Unicode, starting with decimal 41 for 'A'
        return (num - 38) as i32;
    }
}
