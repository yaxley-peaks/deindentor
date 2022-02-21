# Reverses the indentation on your file

## Usage
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


