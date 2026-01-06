# `cond`

Rust macro to use a match-like syntax as an elegant alternative to many `if`-`else` statements.

I got the idea from empty [Go `switch` statements](https://go.dev/ref/spec#Switch_statements). I thought it could be cool if it was in Rust so I asked if that was possible in the Rust community Discord server. They told me it wasn't unless you used a pretty ugly syntax in a match, and Esper89 (GitHub in credits) made a macro for it. I added some tests and documentation and here's my first Rust crate.

**Note:** Requires Rust 2024 edition for let chain support.

## Examples

### With boolean expressions

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

### With pattern matching

You can also use `let` patterns to match on `Option`, `Result`, or any other pattern:

```rs
use cond::cond;

fn main() {
    let maybe_number = Some(42);
    let a = 10;

    let result = cond! {
        a > 100 => 999,
        let Some(number) = maybe_number => number * 2,
        _ => 0
    };

    println!("result: {}", result); // Output: result: 84

    // Works with Result too
    let operation: Result<i32, &str> = Ok(100);
    let value = cond! {
        let Ok(val) = operation => val,
        _ => -1
    };

    println!("value: {}", value); // Output: value: 100
}
```

### With let chains (Rust 2024)

You can use `&&` to chain conditions with `let` patterns:

```rs
use cond::cond;

fn main() {
    let maybe_number = Some(15);

    let result = cond! {
        let Some(x) = maybe_number && x < 10 => "small",
        let Some(x) = maybe_number && x >= 10 => "large",
        _ => "none"
    };

    println!("result: {}", result); // Output: result: large
}
```

## Usage

You can just add the crate with:

```sh
cargo add cond
```

Or just add the macro to your project (requires Rust 2024 edition):

```rs
macro_rules! cond {
    // Match let patterns - start token accumulation
    (let $pat:pat = $($rest:tt)*) => {
        cond!(@let $pat = () $($rest)*)
    };

    // Accumulate tokens after = until we hit =>
    (@let $pat:pat = ($($acc:tt)*) => $value:expr $(, $($rest:tt)*)?) => {
        if let $pat = $($acc)* { $value } else { cond!($($($rest)*)?) }
    };
    (@let $pat:pat = ($($acc:tt)*) $next:tt $($rest:tt)*) => {
        cond!(@let $pat = ($($acc)* $next) $($rest)*)
    };

    // Default branch - must come before boolean expressions to avoid ambiguity
    (_ => $default:expr $(,)?) => {
        $default
    };

    // Match boolean expressions
    ($condition:expr => $value:expr $(, $($rest:tt)*)?) => {
        if $condition { $value } else { cond!($($($rest)*)?) }
    };

    // Empty (unit return)
    () => {
        ()
    };
}
```

## Credits

Credits to [Esper89](https://github.com/Esper89) for essentially making the whole macro in the Rust community Discord server.
