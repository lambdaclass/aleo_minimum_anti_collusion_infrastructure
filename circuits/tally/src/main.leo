// The 'tally' main function.
import poseidon.(Poseidon);
import merkle_tree.(MerkleTree);

function main(votes_as_u32: [u32; 32], votes_as_field: [field;32], votes_merkle_root: field) -> field {

    /*
    We need votes as fields to check the merkle root, and as u32 since
    to group all votes of the same category together

    Input registers don't work
    We use a the merkle tree as a private input but it should be 
    something public
    As an alternative,
    we can publish the input merkle tree and the program commitment since we have
    two registers as output.
    
    This should be updated in Leo 2.0
    */

    let tally_result: u32 = 0;
    let vote_result: [field;8] = [0,0,0,0,0,0,0,0];
    let result_root = 0field;

    console.log("Verifying merkle root");
    let calculated_root = MerkleTree::generate_root32(votes_as_field);
    console.log("Given root: {}", votes_merkle_root);
    console.log("Calculated merkle root: {}", calculated_root);

    //This should be enforced
    //Until we can do such, we return 0 on the commitment to mark it
    //as invalid
    if (calculated_root == votes_merkle_root) {
        console.log("Tallying");        
        //For each vote
        for i in 0..32 {
            //For each category
            //Add the vote to the field with the results
            for j in 1..9{
                //Votes for option 0 are void
                //We place votes for option 1 in position 0 of the array and such
                if (votes_as_u32[i] == j){
                    vote_result[j-1] = vote_result[j-1] + 1;
                }
            }
        }
        result_root = MerkleTree::generate_root8(vote_result);
    }

    console.log("Vote result hash: {}", result_root);
    return result_root;
}
