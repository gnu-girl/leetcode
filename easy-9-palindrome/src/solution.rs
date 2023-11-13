pub fn is_palindrome_no_math(x: i32) -> bool {
    // Do this the string-y way
    let digits = x.to_string().chars().collect::<String>();
    let digits_backwards = digits.chars().rev().collect::<String>();
    
    for (i,_el) in digits.chars().enumerate() {
        if !digits.as_bytes()[i].eq(&digits_backwards.as_bytes()[i]) {
            return false;
        }
    }
    true
}

pub fn is_palindrome(mut x: i32) -> bool {
    // Do this the math-y way (break up into digits)
    let mut digits = Vec::new();

    if x >= 0 {
        // negative numbers are never a palindrome
        while x >= 1 {
            digits.push(x%10);
            x = x / 10;
        }
        if digits.len() != 0 {
            let mut i:usize = digits.len()- 1;
            let mut j:usize = 0; 
        
            while j < i {
                if digits[j] != digits[i] {
                    return false;
                }
                i += 1;
                j += 1;
            }
        }

        return true;
    }
    else {
        return false;
    }

    
}