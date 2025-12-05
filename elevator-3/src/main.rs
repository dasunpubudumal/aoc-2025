use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

struct Stack {
    elems: Vec<u64>,
}

impl Stack {
    fn new() -> Self {
        Stack {
            elems: Vec::<u64>::new(),
        }
    }

    fn pop(&mut self) -> Option<u64> {
        self.elems.pop()
    }

    fn append(&mut self, item: u64) {
        self.elems.push(item);
    }

    fn peek(&self) -> Option<u64> {
        match self.elems.last() {
            Some(val) => Some(*val),
            None => None,
        }
    }

    fn is_empty(&self) -> bool {
        match self.elems.len() {
            0 => true,
            _ => false,
        }
    }

    fn str(&self) -> String {
        let mut res = String::new();
        for i in (0..self.elems.len()) {
            res.push_str(&self.elems.get(i).unwrap().to_string());
        }
        res
    }

    fn size(&self) -> usize {
        self.elems.len()
    }
}

fn main() {
    let lines = read_lines("input.txt");
    //
    // let lines = [
    //     "987654321111111",
    //     "811111111111119",
    //     "234234234234278",
    //     "818181911112111",
    // ];

    let mut sum = 0;

    for input in lines {
        let mut stack: Stack = Stack::new();
        let mut to_remove: usize = input.len() - 12;
        for char in input.chars() {
            let current: u64 = char.to_digit(10).unwrap().into();
            while to_remove > 0 && !stack.is_empty() && current > stack.peek().unwrap() {
                stack.pop();
                to_remove -= 1;
            }
            stack.append(current);
        }

        while to_remove > 0 {
            println!("HI");
            stack.pop();
            to_remove -= 1;
        }

        let result = stack.str().parse::<u64>().unwrap();

        sum += result;
    }

    println!("Sum = {}", sum);
}
