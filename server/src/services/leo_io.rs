use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::str;
const MAX_VOTES: usize = 32;
const TALLY_IN_FILE_PATH: &str = "./../circuits/tally/inputs/tally.in";
const TALLY_OUT_FILE_PATH: &str = "./../circuits/tally/outputs/tally.out";

pub fn generate_input_file(votes: [u32; MAX_VOTES], vote_merkle_root: &str) {
    /*

    This a template of the current input for a Leo program

    This functions builds this string as saves in a file in the Leo directory

    [main]
    votes_as_u32: [u32; 32] = [
            1, 2, 3, 2, 2, 3, 1, 2,
            1, 2, 3, 2, 2, 3, 1, 2,
            1, 2, 3, 2, 2, 3, 1, 2,
            1, 2, 3, 2, 2, 3, 1, 2
        ];

    votes_as_field: [field; 32] = [
            1, 2, 3, 2, 2, 3, 1, 2,
            1, 2, 3, 2, 2, 3, 1, 2,
            1, 2, 3, 2, 2, 3, 1, 2,
            1, 2, 3, 2, 2, 3, 1, 2
        ];

    votes_merkle_root: field = 6081127065217055003429398673533374549058098389318475736416753929574343365699;

    [registers]
    votes_merkle_root: field = 0;
    */

    let votes_strs = format!("{:?}", votes);
    let mut input_as_string: String = String::from("[main]\nvotes_as_u32: [u32; 32] = ");
    input_as_string.push_str(votes_strs.as_str());
    input_as_string.push_str(";\n");
    input_as_string.push_str("votes_as_field: [field; 32] = ");
    input_as_string.push_str(votes_strs.as_str());
    input_as_string.push_str(";\n");
    input_as_string.push_str("votes_merkle_root: field = ");
    input_as_string.push_str(vote_merkle_root);
    input_as_string.push_str(";\n\n");
    input_as_string.push_str("[registers]\nvotes_merkle_root: field = 0;\n");

    let mut file = File::create(TALLY_IN_FILE_PATH).unwrap();

    file.write_all(input_as_string.as_bytes()).unwrap();
}

pub fn read_output_file() {
    let contents = fs::read_to_string(TALLY_OUT_FILE_PATH).unwrap();
    println!("Contents: {}", contents);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_file() {
        read_output_file();
    }
}
