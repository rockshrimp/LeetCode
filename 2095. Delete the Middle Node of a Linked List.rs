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
    pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut node = &head;
        let mut list_len = 0;

        //let step = 10;
        //let mut intermediate_nodes = Vec::new();
        while *node != None{
            //println!("{}", node.as_ref().unwrap().val);
            /*if list_len % step == 0{
                intermediate_nodes.push(node);
            }*/

            node = &node.as_ref().unwrap().next;
            list_len += 1;
        }

        if list_len == 1{
            return None;
        }

        let middle = list_len / 2;
        //println!("len is {}, middle is {}" , list_len, middle);

        let mut node = &mut head;
        for i in 0..(middle - 1){
            //println!("{}", node.as_mut().unwrap().val);
            node = &mut node.as_mut().unwrap().next;
        }

        let middle_node = &node.as_ref().unwrap().next;
        //println!("middle node val is {}", middle_node.as_ref().unwrap().val);

        if middle == list_len - 1{
            node.as_mut().unwrap().next = None;
            //println!("removing element after {}", node.as_mut().unwrap().val);
        }else{
            node.as_mut().unwrap().next = middle_node.as_ref().unwrap().next.clone();
        }        
        return head;
    }
}