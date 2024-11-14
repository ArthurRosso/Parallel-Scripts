use std::fs::File;
use std::io::{self, BufRead};

use std::time::Instant;

use diam::prelude::*;
use rayon::prelude::*;

// pub fn detect_capital_use(word: String) -> bool {
//     let is_fcapital: bool = word.chars().nth(0).unwrap().is_uppercase();
//     let is_scapital: bool = word.chars().nth(1).unwrap_or('a').is_uppercase();

//     if !is_fcapital && is_scapital {
//         return false;
//     }

//     word.chars().skip(2).fold(true, |acc, x| {
//         if x.is_uppercase() != is_scapital {
//             false
//         } else {
//             acc
//         }
//     })
// }

// pub fn detect_capital_use_par(word: String) -> bool {
//     let is_fcapital: bool = word.chars().nth(0).unwrap().is_uppercase();
//     let is_scapital: bool = word.chars().nth(1).unwrap_or('a').is_uppercase();

//     if !is_fcapital && is_scapital {
//         return false;
//     }

//     let chars: Vec<char> = word.chars().collect();

//     chars
//         .par_iter()
//         .skip(2)
//         .fold(
//             || true,
//             |acc, x| {
//                 if x.is_uppercase() != is_scapital {
//                     false
//                 } else {
//                     acc
//                 }
//             },
//         )
//         .reduce(
//             || true,
//             |char1: bool, char2: bool| {
//                 if char1 && char2 {
//                     true
//                 } else {
//                     false
//                 }
//             },
//         )
// }

fn print_svg() {
    let word = String::from("Datadatadatadatadatadatadatadatadatadatadatadatadatadatadatadatadatadatadatadatadatadatadatadata");

    let is_fcapital: bool = word.chars().nth(0).unwrap().is_uppercase();
    let is_scapital: bool = word.chars().nth(1).unwrap_or('a').is_uppercase();

    if !is_fcapital && is_scapital {
        println!("false");
    } else {
        let chars: Vec<char> = word.chars().collect();

        svg("fold.svg", || {
            let s = chars
                .into_par_iter()
                .skip(2)
                .fold(
                    || true,
                    |acc, x| {
                        if x.is_uppercase() != is_scapital {
                            false
                        } else {
                            acc
                        }
                    },
                )
                .log("fold")
                .reduce(|| true, |char1: bool, char2: bool| char1 && char2);
            assert_eq!(s, true);
        })
        .expect("failed saving log");
    }
}

fn main() -> io::Result<()> {
    // let word = String::from("mL");
    // println!("{:?}", detect_capital_use_par(word));

    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let input_s = line?.clone();

        let input_p = input_s.clone();
        println!("For the input: {}", input_s.clone());

        let before = Instant::now();
        let result = detect_capital_use(input_s);
        println!("Elapsed sequential time: {:.2?}", before.elapsed());

        let before_par = Instant::now();
        let result_paralell = detect_capital_use_par(input_p);
        println!("Elapsed parallel time: {:.2?}", before_par.elapsed());

        assert_eq!(result, result_paralell);
    }

    print_svg();

    Ok(())
}
