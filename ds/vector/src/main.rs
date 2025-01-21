use vector::Vector;

fn main() {
    let _v1: Vector<i32> = Vector::new();
    let _v2: Vector<i32> = Vector::with_capacity(10);
    let mut v3: Vector<i32> = Vector::new();
    v3.push(1);
    v3.push(2);
    v3.push(3);
    v3.print();
    let mut v4: Vector<i32> = Vector::with_capacity(3);
    v4.push(1);
    v4.push(2);
    v4.push(3);
    v4.print();
    println!("{0} {1} {2}", v3[0], v4[1], v3[2]);
    v4.pop();
    v4.print();
}
