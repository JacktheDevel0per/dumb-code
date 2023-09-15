
struct Solution {}




// Definition for singly-linked list.
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
}


impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        

        //follow the pointer, add the values combined, and carry the addon, wipe it go again, until both are none, if one is over, act like it is 0


        //no need to clone as it is set in place not mutated. Does not affect the original list


    

        let mut out = Box::new(ListNode::new(0));
       
        let mut out_ptr = out.as_mut();
        


        let mut first_list = &l1;

        let mut second_list = &l2;





        let mut tens_place = 0;


        loop {


   

   
            //check if both are none, if so, break, ready for the addition of the tens place
            if (&first_list).is_none() && (&second_list).is_none() {
                break;
            }



            let mut set_val: Option<i32> = None;
           


            let mut first_val = 0;
            let mut second_val = 0;

            if let Some(first) = first_list {
                first_val = first.val;
            }

            if let Some(second) = second_list {
                second_val = second.val;
            }

            let sum = first_val + second_val + tens_place;

            if sum > 9 {
                set_val= Some(sum - 10);
                tens_place = 1;
            } else {
                set_val = Some(sum);
                tens_place = 0;

            }





            if let Some(in_val) = set_val {
                out_ptr.next = Some(Box::new(ListNode::new(in_val)));
            }







           

            //push over the lists

            if first_list.as_ref().is_some() {
                first_list = &(first_list.as_ref().unwrap().next);
            }

            if second_list.as_ref().is_some() {
                second_list = &(second_list.as_ref().unwrap().next);
            }


            if out_ptr.next.is_some() {
                out_ptr = match out_ptr.next {
                    Some(ref mut boxed) => boxed,
                    None => unreachable!(),
                };
            }

        
            

            // if let Some(next_val) = out_ptr.next {
            //     out_ptr = next_val;
            // }


            //set the next value to a new node, this should work as I have already checked for the next value being none

            // out_ptr.next = None/
  
            }


            //add the tens place if it is greater than 0

            if tens_place > 0 {
                out_ptr.next = Some(Box::new(ListNode::new(tens_place)));

            }
    






            return Some(out.next.unwrap_or_else(|| Box::new(ListNode::new(0))));
         }






    
    }