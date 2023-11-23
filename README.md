# cond
Rust macro to use a match-like syntax as a elegant alternative to nesting if-else statement.  
I got the idea from Go's empty switch statements. I thought it could be cool if it was in rust so I asked if that was possible in rust's discord server. They told me it wasn't unless you used a pretty ugly syntax in a match, and Esper89 (github in credits) made a macro for it. I added some tests and documentation and here's my first rust crate. 

# example:
```rust
use cond::cond;

fn main() {
    let a = 195;
    cond! {
        a < 5 => println!("a is less than 5"),
        a == 195 => {
            println!("this is the way")
        },
        a > 10 => println!("a is greater than 10"),

        // The conditions are executed by order: if one condition is true, conditions below will not get evaluated
    };

    let b = "";
    let result = cond! { // Or use it as a block to return a value
        b == "something" => false,
        b.chars().count() > 10 => true,
        a < 10000 => true,
        _ => false // You must add a default with the return type if you want to return
    };

    println!("result: {}", result);
}

```

# usage
You can just add the crate with
```bash
cargo add cond
```
Or just add the 4 line macro in your project:
```rust
macro_rules! cond {
    ($($condition:expr => $value:expr),* $(, _ => $default:expr)? $(,)?) => {
        match () {
            $(() if $condition => $value,)*
            () => ($($default)?),
        }
    };
}
```

# credits
Credits to github.com/Esper89 for essentially making the whole macro in the Rust discord server.
