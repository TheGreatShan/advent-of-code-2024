use std::fs;

fn main(){
    let file_content : Vec<String>  = fs::read_to_string("input.txt")
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect();  // gather them together into a vector

    let length = file_content.len();

    for n in 0..length {
        let listOfInputInt : Vec<u32> = Vec::new(); 
        let content = file_content[n].split(" ");
        
        for _ in 0..content.len(){
            
        }
    }

    println!("{}", file_content);
}