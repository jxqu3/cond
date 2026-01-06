//! A macro for matching on boolean conditions and patterns.
//!
//! Requires Rust 2024 edition for let chain support.
//!
//! For the full documentation, see [`cond`].

//! Uses `no_std` except during tests (which require `std`).
#![cfg_attr(not(test), no_std)]

#[macro_export]
/// A macro for matching on boolean conditions, like an empty [Go `switch` statement].
///
/// If the branches evalutate to `()`, you don't need a default branch.
///
/// ```
/// # use cond::cond;
/// let a = 4;
/// let b = 5;
/// cond! {
///     a < b => println!("a is less than b"),
///     a > b => println!("a is greater than b"),
/// }
/// ```
///
/// If the branches evalute to any type other than `()`, you must include a default branch. The
/// default branch uses `_` instead of a condition and must come last.
///
/// ```
/// # use cond::cond;
/// let a = 4;
/// let b = 5;
/// let text = cond! {
///     a < b => "a is less than b",
///     a > b => "a is greater than b",
///     _ => "a is equal to b",
/// };
/// assert_eq!(text, "a is less than b");
/// ```
///
/// # Pattern Matching
///
/// You can use `let` patterns to match on values:
///
/// ```
/// # use cond::cond;
/// let maybe_number = Some(42);
/// let result = cond! {
///     let Some(n) = maybe_number => n * 2,
///     _ => 0,
/// };
/// assert_eq!(result, 84);
/// ```
///
/// # Let Chains (Rust 2024)
///
/// You can use `&&` to chain conditions with `let` patterns:
///
/// ```
/// # use cond::cond;
/// let maybe_number = Some(15);
/// let result = cond! {
///     let Some(x) = maybe_number && x < 10 => "small",
///     let Some(x) = maybe_number && x >= 10 => "large",
///     _ => "none",
/// };
/// assert_eq!(result, "large");
/// ```
///
/// # Caveat
///
/// Expressions that end with blocks must still have commas after them in `cond` invocations, unlike
/// in `match` blocks.
///
/// The following `match` block does not need commas after each of its arms:
///
/// ```
/// let x = 5;
/// match x {
///     ..=4 => {
///         println!("x is 4 or less");
///     }
///     // No comma needed!
///     5.. => {
///         println!("x is 5 or greater");
///     }
/// }
/// ```
///
/// But the equivalent `cond` invocation fails to compile:
///
/// ```compile_fail
/// # use cond::cond;
/// let x = 5;
/// cond! {
///     x <= 4 => {
///         println!("x is 4 or less");
///     }
///     // Comma needed here!
///     x >= 5 => {
///         println!("x is 5 or greater");
///     }
/// }
/// ```
///
/// [Go `switch` statement]: <https://go.dev/ref/spec#Switch_statements>
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

#[cfg(test)]
mod tests;
