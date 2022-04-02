// The 'tally' main function.
import hash.(Hash);

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
    
    //We need some field to use mimc
    //Since we don't care about encryption it can be anything
    let a_field: field = 123123123; 

    let merkle_tree_leaves = vote_result;

    for i in 0..8{
        merkle_tree_leaves[i] = Hash::mimc(vote_result[i], a_field);
    }

    //Computes a merkle root, that we use as a commitment
    let padding = [0field;25];
    //We need to 
    //add some padding until the if inside the for gets fixed
    //else, it gets an out of bounds error 
    let merkle_tree_layer = [...merkle_tree_leaves, ...padding];

    //for 0..SQRT(max_options), 8 -> 3
    for i in 0..3{
        console.log("Round {} of Hashes", i);
        //For max_options
        for j in 0..8{
            let step = 2**(i);
            // This is the modulus
            let should_skip = j-(j/2)*2;
            if (((j+1)*step) < 8) && (should_skip == 0){
                console.log("Hashing: {} {} in position: {}", j*step, (j+1)*step, j*step);
                merkle_tree_layer[j*step] = 
                    Hash::mimc(Hash::pack(
                        merkle_tree_layer[j*step], 
                        merkle_tree_layer[(j+1)*step]),
                    a_field);
            }
        }
    }

    return merkle_tree_layer[0];
}
