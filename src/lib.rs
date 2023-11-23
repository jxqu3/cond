#[macro_export]
// This is a macro to use a match with boolean conditions, like a empty switch in Go.
macro_rules! cond {
    ($($cond:expr => $value:expr),* $(, _ => $dft:expr)? $(,)?) => {
        match () {
            $(() if $cond => $value,)*
            () => ($($dft)?),
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = 1235;
        let mut result = false;
        cond! {
            a < 5 => result = false,
            a < 10 => result = false,
            a < 10000 => result = true,
        };
        assert_eq!(result, true);
        let result = cond! {
            a < 5 => false,
            a < 10 => false,
            a < 10000 => true,
            _ => false
        };
        assert_eq!(result, true);
    }
}
