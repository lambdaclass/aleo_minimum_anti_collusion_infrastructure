// The 'tally' main function.
import hash.(Hash);
import merkle_tree.(MerkleTree);
import merkle_tree.(MerkleTreeForTest);

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


// TESTING CODE BELOW
// TO DO: Modules can't have tests, unless they are projects
//  -> Instantiate projects for each module to test
@test
function test_merkle_tree(){
        let array: [field;8] = [0,0,1,1,0,1,0,1];
        let root = MerkleTreeForTest::generate_dummy_root(array);
        //Hash function adds one so we get
        //1 1 2 2 1 2 1 2
        //Then we group them by adding them
        //2 4 3 3
        //Then we hash them adding one
        //3 5 4 4
        //Then we pack them by adding them
        //8 8
        //Then we hash them by adding one
        //9 9
        //Then we pack them
        //18
        //Then we hash them by adding one
        //19
        console.log("Root is: {}", root);
        console.assert(root == 19field);
}
