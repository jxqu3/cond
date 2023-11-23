# `cond`

Rust macro to use a match-like syntax as an elegant alternative to many `if`-`else` statements.

I got the idea from empty [Go `switch` statements](https://go.dev/ref/spec#Switch_statements). I thought it could be cool if it was in Rust so I asked if that was possible in the Rust community Discord server. They told me it wasn't unless you used a pretty ugly syntax in a match, and Esper89 (GitHub in credits) made a macro for it. I added some tests and documentation and here's my first Rust crate.

## Example

```rs
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

## Usage

You can just add the crate with:

```sh
cargo add cond
```

Or just add the 8 line macro to your project:

```rs
macro_rules! cond {
    ($($condition:expr => $value:expr),* $(, _ => $default:expr)? $(,)?) => {
        match () {
            $(() if $condition => $value,)*
            () => ($($default)?),
        }
    };
}
```

## Credits

Credits to [Esper89](https://github.com/Esper89) for essentially making the whole macro in the Rust community Discord server.
