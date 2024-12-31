use std::fs;

fn main() {
    let file_content: Vec<Vec<char>> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut counter = 0;

    for line_index in 0..file_content.len() {
        let to_be_investigated = &file_content[line_index];

        for char_index in 0..to_be_investigated.len() {
            if to_be_investigated[char_index] == 'X' {
                // CASE 1: Go up
                if line_index >= 3 {
                    if file_content[line_index - 1][char_index] == 'M'
                        && file_content[line_index - 2][char_index] == 'A'
                        && file_content[line_index - 3][char_index] == 'S'
                    {
                        counter += 1;
                        println!("Found XMAS going up at ({}, {})", line_index, char_index);
                    }
                }

                // CASE 2: Go down
                if line_index + 3 < file_content.len() {
                    if file_content[line_index + 1][char_index] == 'M'
                        && file_content[line_index + 2][char_index] == 'A'
                        && file_content[line_index + 3][char_index] == 'S'
                    {
                        counter += 1;
                        println!("Found XMAS going down at ({}, {})", line_index, char_index);
                    }
                }

                // CASE 3: Go right
                if char_index + 3 < to_be_investigated.len() {
                    if to_be_investigated[char_index + 1] == 'M'
                        && to_be_investigated[char_index + 2] == 'A'
                        && to_be_investigated[char_index + 3] == 'S'
                    {
                        counter += 1;
                        println!("Found XMAS going right at ({}, {})", line_index, char_index);
                    }
                }

                // CASE 4: Go left
                if char_index >= 3 {
                    if to_be_investigated[char_index - 1] == 'M'
                        && to_be_investigated[char_index - 2] == 'A'
                        && to_be_investigated[char_index - 3] == 'S'
                    {
                        counter += 1;
                        println!("Found XMAS going left at ({}, {})", line_index, char_index);
                    }
                }

                // CASE 5: Diagonal down-right
                if line_index + 3 < file_content.len() && char_index + 3 < to_be_investigated.len() {
                    if file_content[line_index + 1][char_index + 1] == 'M'
                        && file_content[line_index + 2][char_index + 2] == 'A'
                        && file_content[line_index + 3][char_index + 3] == 'S'
                    {
                        counter += 1;
                        println!("Found XMAS diagonal down-right at ({}, {})", line_index, char_index);
                    }
                }

                // CASE 6: Diagonal down-left
                if line_index + 3 < file_content.len() && char_index >= 3 {
                    if file_content[line_index + 1][char_index - 1] == 'M'
                        && file_content[line_index + 2][char_index - 2] == 'A'
                        && file_content[line_index + 3][char_index - 3] == 'S'
                    {
                        counter += 1;
                        println!("Found XMAS diagonal down-left at ({}, {})", line_index, char_index);
                    }
                }

                // CASE 7: Diagonal up-right
                if line_index >= 3 && char_index + 3 < to_be_investigated.len() {
                    if file_content[line_index - 1][char_index + 1] == 'M'
                        && file_content[line_index - 2][char_index + 2] == 'A'
                        && file_content[line_index - 3][char_index + 3] == 'S'
                    {
                        counter += 1;
                        println!("Found XMAS diagonal up-right at ({}, {})", line_index, char_index);
                    }
                }

                // CASE 8: Diagonal up-left
                if line_index >= 3 && char_index >= 3 {
                    if file_content[line_index - 1][char_index - 1] == 'M'
                        && file_content[line_index - 2][char_index - 2] == 'A'
                        && file_content[line_index - 3][char_index - 3] == 'S'
                    {
                        counter += 1;
                        println!("Found XMAS diagonal up-left at ({}, {})", line_index, char_index);
                    }
                }
            }
        }
    }

    println!("Total XMAS occurrences: {}", counter);
}