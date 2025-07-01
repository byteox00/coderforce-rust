// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut node = &head;
        for _ in 0..k {
            if node.is_none() {
                return head;
            }
            node = &node.as_ref().unwrap().next;
        }
        // Reverse first k nodes
        let mut prev = None;
        let mut curr = head;
        for _ in 0..k {
            let mut n = curr.unwrap();
            curr = n.next.take();
            n.next = prev;
            prev = Some(n);
        }
        // Connect with recursion
        let tail = prev.as_mut().unwrap();
        let mut tail_ptr = tail;
        while let Some(ref mut next_node) = tail_ptr.next {
            tail_ptr = next_node;
        }
        tail_ptr.next = Solution::reverse_k_group(curr, k);
        prev
    }
}

// Helper to convert a vector to a linked list
fn vec_to_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &num in nums.iter().rev() {
        let mut node = Box::new(ListNode::new(num));
        node.next = head;
        head = Some(node);
    }
    head
}

fn v2l(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    let mut curr = &mut head;
    for num in nums {
        *curr = Some(Box::new(ListNode::new(num)));
        curr = &mut curr.as_mut().unwrap().next
    }
    head
}

fn v2l1(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    let mut curr: &mut Option<Box<ListNode>> = &mut head;
    for num in nums {
        *curr = Some(Box::new(ListNode::new(num)));
        curr = &mut curr.as_mut().unwrap().next;
    }
    head
}

// Helper to print a linked list
fn print_list(mut head: Option<Box<ListNode>>) {
    while let Some(node) = head {
        print!("{}", node.val);
        head = node.next;
        if head.is_some() {
            print!(" -> ");
        }
    }
    println!();
}

// Example usage
fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let k = 2;

    let list = v2l(nums);
    println!("Original list:");
    print_list(list.clone());

    let result = Solution::reverse_k_group(list, k);
    println!("Reversed in groups of {}:", k);
    print_list(result);
}
