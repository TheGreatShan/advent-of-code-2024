use std::fs;

fn main(){
    let file_content : Vec<String>  = fs::read_to_string("input.txt")
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect();  // gather them together into a vector

    let length = file_content.len();

    let mut safe = 0;

    for n in 0..length {
        let content : Vec<i32> = file_content[n]
            .split(" ")
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect();
        let mut operation = "";

        for x in 1..content.len(){
            let set_one = content[x];
            let set_two = content[x-1];
            let difference = set_one - set_two;

            if (operation == "" || operation == "-") && (difference == -1 || difference == -2 || difference == -3){
                operation = "-";
                continue;
            }
            else if (operation == "" || operation == "+") && (difference == 1 || difference == 2 || difference == 3){
                operation = "+";
                continue;
            } else {
                operation = "not safe";
                continue;
            }
        }
        if operation != "not safe" {
            safe += 1;
        }
    }

    println!("{}", safe);
}