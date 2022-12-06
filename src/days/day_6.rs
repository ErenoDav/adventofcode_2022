use std::fs;

pub fn its_all_different(chars: &Vec<char>) -> bool {
    for i in 0..chars.len() {
        for j in (i + 1)..chars.len() {
            if chars[i] == chars[j] {
                return false;
            }
        }
    }
    true
}

pub fn check(chars: &Vec<char>, counts: usize) {
    for i in 0..(chars.len() - counts) {
        let mut cur_shop = Vec::new();
        for j in i..(i + counts) {
            cur_shop.push(chars[j]);
        }

        if its_all_different(&cur_shop) {
            println!("{}", i + counts);
            break;
        }
    }
}

pub fn first() {
    let contents =
        fs::read_to_string("./input/6/input.txt").expect("Should have been able to read the file");

    let chars = contents.chars().collect::<Vec<char>>();
    check(&chars, 4);
    println!("");
}

pub fn second() {
    let contents =
        fs::read_to_string("./input/6/input.txt").expect("Should have been able to read the file");

    let chars = contents.chars().collect::<Vec<char>>();

    check(&chars, 14);

    println!("");
}
