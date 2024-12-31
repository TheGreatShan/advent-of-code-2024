use std::fs;

pub(crate) fn run() {
    let file_content: Vec<Vec<char>> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut counter = 0;

    for line_index in 0..file_content.len() - 1 {
        let to_be_investigated = &file_content[line_index];

        for char_index in 0..to_be_investigated.len() - 1 {
            if to_be_investigated[char_index] == 'A' {
                if     line_index >= 1
                    && line_index <= to_be_investigated.len() - 1
                    && char_index >= 1
                    && char_index <= to_be_investigated.len() - 1
                {
                    let char_upper_left = file_content[line_index - 1][char_index - 1];
                    let char_upper_right = file_content[line_index - 1][char_index + 1];
                    let char_lower_right = file_content[line_index + 1][char_index + 1];
                    let char_lower_left = file_content[line_index + 1][char_index - 1];

                    if ((char_upper_left == 'S' && char_lower_right == 'M')
                        || (char_upper_left == 'M' && char_lower_right == 'S'))
                        && ((char_upper_right == 'S' && char_lower_left == 'M')
                        || (char_upper_right == 'M' && char_lower_left == 'S'))

                    {
                        counter += 1;
                        println!(
                            "Case: at ({}, {})", line_index, char_index
                        )
                    }
                }
            }
        }
    }

    println!("Total XMAS occurrences: {}", counter);
}
