use regex::Regex;
use std::fs;

fn main(){
    let file_content : String = fs::read_to_string("input.txt").unwrap();

    let re = Regex::new(r"mul\(([1-9]?[0-9]|[1-9][0-9]{2}),([1-9]?[0-9]|[1-9][0-9]{2})\)").unwrap();
    let mut total = 0;
    for mul in re.captures_iter(&file_content) {
        let selected = mul.get(0).unwrap().as_str();
        println!("{}", selected);
        let new_selected: Vec<String> = selected
            .split("mul(")
            .skip(1)
            .map(|x| x.trim_end_matches(')').to_string())
            .collect();

        if let Some(second) = new_selected.get(0){
            println!("{}", second);
            let multiply : Vec<i32> = second
                .split(",")
                .map(|x| x.trim().parse::<i32>().unwrap())
                .collect();

            if let Some(multiply1) = multiply.get(0){
                if let Some(multiple2) = multiply.get(1){
                    total = total + multiply1 * multiple2;
                }
            }
        }
    }

    println!("{}", total);
}