use core::fmt;
use std::{cmp::{self, max}, ops::Deref};

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

  /// Reuturns the number of nodes in the given list
  pub fn length(&self) -> i32{
    let mut l1 = self;
    let mut i = 0;

        while l1.next != None {
            l1 = l1.next.as_ref().unwrap();
            i += 1;
        }

    i + 1
  }

  /// Add and return a new node to a given list
  pub fn add(self, val: i32) -> ListNode{
    let new_node = ListNode {
        val: val,
        next: Some(Box::new(self))
    };
    new_node
  }

  /// Returns a modified list with padded zeros inserted at the beginning until list is digits numbered of nodes long
  pub fn pad_zeros(self, digits: i32) -> ListNode {

    let mut l1 = self;

        for i in 0..digits {
            l1 = l1.add(0);
        }
    l1
  }

  /// Returns a new list in modified order of the original
  pub fn reverse(self) -> ListNode {
    let mut l1 = self;
    let mut return_list = ListNode::new(l1.val);

    while l1.next.is_some() {
        l1 = *l1.next.unwrap();
        return_list = return_list.add(l1.val)
    }
    return_list
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

// Leetcode provides the input as this structure - adding this as extra challenge to convert string to list
pub fn convert_to_list(strs: &Vec<String>) -> Option<Box<ListNode>>  {
    
    // TODO: Make this not jank - have to do this to prime the loop
    let mut str1:String = strs[0].clone().chars().collect::<String>();
    let mut str2:String = strs[1].clone().chars().collect::<String>();    

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
    for (_,element) in str1.chars().rev().enumerate() {
        l1 = l1.add(element.to_digit(10).unwrap() as i32);
        
    }

    for (_,element) in str2.chars().rev().enumerate() {
        l2 = l2.add(element.to_digit(10).unwrap() as i32);
    }

    add(Some(Box::new(l1)), Some(Box::new(l2)))

}

pub fn add(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut carry = 0;
    let mx_digs = max(l1.as_ref().unwrap().length(),l2.as_ref().unwrap().length());
    let l1_digs = l1.as_ref().unwrap().length();
    let l2_digs = l2.as_ref().unwrap().length();
    let mut l1_ref = Some(Box::new(l1.as_ref().unwrap().clone().reverse().pad_zeros(mx_digs - l1_digs).reverse()));
    let mut l2_ref = Some(Box::new(l2.as_ref().unwrap().clone().reverse().pad_zeros(mx_digs - l2_digs).reverse()));
    let mut return_list = ListNode::new(l1_ref.as_ref().unwrap().val + l2_ref.as_ref().unwrap().val);

    // Uncomment for debugigng output
    // println!("L1 = {}\nL2 = {}",l1_ref.as_ref().unwrap(), l2_ref.as_ref().unwrap());
    // println!("[-1] {:?} + {:?} = {:?}",l1_ref.clone().unwrap().val, l2_ref.clone().unwrap().val, return_list.val);

    // Handle carry if it occurs on first digit
    if return_list.val >= 10 {
        return_list.val %= 10;
        carry = 1;
    }

    // Manually progress list to second node
    l1_ref = l1_ref.unwrap().next;
    l2_ref = l2_ref.unwrap().next;

    // Iterate through the list of nodes and add them, handling carrys on next loop iteration
    for i in 0..max(l1.unwrap().length(),l2.unwrap().length()) {
        if l1_ref.as_ref().is_some() && l2_ref.as_ref().is_some(){
            if l1_ref.as_ref().unwrap().val + l2_ref.as_ref().unwrap().val + carry >= 10 {
                return_list = return_list.add((l1_ref.as_ref().unwrap().val + l2_ref.as_ref().unwrap().val + carry) % 10);
                
                // Uncomment for debugging
                // println!("[{:?}] {:?} + {:?} + {:?} = {:?}",i, l1_ref.as_ref().unwrap().val, l2_ref.as_ref().unwrap().val, carry, return_list.val);
                carry = 1;
            }
            else {
                return_list = return_list.add(l1_ref.as_ref().unwrap().val + l2_ref.as_ref().unwrap().val + carry);
                // Uncomment for debugging
                // println!("[{:?}] {:?} + {:?} + {:?} = {:?}",i, l1_ref.as_ref().unwrap().val, l2_ref.as_ref().unwrap().val, carry, return_list.val);
                carry = 0;
            }
            
            // Advance loop forward to next node
            l1_ref = l1_ref.unwrap().next;
            l2_ref = l2_ref.unwrap().next;
        }   
    }
    if carry == 1 {
        return_list = return_list.add(1);
    }

    // Reverse output as per leetcode instructions
    Some(Box::new(return_list.reverse()))   
}