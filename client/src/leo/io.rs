use std::fs::File;
use std::io::prelude::*;
use std::str;
const WHITELIST_IN_FILE_PATH: &str = "./circuits/whitelist/inputs/whitelist.in";

pub fn generate_input_file(leaf: &str, proof: &str, path_index: &str, root: &str) {
    /*

    This a template of the current input for a Leo program

    This functions builds this string as saves in a file in the Leo directory

    [main]
    leaf: field = 98172398712398172387123786;
    proof: [field; 5] = [1238917241,123891723,1237864368,892634234,871623123876];
    path_index: [bool; 5] = [true,true,false,false,true];
    root: field = 12398413491283712368743;

    [registers]
    r0: bool = false;

    */

    let input_as_string = String::from(&format!(
        "[main]\n\
        leaf: field = {};\n\
        proof: [field; 5] = {};\n\
        path_index: [bool; 5] = {};\n\
        root: field = {};\n\
        \n\
        [registers]\n\
        r0: bool = false;\n\
        ",
        leaf, proof, path_index, root
    ));

    let mut file = File::create(WHITELIST_IN_FILE_PATH).unwrap();
    file.write_all(input_as_string.as_bytes()).unwrap();
}
