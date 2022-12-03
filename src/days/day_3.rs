use std::fs;

fn compute_priority(c: char) -> i32 {
    let val = c as i32;

    if val > 95 {
        return val - 96;
    } else {
        return val - 64 + 26;
    }
}

fn remove_duplicates(chars: &Vec<char>, from: usize, to: usize) -> Vec<char> {
    let mut res = Vec::new();
    for i in from..to {
        if !res.iter().any(|s: &char| s == &chars[i]) {
            res.push(chars[i]);
        }
    }

    res
}

pub fn first() {
    let contents =
        fs::read_to_string("./input/3/input.txt").expect("Should have been able to read the file");

    let mut sum = 0;
    let sacks = contents.split("\n");
    for sack in sacks {
        if sack.len() > 0 {
            let size = sack.len() / 2;
            let chars = sack.chars().collect::<Vec<char>>();

            let sack_1 = remove_duplicates(&chars, 0, size as usize);
            let sack_2 = remove_duplicates(&chars, size, chars.len() as usize);

            for i in 0..sack_1.len() {
                for j in 0..sack_2.len() {
                    if sack_1[i] == sack_2[j] {
                        sum += compute_priority(sack_1[i]);
                    }
                }
            }
        }
    }

    println!("{}", sum);
}

pub fn second() {
    let contents =
        fs::read_to_string("./input/3/input.txt").expect("Should have been able to read the file");
    let sacks = contents.split("\n").collect::<Vec<&str>>();
    let mut i = 0;
    let mut sum = 0;
    while i + 3 < sacks.len() {
        let chars_1 = sacks[i].chars().collect::<Vec<char>>();
        let chars_2 = sacks[i + 1].chars().collect::<Vec<char>>();
        let chars_3 = sacks[i + 2].chars().collect::<Vec<char>>();

        let sack_1 = remove_duplicates(&chars_1, 0, chars_1.len());
        let sack_2 = remove_duplicates(&chars_2, 0, chars_2.len());
        let sack_3 = remove_duplicates(&chars_3, 0, chars_3.len());

        for x in 0..sack_1.len() {
            for y in 0..sack_2.len() {
                for z in 0..sack_3.len() {
                    if sack_1[x] == sack_2[y] && sack_2[y] == sack_3[z] {
                        sum += compute_priority(sack_1[x]);
                    }
                }
            }
        }
        i += 3;
    }
    println!("{}", sum);
}
