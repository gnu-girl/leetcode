pub fn is_palindrome(mut x: i32) -> bool {
    let mut digits: Vec<i32> = Vec::new();
    x = 1001;
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
                i-=1;
                j+=1;
            }
        }

        return true;
    }
    else {
        return false;
    }

    
}
