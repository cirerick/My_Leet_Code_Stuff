// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut carry: i32 = 0; //store tenth place of sum
        let mut sum_of_lists: i32 = 0; //get sum of both lists
        //new list that we will return
        let mut new_and_improved_list: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
        
        let mut ptr1: &Option<Box<ListNode>> = &l1;
        let mut ptr2: &Option<Box<ListNode>> = &l2;
        let mut ptr3: &mut Option<Box<ListNode>> = &mut new_and_improved_list; 
        
        while ptr1.is_some() || ptr2.is_some() {
            //get value from first while iterating
            match ptr1 {
                Some(ref node1) => {
                    sum_of_lists = node1.val;
                    ptr1 = &node1.next;
                },
                None => {
                    sum_of_lists = 0;
                }
            }
            //get value of second list while iterating
            match ptr2 {
                Some(ref node2) => {
                    sum_of_lists = sum_of_lists + node2.val;
                    ptr2 = &node2.next;
                },
                None => {
                    sum_of_lists = sum_of_lists + 0; 
                }
            }

            match ptr3 {
                Some(ref mut node3) => {
                    node3.val = (sum_of_lists + carry) % 10;
                    node3.next = Some(Box::new(ListNode::new(0)));
                    ptr3 = &mut node3.next
                },
                None => {
                    println!("Nothing here.");
                }
            }

            //store the 10th digit of sum
            sum_of_lists = sum_of_lists + carry;
            carry = sum_of_lists / 10;
            //perpare to do it again
            sum_of_lists = 0;
        }

        //lose the last node //not really sure how to intregate within while loop without breaking it
        if carry > 0 {
            match ptr3 {
                Some(ref mut node3) => {
                    node3.val = carry;
                },
                None => {
                    println!("Nothing here.");
                },
            }
        } else {
            ptr3.take();
        }

        return new_and_improved_list;
    }
}