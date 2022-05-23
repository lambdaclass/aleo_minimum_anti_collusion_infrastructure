import merkle_tree.(MerkleTree);

// The 'sign-up' main function.
/// Takes a leaf, a merkle proof (of inclusion)
// and a merkle root, and validates it's included in the set (whitelist)
/// Note the merkle root should be one that matches a record of the voting system
/// app in the future

//Note the proof size should be height - 1
function main(leaf: field , proof: [field;5], path_index: [bool;5], root: field) -> field {
    //Note we can't use comparator < > with fields
    //To know if the proof is hashed as the right or left element
    //We use path index, which can be 1 or 0
    //if
    //MerkleTree::validate_proof(leaf,path_index,root,u32)
    //ret 0
    //else 1
    return 0field;
}
