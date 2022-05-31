pub const MAX_AMOUNT_OF_VOTES: usize = 32;

// Transform votes to a fixed array of MAX_AMOUNT_OF_VOTES elements
// if there is and invalid vote option, it will be count as 0
// if there is less than MAX_AMOUNT_OF_VOTES, the reamaning votes will be count as 0
pub fn votes_to_fix_array(votes: &[u32]) -> [u32; MAX_AMOUNT_OF_VOTES] {
    let mut votes_fixed: [u32; MAX_AMOUNT_OF_VOTES] = [0; MAX_AMOUNT_OF_VOTES as usize];
    for i in 0..MAX_AMOUNT_OF_VOTES {
        votes_fixed[i as usize] = match votes.get(i as usize) {
            Some(v) => *v,
            None => 0,
        };
    }
    votes_fixed
}
