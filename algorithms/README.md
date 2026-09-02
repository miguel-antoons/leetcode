# Leetcode Solutions

Repository containing my own leetcode exercise solutions in Rust.

## Quick Start

### 1. Create a New Project

Use the provided script to create a new LeetCode project:

```bash
./create_leetcode_project "2236. Root Equals Sum of Children"
```

This will:
- Create a directory named `s_2236_Root_Equals_Sum_of_Children/` (sanitized title, with `s_` prefix if starting with a digit)
- Generate `Cargo.toml` with package configuration
- Create `src/lib.rs` with a Solution struct template (as a library)
- Create `src/main.rs` with a simple entry point
- Create `tests/test.rs` with a dummy integration test that imports from lib
- Create `REPORT.md` with problem documentation template
- Add the project to the root `Cargo.toml` workspace members

### 2. Implement Your Solution

Implement your solution in the `src/lib.rs` file (as a library):

```rust
// src/lib.rs
pub struct Solution;

impl Solution {
    pub fn check_tree(root: Option<Box<TreeNode>>) -> bool {
        // Your implementation here
        true
    }
}
```

The `src/main.rs` file contains a simple entry point:

```rust
// src/main.rs
fn main() {
    println!("Hello, LeetCode!");
}
```

### 3. Implement Tests

#### Integration Tests in tests/test.rs
The script creates `tests/test.rs` with a default template that imports your Solution from the library. Edit this file to add your tests:

```rust
use s_2236_Root_Equals_Sum_of_Children::Solution;

#[test]
fn test_example_1() {
    // Test case 1
    // assert!(Solution::check_tree(None));
}

#[test]
fn test_example_2() {
    // Test case 2
}
```

Note: Integration tests are external to your crate and test it as a user would. The crate name in the import matches the package name in Cargo.toml.

### 4. Run Tests and Code

#### Run all tests for a specific project:
```bash
cd s_2236_Root_Equals_Sum_of_Children
cargo test
```

#### Run tests for all workspace projects:
```bash
cd /path/to/leetcode  # root directory
cargo test --workspace
```

#### Run a specific project:
```bash
cd s_2236_Root_Equals_Sum_of_Children
cargo run
```

#### Run all workspace projects:
```bash
cargo run --workspace
```

## Project Structure

```
leetcode/
├── Cargo.toml                    # Root workspace configuration
├── create_leetcode_project       # Project creation script
├── README.md                     # This file
└── s_2236_Root_Equals_Sum_of_Children/  # Example project (note: s_ prefix for numeric titles)
    ├── Cargo.toml
    ├── REPORT.md                 # Problem documentation
    ├── src/
    │   ├── main.rs               # Entry point
    │   └── lib.rs                # Solution code (library)
    └── tests/
        └── test.rs               # Integration tests
```

## REPORT.md Template

Each project includes a `REPORT.md` file that can contain interesting information about the problem, your solution approach, and any relevant notes.
