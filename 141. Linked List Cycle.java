/**
 * Definition for singly-linked list.
 * class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode(int x) {
 *         val = x;
 *         next = null;
 *     }
 * }
 */
public class Solution {
    public boolean hasCycle(ListNode head) {
        if (head == null){
            return false;
        }
        HashSet<ListNode> set = new HashSet<>();
        ListNode current = head;
        while (current.next != null){
            if (set.contains(current)){
                return true;
            }else{
                set.add(current);
            }
            current = current.next;
        }
        return false;
    }
}