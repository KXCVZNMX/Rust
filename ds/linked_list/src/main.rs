use linked_list::ListNode;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let mut l1 = ListNode::new(1);
    let temp = ListNode::from_vec(vec![2, 3, 4, 5]);
    l1.next = Some(temp);
    l1.print();

    ListNode::from_vec(vec!["a", "b", "c", "d", "e"]).print();

    let mut l3 = ListNode::from_vec(vec![2, 3, 4, 5]);
    l3.push(1);
    l3.print();

    let mut l4 = ListNode::from_vec(vec![1, 2, 3, 4]);
    l4.push_back(5);
    l4.print();

    let mut l5 = ListNode::from_vec(vec![1, 2, 2, 3, 4, 5]);
    match l5.delete(2) {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }
    l5.print();

    let mut l6 = ListNode::from_vec(vec![3, 3, 3, 1, 2, 3, 4, 5]);
    let l6 = match l6.find(1) {
        Ok(res) => res,
        Err(e) => panic!("{e}"),
    };
    l6.print();

    let mut l7 = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
    let length = l7.len();
    println!("{length}");

    let mut l8 = ListNode::from_vec(vec![5, 4, 3, 2, 1]);
    l8.reverse();
    l8.print();

    let temp = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
    let l9 = temp.copy();
    l9.print();

    let l10 = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
    let element = l10[1];
    let element2 = l10[2];
    println!("{0} {1}", element, element2);

    let mut l11 = ListNode::from_vec(vec![1, 3, 4, 5]);
    let _ = match l11.insert(1, 2) {
        Ok(_) => (),
        Err(e) => panic!("{e}"),
    };
    l11.print();

    let mut l12 = ListNode::from_vec(vec![1, 1, 2, 3, 4, 5]);
    let poped = l12.pop().unwrap();
    println!("{poped}");
    l12.print();

    let l13 = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
    let found = l13.contains(3);
    println!("{found:?}");
    let notfound = l13.contains(0);
    println!("{notfound:?}");

    let t1 = ListNode::from_vec(vec![1, 3, 5]);
    let t2 = ListNode::from_vec(vec![2, 4]);
    let l16 = ListNode::<i32>::merge(Some(t1), Some(t2)).unwrap();
    l16.print();

    let l17 = ListNode::from_vec(vec![1, 2, 3, 4, 5, 1, 2, 3, 4, 5]);
    let v1 = ListNode::<i32>::split_list(Some(l17), 2);
    let (l18, l19) = (v1[0].clone().unwrap(), v1[1].clone().unwrap());
    l18.print();
    l19.print();

    let l18 = ListNode::from_vec(vec![1, 5, 3, 2, 4]);
    let l18 = ListNode::<i32>::sort(Some(l18)).unwrap();
    l18.print();

    let elapsed = now.elapsed();
    println!("\nTime Elapsed: {:.4?}", elapsed);
}
