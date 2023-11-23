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

fn test_bool(test: &str, res: &mut String) -> bool {
    *res = test.to_string();
    println!("test: {}", test);
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = 195;
        let mut result = false;
        let mut res = String::new();
        cond! {
            a < 5 => println!("a is less than 5"),
            test_bool("This will get executed and nothing else", &mut res) => {
                println!("this is the way");
                result = true
            },
            test_bool("This will not get executed as the condition before is true", &mut res) => println!("a is greater than 10"),
        };
        assert_eq!(result, true);
        assert_eq!(res, "This will get executed and nothing else");
        let b = "";
        let result = cond! {
            b.chars().count() < 10 => true,
            _ => false
        };
        assert_eq!(result, true);
    }
}
