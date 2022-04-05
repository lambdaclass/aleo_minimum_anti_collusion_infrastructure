import poseidon.(Poseidon);

circuit MerkleTree {
    function generate_root(array: [field;8]) -> field{

        const amount_of_leaves: u32 = 8;
        const log2_amount_leaves: u32 = 3;
        //A field to use with mimc
        //Since we don't care about the encryption it can be any const
        const a_field: field = 123123123; 
        let hashed_array = array;

        for i in 0..amount_of_leaves{
            hashed_array[i] = Poseidon::poseidon_2_1(array[i], a_field);
        }

        //Computes a merkle root, that we use as a commitment
        let padding = [0field;25];
        //We need to 
        //add some padding until the if inside the for gets fixed
        //else, it gets an out of bounds error 
        let array_of_leaves = [...hashed_array, ...padding];

        for i in 0..log2_amount_leaves {
            console.log("Round {} of Hashes", i);
            //For max_options
            for j in 0..amount_of_leaves {
                let step = 2**(i);
                // This is the modulus
                let should_skip = j-(j/2)*2;
                if (((j+1)*step) < 8) && (should_skip == 0){
                    console.log("Hashing: {} {} in position: {}", j*step, (j+1)*step, j*step);
                    array_of_leaves[j*step] = 
                        Poseidon::poseidon_2_1(
                            array_of_leaves[j*step], 
                            array_of_leaves[(j+1)*step]
                        );
                }
            }
        }
        return array_of_leaves[0];
    }
}


// CODE FOR TESTING BELOW

circuit MerkleTreeForTest {
    function dummy_hash_plus_one(value:field) -> field{
        return value+1;
    }

    function dummy_pack_add(a_value:field, another_value:field) -> field{
        return a_value+another_value;
    }

    //This is always the same code as the merkle tree, 
    //but with a dummy hash to test
    function generate_dummy_root(array: [field;8]) -> field{

        const amount_of_leaves: u32 = 8;
        const log2_amount_leaves: u32 = 3;
        //A field to use with mimc
        //Since we don't care about the encryption it can be any const
        const a_field: field = 123123123; 
        let hashed_array = array;

        for i in 0..amount_of_leaves{
            hashed_array[i] = MerkleTreeForTest::dummy_hash_plus_one(array[i]);
        }

        //Computes a merkle root, that we use as a commitment
        let padding = [0field;25];
        //We need to 
        //add some padding until the if inside the for gets fixed
        //else, it gets an out of bounds error 
        let array_of_leaves = [...hashed_array, ...padding];

        for i in 0..log2_amount_leaves {
            console.log("Round {} of Hashes", i);
            //For max_options
            for j in 0..amount_of_leaves {
                let step = 2**(i);
                // This is the modulus
                let should_skip = j-(j/2)*2;
                if (((j+1)*step) < 8) && (should_skip == 0){
                    console.log("Hashing: {} {} in position: {}", j*step, (j+1)*step, j*step);
                    array_of_leaves[j*step] = 
                        MerkleTreeForTest::dummy_hash_plus_one(MerkleTreeForTest::dummy_pack_add(
                            array_of_leaves[j*step], 
                            array_of_leaves[(j+1)*step]));
                }
            }
        }
        return array_of_leaves[0];
    }
}
