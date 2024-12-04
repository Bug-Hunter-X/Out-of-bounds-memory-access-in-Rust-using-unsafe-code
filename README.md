# Out-of-bounds Memory Access in Rust

This repository demonstrates a common error in Rust: out-of-bounds memory access within unsafe code blocks.  The code attempts to write data beyond the allocated bounds of a vector, leading to undefined behavior (crashes, data corruption, etc.).  This example highlights the importance of careful memory management when working with unsafe Rust.

**Bug:** The `bug.rs` file contains the erroneous code.  The solution (`bugSolution.rs`) provides a corrected version using safe Rust techniques.

**Solution:** The solution uses safe Rust constructs to avoid memory access violations.  Always prioritize safe Rust practices to ensure memory safety and prevent vulnerabilities.