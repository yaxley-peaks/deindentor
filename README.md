# Reverses the indentation on your file

## Usage
1. Install with cargo.

```cargo install deindentor```

2. Use it.

```deindentor <INPUT-FILE> <OUTPUT-FILE>```

## Example
#### Before:
```rust
fn foo(){
    if true {
        println!("true");
    } else {
        println!("false");
    }
}
```
#### After:
```rust
        fn foo(){
    if true {
println!("true");
    } else {
println!("false");
    }
        }
```


