use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { data: Vec::new() }
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn push(&mut self, item: T) {
        self.data.push(item)
    }

    fn empty(&self) -> bool {
        self.data.is_empty()
    }
}

pub fn first() {
    let contents =
        fs::read_to_string("./input/5/input.txt").expect("Should have been able to read the file");

    let rows = contents.split("\n");

    let mut crate_section = Vec::new();
    let mut index_section = 0;
    for row in rows {
        index_section += 1;
        if row == "" || row == " " {
            break;
        }
        crate_section.push(row);
    }
    let crate_index = crate_section
        .last()
        .unwrap()
        .split("")
        .filter(|x| x != &"")
        .collect::<Vec<&str>>();

    let mut crates = Vec::new();

    for i in 0..crate_index.len() {
        if crate_index[i] == "" || crate_index[i] == " " {
            continue;
        }

        let mut stack = Stack::new();
        let max_crate_len = crate_section.len() - 1;
        for j in 0..max_crate_len {
            let c = crate_section[max_crate_len - j - 1].chars().nth(i).unwrap();
            if c == ' ' {
                break;
            }
            stack.push(c);
        }

        crates.push(stack);
    }

    let collected_rows = contents.split("\n").collect::<Vec<&str>>();
    for i in index_section..collected_rows.len() {
        let current_row = collected_rows[i];
        if current_row == "" || current_row == " " {
            break;
        }
        let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        let cap = re.captures(current_row).unwrap();
        let qty = cap
            .get(1)
            .map_or("", |m| m.as_str())
            .parse::<i32>()
            .unwrap();
        let from = cap
            .get(2)
            .map_or("", |m| m.as_str())
            .parse::<i32>()
            .unwrap();
        let to = cap
            .get(3)
            .map_or("", |m| m.as_str())
            .parse::<i32>()
            .unwrap();

        for _ in 0..qty {
            let item = crates[(from - 1) as usize].pop().unwrap();
            crates[(to - 1) as usize].push(item);
        }
    }

    for i in 0..crates.len() {
        if !crates[i].empty() {
            let top_element = crates[i].pop();
            print!("{}", top_element.unwrap());
        } else {
            println!("Crate {}: empty", i + 1);
        }
    }
    println!("");
}

pub fn second() {
    let contents =
        fs::read_to_string("./input/5/input.txt").expect("Should have been able to read the file");

    let rows = contents.split("\n");

    let mut crate_section = Vec::new();
    let mut index_section = 0;
    for row in rows {
        index_section += 1;
        if row == "" || row == " " {
            break;
        }
        crate_section.push(row);
    }

    let crate_index = crate_section
        .last()
        .unwrap()
        .split("")
        .filter(|x| x != &"")
        .collect::<Vec<&str>>();

    let mut crates = Vec::new();

    for i in 0..crate_index.len() {
        if crate_index[i] == "" || crate_index[i] == " " {
            continue;
        }

        let mut stack = Stack::new();
        let max_crate_len = crate_section.len() - 1;
        for j in 0..max_crate_len {
            let c = crate_section[max_crate_len - j - 1].chars().nth(i).unwrap();
            if c == ' ' {
                break;
            }
            stack.push(c);
        }

        crates.push(stack);
    }

    let collected_rows = contents.split("\n").collect::<Vec<&str>>();
    for i in index_section..collected_rows.len() {
        let current_row = collected_rows[i];
        if current_row == "" || current_row == " " {
            break;
        }
        let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        let cap = re.captures(current_row).unwrap();
        let qty = cap
            .get(1)
            .map_or("", |m| m.as_str())
            .parse::<i32>()
            .unwrap();
        let from = cap
            .get(2)
            .map_or("", |m| m.as_str())
            .parse::<i32>()
            .unwrap();
        let to = cap
            .get(3)
            .map_or("", |m| m.as_str())
            .parse::<i32>()
            .unwrap();

        let mut temp = Stack::new();
        for _ in 0..qty {
            let item = crates[(from - 1) as usize].pop().unwrap();
            temp.push(item);
        }
        for _ in 0..qty {
            let item = temp.pop().unwrap();
            crates[(to - 1) as usize].push(item);
        }
    }

    for i in 0..crates.len() {
        if !crates[i].empty() {
            let top_element = crates[i].pop();
            print!("{}", top_element.unwrap());
        } else {
            println!("Crate {}: empty", i + 1);
        }
    }

    println!("");
}
