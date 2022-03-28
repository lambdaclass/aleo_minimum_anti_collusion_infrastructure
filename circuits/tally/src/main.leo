// The 'tally' main function.

//We assume 32 votes max for testing purpuses
//Note we must return an u32 in Leo 1.5.3
function main(votes: [u32; 32]) -> [u32; 8] {

    let tally_result: u32 = 0;

    let vote_result: [u32;8] = [0,0,0,0,0,0,0,0];

    for i in 0..32 {
        //We assume 2 possible candidates for testing purposes
        //We still use an u32 since we can't define an u8 in Leo 1.5.3
        for j in 0..8{
            if (votes[i] == j){
                vote_result[j] = vote_result[j] + 1;
            }
        }
    }

    //TO DO: Hash vote_results
    return vote_result;
}
