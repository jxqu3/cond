use super::*;
    fn test_bool(test: &str, res: &mut String) -> bool {
        *res = test.to_string();
        println!("test: {}", test);
        true
    }

    #[test]
    fn it_works() {
        let a = 195;
        let b = 195;
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
        cond! { a > b => {}, a < b => {} }

        cond! {
            ,_ => println!("a is equal to 5"),
        }
        cond! {
            a > 45 => println!("a is equal to 5"),
        }
        assert_eq!(result, true);
        assert_eq!(res, "This will get executed and nothing else");
        let b = "";
        let result = cond! {
            b.chars().count() < 10 => true,
            _ => false,
        };
        assert_eq!(result, true);
    }