pub fn longest_common_prefix(strs: &Vec<String>) -> String {
        
    // Create vector that contains matching prefixes
    let mut prefix: Vec<char> = Vec::new();
    let first_word = strs[0].clone();
    
    // Create vectors of individual chars for easy comparison
    for (i, chr) in first_word.chars().enumerate() {
        if get_letters_at_index(i,&strs).iter().all(|&item| item == first_word.chars().nth(i).unwrap()) {
            prefix.push(chr);
        }
        else {
            return prefix.into_iter().collect();
        }
    }

    prefix.into_iter().collect()
}

/// Return a vector of chars from a given string at a specified index
pub fn get_letters_at_index(index: usize, strs: &Vec<String>) -> Vec<char> {

    let mut letters: Vec<char> = Vec::new();
    
    for st in strs {
        if st.chars().nth(index).is_some() {
            letters.push(st.chars().nth(index).unwrap());
            // since leetcode, will ignore possibilities of index out of range
            // irl would do error checking
        } else {
            letters.push(' ');
        }
    }

    letters
}