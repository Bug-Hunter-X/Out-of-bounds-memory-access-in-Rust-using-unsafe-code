fn main() {
    let mut v = vec![1, 2, 3];
    // Safe way to add a new element
    v.push(4);
    println!("Vector: {:?}", v);
} 