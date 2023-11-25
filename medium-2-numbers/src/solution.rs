use core::fmt;

/// Define our list as singly-linked
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
  pub fn add(self, val: i32) -> ListNode{
    let new_node = ListNode {
        val: val,
        next: Some(Box::new(self))
    };
    new_node
  }
}

// Quality of life function to faciliate printing out our list for debugging
impl fmt::Display for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        let mut l1 = &*self;

        while l1.next != None {
            write!(f, "[{}]-> ", l1.val);
            l1 = &l1.next.as_ref().unwrap();
        }

        write!(f, "[{}]", l1.val);
        Ok(())
    }
}

// Leetcode provides the input as this structure - adding this so local structure matches
pub fn convert_to_list(strs: &Vec<String>) -> Option<Box<ListNode>>  {
    
    // TODO: Make this not jank - have to do this to prime the loop
    let mut str1:String = strs[0].clone().chars().collect::<String>();
    let mut str2:String = strs[1].clone().chars().collect::<String>();    
    // let n1: i32 = 0;
    // let n2: i32 = 0;

    /* Since we are manually addng the first charachter of each string to the list, 
        remove it from the given list so it will work with our loop logic
     */
    String::truncate(&mut str1,strs[0].len() -1);
    String::truncate(&mut str2,strs[1].len() -1);

    // Prime the loop for the both lists by inserting the first element in each
    let mut l1:ListNode = ListNode::new(
                strs[0]
                    .chars()
                    .nth(strs[0].len() -1)
                    .unwrap()
                    .to_digit(10)
                    .unwrap() as i32
                );
    let mut l2:ListNode = ListNode::new(
                strs[1]
                    .chars()
                    .nth(strs[1].len() -1)
                    .unwrap()
                    .to_digit(10)
                    .unwrap() as i32
                );

    
    // println!("NEW STRING ISSSS - {:?}", str1);

    let cpy = l1.clone();
    for (i,element) in str1.chars().rev().enumerate() {
        l1 = l1.add(element.to_digit(10).unwrap() as i32);
        
    }

    for (i,element) in str2.chars().rev().enumerate() {
        l2 = l2.add(element.to_digit(10).unwrap() as i32);
    }
        
    println!("L1 = {}\nL2 = {}", l1, l2);

    add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)))

}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    
    let mut l1_ref = &l1;
    let mut l2_ref = &l2;
    let mut l1_str = String::new();
    let mut l2_str = String::new();
    let mut return_string:ListNode;

    let mut l1_num: i64 = 0;
    let mut l2_num: i64 = 0;
    let mut i = 0;
    let mut j = 0;

    while l1_ref.as_ref().unwrap().next != None {
        // l1_num += i64::pow(10, i) * l1_ref.as_ref().unwrap().val as i64;
        l1_str.push(std::char::from_digit(l1_ref.as_ref().unwrap().val.try_into().unwrap(), 10).unwrap());
        l1_ref = &l1_ref.as_ref().unwrap().next;
        // i += 1;
    }

    while l2_ref.as_ref().unwrap().next != None {
        l2_num += i64::pow(10, j) * l2_ref.as_ref().unwrap().val as i64;

        l2_ref = &l2_ref.as_ref().unwrap().next;
        j += 1;
    }

    l1_num += i64::pow(10, i) * l1_ref.as_ref().unwrap().val as i64;
    l2_num += i64::pow(10, j) * l2_ref.as_ref().unwrap().val as i64;
    
    // Uncomment for debugging the numeric representation of the lists
    println!("l1_num = {:?}", l1_num);
    println!("l2_num = {:?}", l2_num);
    // println!("l2_num = {:?}", l2_num);

    let mut sum = (l1_num+l2_num)
        .to_string()
        .chars()
        .collect::<String>();

    return_string = ListNode:: new(
        sum
            .chars()
            .nth(0)
            .unwrap()
            .to_digit(10)
            .unwrap() as i32
    );
    // Remove the first strng charachte rthat we've already added to the list
    sum.remove(0);

    // Add remainig string charachters as list nodes
    for digit in sum.chars() {
        return_string = return_string.add(digit.to_digit(10).unwrap() as i32);
    }

    Some(Box::new(return_string))
        
}