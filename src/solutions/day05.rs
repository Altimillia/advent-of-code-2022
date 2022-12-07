pub fn part_one(input: String) -> String { 

    let splits:Vec<&str> = input.split("\n\n").into_iter().collect();

    let storage = Storage::new(splits[0].to_string());


    let instructions:Vec<CraneInstruction> = splits[1]
        .to_string()
        .lines()
        .map(|l| CraneInstruction::new(l.to_string()))
        .collect();

    return run_instructions(storage, instructions);
}

pub fn part_two(input: String) -> String { 
    let splits:Vec<&str> = input.split("\n\n").into_iter().collect();

    let mut storage = Storage::new(splits[0].to_string());

    splits[1]
        .to_string()
        .lines()
        .map(|l| CraneInstruction::new(l.to_string()))
        .for_each(|ins| {
            storage.pop_from_storage_stack_9001(ins.source as usize, ins.amount, ins.destination as usize);
        });


    return storage.get_top_row_string();
}

fn run_instructions(mut storage: Storage, instructions: Vec<CraneInstruction>) -> String
{
    instructions.into_iter().for_each(|ins| {
        storage.pop_from_storage_stack(ins.source as usize, ins.amount, ins.destination as usize);
    });

    return storage.get_top_row_string().to_string();
}


struct CraneInstruction {
    amount: i32,
    source: i32,
    destination: i32
}

impl CraneInstruction {
    pub fn new(instruction_string: String) -> Self {
        let split: Vec<&str> = instruction_string.split_ascii_whitespace().into_iter().collect();

        return CraneInstruction { 
            amount: split[1].parse::<i32>().unwrap(), 
            source: split[3].parse::<i32>().unwrap(), 
            destination: split[5].parse::<i32>().unwrap() 
        }
    }
}



struct Storage {
    crate_stacks: Vec<CrateStack>
}

impl Storage {

    pub fn new(item_string: String) -> Self {
        let last_line = item_string.lines().last().unwrap();

        fn create_stack(storage_string: String, index: usize) -> CrateStack {

            let mut crate_characters:Vec<char> = Vec::new();

            for x in (0..storage_string.lines().count() - 1).rev() {
                let crate_char = storage_string
                    .lines()
                    .nth(x)
                    .unwrap()
                    .chars()
                    .nth(index)
                    .unwrap();
                    
                if crate_char != ' ' { 
                    crate_characters.push(crate_char);
                }
            }

            return CrateStack { crates: crate_characters };
        }

        let mut stacks: Vec<CrateStack> = Vec::new();

        for x in 0..last_line.len() {
            let stack_number = last_line.chars().nth(x).unwrap();
            if stack_number != ' '  {
                stacks.push(create_stack(item_string.to_string(), x));
            }
        }

        return Storage { crate_stacks: stacks }
    }

    pub fn get_crate_stack_mut(&mut self, number: usize) -> &mut CrateStack {
        return self.crate_stacks.get_mut(number - 1).unwrap();
    }

    pub fn pop_from_storage_stack(&mut self, source: usize, amount: i32, destination: usize) {
        let source_stack = self.get_crate_stack_mut(source);
        let crates = source_stack.pop_crates(amount);

        let destination_stack = self.get_crate_stack_mut(destination);
        destination_stack.push_crates(crates);
    }

    pub fn pop_from_storage_stack_9001(&mut self, source: usize, amount: i32, destination: usize) { 
        let source_stack = self.get_crate_stack_mut(source);
        let crates = source_stack.pop_crates(amount);

        let destination_stack = self.get_crate_stack_mut(destination);
        destination_stack.push_crates_9001(crates);
    }

    pub fn print_stacks(&self) {
        self.crate_stacks.clone().into_iter().for_each(|f| {
            f.crates.clone().into_iter().for_each(|cs| {
                print!("{}",cs);
            });
            println!("");
        });
    }

    pub fn get_top_row_string(&self) -> String {
        let characters:String = self.crate_stacks.clone().into_iter().map(|f| f.peek()).collect();

        return characters;
    }
}

#[derive(PartialEq, Eq, Clone)]
struct CrateStack {
    crates: Vec<char>
}

impl CrateStack {
    
    fn pop_crates(&mut self, amount: i32) -> Vec<char> {
        let mut popped_crates: Vec<char> = Vec::new();
        for x in 0..amount {
            popped_crates.push(self.crates.pop().unwrap());
        }

        return popped_crates;
    }

    fn peek(&self) -> char {
        return *self.crates.last().unwrap();
    }

    fn push_crates(&mut self, crates: Vec<char>) {
        crates.into_iter().for_each(|c| {
            self.crates.push(c);
        });
    }
    fn push_crates_9001(&mut self, crates: Vec<char>) {
        for x in (0..crates.len()).rev() {
            self.crates.push(*crates.get(x).unwrap());
        }
    }
}

mod tests {

    use crate::solutions::day05::CrateStack;
    use super::{CraneInstruction, Storage};

    #[test]
    fn crane_instruction_can_be_created_from_string() {
        let input = r#"move 2 from 5 to 9"#;

        let instruction = CraneInstruction::new(input.to_string());

        assert_eq!(instruction.amount, 2);
        assert_eq!(instruction.source, 5);
        assert_eq!(instruction.destination, 9);
    }

    #[test]
    fn storage_can_be_created_from_string() {
        let input = "
[C]         [S] [H]                
[F] [B]     [C] [S]     [W]        
[B] [W]     [W] [M] [S] [B]        
[L] [H] [G] [L] [P] [F] [Q]        
[D] [P] [J] [F] [T] [G] [M] [T]    
[P] [G] [B] [N] [L] [W] [P] [W] [R]
[Z] [V] [W] [J] [J] [C] [T] [S] [C]
[S] [N] [F] [G] [W] [B] [H] [F] [N]
 1   2   3   4   5   6   7   8   9 ";

        let storage = Storage::new(input.trim().to_string());

        assert_eq!(storage.crate_stacks.len(), 9);
        storage.print_stacks();
        assert_eq!(storage.crate_stacks.len(), 9);
    }

    #[test]
    fn crates_can_be_popped() {
        let input:Vec<char> = ['C', 'S', 'T'].to_vec();

        let mut crate_stack = CrateStack { crates: input };

        assert_eq!(crate_stack.peek(), 'T');
        let popped_crate = crate_stack.pop_crates(1);
        assert_eq!(*popped_crate.first().unwrap(), 'T');
        assert_eq!(crate_stack.peek(), 'S');
        crate_stack.push_crates(['K'].to_vec());
        assert_eq!(crate_stack.peek(), 'K');
    }
}