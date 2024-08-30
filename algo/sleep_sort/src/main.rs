use std::thread;
use std::time::Duration;

fn sleep_sort(v: Vec<u64>) {
    let mut handles = vec![];

    for &num in &v {
        handles.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(num));
            println!("{num}");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    let v = vec![35, 124, 326, 2, 124, 1];
    sleep_sort(v);
}
