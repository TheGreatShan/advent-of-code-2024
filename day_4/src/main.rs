use std::fs;

fn main() {
    let file_content : Vec<String>  = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut counter = 0;

    for lineItter in  0.. file_content.len() - 1{
        let to_be_investigated : Vec<String> = file_content[lineItter]
            .split("")
            .map(|x| x.to_string())
            .collect();

        for charItter in 0.. to_be_investigated.len() - 1 {
            if to_be_investigated[charItter] == "X" {
                // CASE 1: Go up
                if lineItter >= 3 {
                    let oneCharAbove : Vec<String> = file_content[lineItter - 1]
                        .split("")
                        .map(|x| x.to_string())
                        .collect();

                    if oneCharAbove[charItter] == "M" {
                        let twoCharsAbove  : Vec<String> = file_content[lineItter - 2]
                            .split("")
                            .map(|x| x.to_string())
                            .collect();

                        if twoCharsAbove[charItter] == "A" {
                            let threeCharsAbove  : Vec<String> = file_content[lineItter - 3]
                                .split("")
                                .map(|x| x.to_string())
                                .collect();

                            if threeCharsAbove[charItter] == "S" {
                                counter += 1;
                                println!("Crazy. XMAS is against above")
                            }
                        }
                    }
                }
                // CASE 2: Go down
                if lineItter <= file_content.len() - 5 {
                    let oneCharDown: Vec<String> = file_content[lineItter + 1]
                        .split("")
                        .map(|x| x.to_string())
                        .collect();

                    if oneCharDown[charItter] == "M" {
                        let twoCharsDown: Vec<String> = file_content[lineItter + 2]
                            .split("")
                            .map(|x| x.to_string())
                            .collect();

                        if twoCharsDown[charItter] == "A" {
                            let threeCharsDown: Vec<String> = file_content[lineItter + 3]
                                .split("")
                                .map(|x| x.to_string())
                                .collect();

                            if threeCharsDown[charItter] == "S" {
                                counter += 1;
                                println!("Crazy. XMAS is against down")
                            }
                        }
                    }
                }
                // CASE 3: Go right

                // CASE 4: Go left

                // Case 5: Go diagonal right

                // CASE 6: Go diagonal left
            }
        }
    }
    println!("{}", counter)
}
