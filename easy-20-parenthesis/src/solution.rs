pub fn closing_brace(s: char) -> char {
    match s {
        '[' => ']',
        '{' => '}',
        '(' => ')',
        _ => ' ',
    }
}

pub fn is_valid(s: &str) -> bool{

    let mut stack: Vec<char> = Vec::new();
    
    for symbol in s.chars().collect::<Vec<char>>() {
        if symbol == '[' || symbol == '{' || symbol == '(' {
            stack.push(symbol)
        }
        else if (symbol == ']' || symbol == '}' || symbol == ')') && !stack.is_empty() {
            if !symbol.eq(&closing_brace(stack.pop().unwrap())){
                return false;
            }
        }
        else { return false }
    }
    
    stack.len() == 0
}