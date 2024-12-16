use std::fs;
use std::collections::HashMap;

fn main(){
    let file_content : Vec<String>  = fs::read_to_string("input.txt")
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect();  // gather them together into a vector

    let length = file_content.len();

    let mut safe : u32 = 0;
    let mut breaker : i32 = 0;

    for n in 0..length {
        let mut content : Vec<i32> = file_content[n]
            .split(" ")
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect();
        let mut difference_map: HashMap<i32, &str> = HashMap::new();
        let mut count_not_safe = 0;

        iterate_through_list(&mut content, &mut difference_map, &mut safe, &mut count_not_safe, &mut breaker);
        breaker = 0;
    }

    println!("{}", safe);
}

fn iterate_through_list(content: &mut Vec<i32>, difference_map: &mut HashMap<i32, &str>, safe: &mut u32,  count_not_safe: &mut i32, breaker: &mut i32) {
    let mut operation = "";

    for x in 1..content.len(){
        let set_one = content[x];
        let set_two = content[x-1];
        let difference = set_one - set_two;
        

        if (operation == "" || operation == "-") && (difference == -1 || difference == -2 || difference == -3){
            operation = "-";
        }
        else if (operation == "" || operation == "+") && (difference == 1 || difference == 2 || difference == 3){
            operation = "+";
        } 
        else {
            operation = "not safe";
        }
        difference_map.insert(x as i32, operation);
    }

    for (_, &value) in difference_map.iter() {
        if value == "not safe" {
            println!("{}", value);
            *count_not_safe += 1;
        }
    }

    if *count_not_safe == 0 {
        *safe += 1;
        *count_not_safe = 0;
    } else if *count_not_safe == 1 && *breaker == 0 {
        println!("safe: {}", safe);
        println!("could not safe map: {}", count_not_safe);

        if let Some(key) = difference_map.iter().find(|&(_, &v)| v == "not safe").map(|(k, _)| *k) {
            content.remove(key as usize);
            difference_map.clear();
            *count_not_safe = 0;
            *breaker += 1;
            iterate_through_list(content, difference_map, safe, count_not_safe, breaker);
        } else {
            *count_not_safe +=1;
            println!("No 'not safe' entry found, exiting recursion.");
        }
    }
}