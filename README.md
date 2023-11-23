# cond
Rust macro to use a match-like syntax as a elegant alternative to nesting if-else statement.  
This was 

# example:
```rust
let a = 195
cond! {
    a < 5 => println!("a is less than 5"),
    a == 195 => {
        println!("this is the way")
    },
    a > 10 => println!("a is greater than 10"),

    // The conditions are executed by order: if one condition is true, conditions below will not get evaluated
};

let b = ""
let result = cond! { // Or use it as a block to return a value
    b == "something" => false,
    b.chars().count() > 10 => true,
    a < 10000 => true,
    _ => false // You must add a default with the return type if you want to return
};
```

# usage
You can just add the crate with
```bash
cargo add cond
```
Or just add the 4 line macro in your project:
```rust
macro_rules! cond {
    ($($cond:expr => $value:expr),* $(, _ => $dft:expr)? $(,)?) => {
        match () {
            $(() if $cond => $value,)*
            () => ($($dft)?),
        }
    };
}
```

# credits
Credits to github.com/Esper89 for essentially making the whole macro in the Rust discord server.