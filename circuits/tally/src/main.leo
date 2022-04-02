// The 'tally' main function.
import hash.(Hash);
import merkle_tree.(MerkleTree);

function main(votes: [u32; 32]) -> field {

    let tally_result: u32 = 0;

    let vote_result: [field;8] = [0,0,0,0,0,0,0,0];

    //For each vote
    for i in 0..32 {
        //For each category
        //Add the vote to the field with the results
        for j in 0..8{
            if (votes[i] == j){
                vote_result[j] = vote_result[j] + 1;
            }
        }
    }
    
    return MerkleTree::generate_root(vote_result);
}
