use std::fs;

fn normalize(input: &str) -> &str {
    match input {
        "A" => "R",
        "B" => "P",
        "C" => "S",
        "X" => "R",
        "Y" => "P",
        "Z" => "S",
        _ => input,
    }
}

fn compare(m1: &str, m2: &str) -> i32 {
    if m1 == m2 {
        3
    } else if (m1 == "R" && m2 == "S") || (m1 == "P" && m2 == "R") || (m1 == "S" && m2 == "P") {
        6
    } else {
        0
    }
}

fn compute(pl1: &str, pl2: &str) -> i32 {
    // A and X rock
    // B and Y paper
    // C and Z scissors

    let mut score = match pl1 {
        "R" => 1,
        "P" => 2,
        "S" => 3,
        _ => 0,
    };

    score += compare(pl1, pl2);

    return score;
}

fn decrypt_move<'a>(input: &str, strategy: &str) -> &'a str {
    if input.eq("A") {
        return match strategy {
            "X" => "S",
            "Y" => "R",
            "Z" => "P",
            _ => panic!("Invalid strategy"),
        };
    }

    if input.eq("B") {
        return match strategy {
            "X" => "R",
            "Y" => "P",
            "Z" => "S",
            _ => panic!("Invalid strategy"),
        };
    };

    if input.eq("C") {
        return match strategy {
            "X" => "P",
            "Y" => "S",
            "Z" => "R",
            _ => panic!("Invalid strategy"),
        };
    }

    panic!("Invalid input da");
}

fn figure_out_winner(pl2: &str, strategy: &str) -> i32 {
    let you = decrypt_move(pl2, strategy);
    let opponent = normalize(pl2);

    compute(you, opponent)
}

pub fn first() {
    let contents =
        fs::read_to_string("./input/2/input.txt").expect("Should have been able to read the file");

    let mut score = 0;
    let matches = contents.split("\n");
    for m in matches {
        if m.len() > 0 {
            let mut players = m.split(" ");
            let opponent = players.next().unwrap();
            let you = players.next().unwrap();
            score += compute(normalize(opponent), normalize(you));
        }
    }

    println!("{}", score);
}

pub fn second() {
    let contents =
        fs::read_to_string("./input/2/input.txt").expect("Should have been able to read the file");

    let mut score = 0;
    let matches = contents.split("\n");
    for m in matches {
        if m.len() > 0 {
            let mut players = m.split(" ");
            let opponent = players.next().unwrap();
            let strategy = players.next().unwrap();
            score += figure_out_winner(opponent, strategy);
        }
    }

    println!("{}", score);
}
