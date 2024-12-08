This repository demonstrates a common error in Rust: accessing an index out of bounds in a vector. The `bug.rs` file contains the erroneous code, which attempts to access an element beyond the vector's capacity. This leads to a runtime panic. The `bugSolution.rs` file shows how to safely handle this by checking the index before attempting to access it using the `.get()` method.