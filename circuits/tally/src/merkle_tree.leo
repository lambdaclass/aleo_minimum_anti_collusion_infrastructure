import hash.(Hash);

circuit MerkleTree {
    function generate_root(array: [field;8]) -> field{

        const amount_of_leaves: u32 = 8;
        const log2_amount_leaves: u32 = 3;
        //A field to use with mimc
        //Since we don't care about the encryption it can be any const
        const a_field: field = 123123123; 
        let hashed_array = array;

        for i in 0..amount_of_leaves{
            hashed_array[i] = Hash::mimc(array[i], a_field);
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
                        Hash::mimc(Hash::pack(
                            array_of_leaves[j*step], 
                            array_of_leaves[(j+1)*step]),
                        a_field);
                }
            }
        }
        return array_of_leaves[0];
    }
}
