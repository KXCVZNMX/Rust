#[allow(unused_assignments)]
fn binary_search(v: &Vec<i32>, target: i32) -> Option<usize> {
    let length = v.len();
    let mut mid: usize = v.len() / 2;
    let mut found = false;

    while !found {
        if target == v[mid] {
            found = true;
            return Some(mid);
        } else if target > v[mid] {
            mid = mid + ((length - mid) / 2);
        } else if target < v[mid] {
            mid = mid / 2;
        }
        if mid == 0 || mid == length {
            break;
        }
    }
    None
}

fn main() {
    let v = vec![1, 3, 4, 5, 8, 9];
    let index = binary_search(&v, 3).unwrap_or_else(|| {
        panic!("This value does not exist");
    });
    println!("{}", index);
    println!("");
    let index2 = binary_search(&v, 0).unwrap_or_else(|| {
        panic!("This value does not exist");
    });
    println!("{}", index2);
}
