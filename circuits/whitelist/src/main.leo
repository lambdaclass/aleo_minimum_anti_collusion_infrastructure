import merkle_tree.(MerkleTree);

// The 'whitelist' main function.
/// Takes a leaf, a merkle proof (of inclusion)
// and a merkle root, and validates it's included in the set (whitelist)
/// Notes for future versions:
/// 1. The merkle root should be one that matches a record on the blockchain state of the voting system
/// 2. The returned value to be submitted on the blockchain must be the vote
/// 3. The circuit should not generate a proof if the inclusion proof is invalid.

//Note the proof size should be height - 1
function main(leaf: field , proof: [field;5], path_index: [bool;5], root: field) -> bool {
    return MerkleTree::verify_proof32(leaf,proof, path_index, root);
}
