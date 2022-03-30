// The 'tally' main function.
import hash.(Hash);

//Note: We don't use szudziks  or other 
//since manually comparing fields is costly, and the prime field is big enough
//to simply map every value as a*b
function pack(a: field, b: field) -> field{
    return a*b;
}

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

    //TO DO: Generalize (To do so, implement the mod function)
    let first_row: [field; 4] = [0,0,0,0];
    let second_row: [field; 2] = [0,0];
    let root: field = 0;
    
    let a_field: field = 123123123; 
    first_row[0] = Hash::mimc(pack(vote_result[0], vote_result[1]), a_field);
    first_row[1] = Hash::mimc(vote_result[2], vote_result[3]);
    first_row[2] = Hash::mimc(vote_result[4], vote_result[5]);
    first_row[3] = Hash::mimc(vote_result[6], vote_result[7]);

    second_row[0] = pack(first_row[0], first_row[1]);
    second_row[1] = pack(first_row[1], first_row[2]);

    root = pack(second_row[0], second_row[1]);

    return root;
}
