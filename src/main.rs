mod days;

use std::env;

fn main() {
    // get day number form comand line and exec the corrisponding day for all 25 days
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    let part = &args[2];

    match day.as_str() {
        "1" => {
            if part == "1" {
                days::day_1::first();
            } else {
                days::day_1::second();
            }
        }
        "2" => {
            if part == "1" {
                days::day_2::first();
            } else {
                days::day_2::second();
            }
        }
        "3" => {
            if part == "1" {
                days::day_3::first();
            } else {
                days::day_3::second();
            }
        }
        //"4" => {
        //if part == "1" {
        //days::day_4::first();
        //} else {
        //days::day_4::second();
        //}
        //}
        "5" => {
            if part == "1" {
                days::day_5::first();
            } else {
                days::day_5::second();
            }
        }
        "6" => {
            if part == "1" {
                days::day_6::first();
            } else {
                days::day_6::second();
            }
        }
        _ => println!("No day found"),
    }
}
