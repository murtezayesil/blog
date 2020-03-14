# Using multiple Rust source files

I am learning [Rust Programming Language](httpd://www.rust-lang.org). Its way of taking care of data helps developers avoid many problems in the future.


#### src/square.rs

```rust
fn square_of( number:i32 ) -> i64 {
    number * number
}
```
  

#### src/main.rs

```rust
//  add square.rs as a MODule
//  mod PATH_TO_FILE (without .rs extension) ;
mod square;

fn main() {
    let number = 4;
    println!("Square of {} is {}", number, square_of(number) );
}
```

This program will *import* `square_of()` function form `square.rs`.

___

Until today, I have been writing all the code for a program into main.rs file, main source file for Rust.  
The problems with this strategy are:
 - code becomes hard to read and fix.
 - computer takes longer to compile 🤔️ [^1]
 - seeing unrelated code, of other functions, can break focus. (This may sound like a none-sense, but I lose focus easily)
 
 [^1]: Not always but parallel compiling should benefit from this. While making this claim I want to point out that I DO NOT know how Rust compiler handles parallel compiling. But it is a good practice none-the-less.
