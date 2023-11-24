//! A macro for matching on boolean conditions.
//!
//! For the full documentation, see [`cond`].

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
    ($($condition:expr => $value:expr),* $(, _ => $default:expr)? $(,)?) => {
        $(if $condition { $value } else)*
        { $($default)? }
    };
}

#[cfg(test)]
mod tests;
