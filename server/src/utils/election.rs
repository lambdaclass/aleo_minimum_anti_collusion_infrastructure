pub const MAX_AMOUNT_OF_VOTES: usize = 32;

pub fn votes_to_fix_array(votes: &Vec<u32>) -> [u32; MAX_AMOUNT_OF_VOTES] {
    let mut votes_fixed: [u32; MAX_AMOUNT_OF_VOTES] = [0; MAX_AMOUNT_OF_VOTES as usize];
    for i in 0..MAX_AMOUNT_OF_VOTES {
        votes_fixed[i as usize] = match votes.get(i as usize) {
            Some(v) => v.clone(),
            None => 0,
        };
    }
    votes_fixed
}
