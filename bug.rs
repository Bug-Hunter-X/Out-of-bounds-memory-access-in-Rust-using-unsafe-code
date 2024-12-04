fn main() {
    let mut v = vec![1, 2, 3];
    let ptr = v.as_mut_ptr();
    // Here is the bug: unsafe code without proper bounds checking
    unsafe {
        *ptr.add(3) = 4; // Accessing memory out of bounds
    }
}