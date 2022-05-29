use std::io::prelude::*;
use std::str;
use std::{fs::File, path};
const MAX_WHITELIST_SIZE: usize = 32;

const WHITELIST_IN_FILE_PATH: &str = "./circuits/whitelist/inputs/whitelist.in";
use num::{BigUint, Num};

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

///Converts a string with Fr format to a decimal string for Leo
///Ex: Fr(0x0d71cbc322578e133085b861a656d34b3abc2cc65ac11d24618aa53d49e5d443) ->
/// 6081127065217055003429398673533374549058098389318475736416753929574343365699

pub fn fr_string_to_leo_str(fr: String) -> String {
    let sliced_string = fr[5..(fr.len() - 1)].to_string();
    let sliced_str: &str = sliced_string.as_str();
    BigUint::from_str_radix(sliced_str, 16).unwrap().to_string()
}

mod tests {

    use super::*;
    #[test]
    fn test_fr_leo() {
        let fr_str =
            "Fr(0x0d71cbc322578e133085b861a656d34b3abc2cc65ac11d24618aa53d49e5d443)".to_string();
        let str = super::fr_string_to_leo_str(fr_str);
        assert_eq!(
            str,
            "6081127065217055003429398673533374549058098389318475736416753929574343365699"
                .to_string()
        );
    }

    #[test]
    fn test_write_file() {
        let leaf = "32";
        let proof = "[1,2,3,4,5]";
        let path_index = "[true,true,false,false,true]";
        let root = "123897143657234";
        generate_input_file(leaf, proof, path_index, root);
    }
}
