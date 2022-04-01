// The 'tally' main function.
import hash.(Hash);

//We assume 32 votes max for testing purpuses
//Note we must return an u32 in Leo 1.5.3
function main(votes: [u32; 32]) -> field {

    let tally_result: u32 = 0;

    let vote_result: [field;8] = [0,0,0,0,0,0,0,0];

    for i in 0..32 {
        //We assume 2 possible candidates for testing purposes
        //We still use an u32 since we can't define an u8 in Leo 1.5.3
        for j in 0..8{
            if (votes[i] == j){
                vote_result[j] = vote_result[j] + 1;
            }
        }
    }
    
    let a_field: field = 123123123; 

    let merkle_tree_layer = vote_result;

    for i in 0..8{
        merkle_tree_layer[i] = Hash::mimc(vote_result[i], a_field);
    }

    //for SQRT(max_options), 8 -> 2
    //Computes a merkle root, that we use as a commitment
    for i in 0..3{
        //For max_options / 2
        for j in 0..4{
            // Depending on the size of the options stopping the loop
            // Costs more constraints than just keeping hashing
            // if (j < (4/(2**i))){
            merkle_tree_layer[j] = Hash::mimc(Hash::pack(merkle_tree_layer[j*2], merkle_tree_layer[(j*2)+1]), a_field);
        }
    }

    return merkle_tree_layer[0];
}
