use std::fs;

pub fn first() {
    let contents =
        fs::read_to_string("./input/1/input.txt").expect("Should have been able to read the file");

    let mut max = 0;

    let splitted = contents.split("\n\n").collect::<Vec<&str>>();
    for x in splitted.iter() {
        let mut sum = 0;
        for ns in x.split("\n") {
            if ns.len() > 0 {
                sum += ns.parse::<i32>().unwrap();
            }
        }

        if sum > max {
            max = sum;
        }
    }

    println!("{}", max);
}

pub fn second() {
    let contents =
        fs::read_to_string("./input/1/input.txt").expect("Should have been able to read the file");

    let mut max_1 = 0;
    let mut max_2 = 0;
    let mut max_3 = 0;

    let splitted = contents.split("\n\n").collect::<Vec<&str>>();
    for x in splitted.iter() {
        let mut sum = 0;
        for ns in x.split("\n") {
            if ns.len() > 0 {
                sum += ns.parse::<i32>().unwrap();
            }
        }

        if sum > max_3 {
            max_3 = sum;
        }

        if max_3 > max_2 {
            let tmp = max_2;
            max_2 = max_3;
            max_3 = tmp;
        }

        if max_2 > max_1 {
            let tmp = max_1;
            max_1 = max_2;
            max_2 = tmp;
        }
    }

    println!("{}", max_1 + max_2 + max_3);
}
