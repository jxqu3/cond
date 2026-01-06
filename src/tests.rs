use super::*;

fn test_bool(test: &str, res: &mut String) -> bool {
    *res = test.to_string();
    println!("test: {}", test);
    true
}

#[test]
fn test_simple_case() {
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
        _ => println!("a is equal to 5"),
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

#[test]
fn test_let_pattern_with_options() {
    // Test with Some
    let maybe_number = Some(42);
    let result = cond! {
        let Some(n) = maybe_number => n * 2,
        _ => 0,
    };
    assert_eq!(result, 84);

    // Test with None
    let maybe_number: Option<i32> = None;
    let result = cond! {
        let Some(n) = maybe_number => n * 2,
        _ => 0,
    };
    assert_eq!(result, 0);

    // Test mixed conditions
    let a = 10;
    let no_number: Option<u32> = None;
    let maybe_number = Some(1);
    let result = cond! {
        a > 100 => 999,
        let Some(_) = no_number => 500,
        let Some(n) = maybe_number => n,
        _ => 50,
    };
    assert_eq!(result, 1);

    // Test let pattern with boolean condition after
    let maybe_val = None;
    let result = cond! {
        let Some(v) = maybe_val => v,
        a > 5 => 10,
        _ => 0,
    };
    assert_eq!(result, 10);
}

#[test]
fn test_let_pattern_with_result() {
    let ok_val: Result<i32, &str> = Ok(100);
    let result = cond! {
        let Ok(val) = ok_val => val,
        _ => -1,
    };
    assert_eq!(result, 100);

    let err_val: Result<i32, &str> = Err("error");
    let result = cond! {
        let Ok(val) = err_val => val,
        _ => -1,
    };
    assert_eq!(result, -1);
}

#[test]
fn test_let_pattern_short_circuit() {
    let mut side_effect = String::new();

    let maybe_number = Some(5);
    cond! {
        let Some(_) = maybe_number => side_effect.push_str("matched"),
        test_bool("should not execute", &mut side_effect) => {},
    };

    assert_eq!(side_effect, "matched");
}

#[test]
fn test_let_chain_with_conditions() {
    let maybe_number = Some(15);

    // Test with && chain
    let result = cond! {
        let Some(x) = maybe_number && x < 10 => "small",
        let Some(x) = maybe_number && x == 10 => "medium",
        let Some(x) = maybe_number && x > 10 => "large",
        _ => "none",
    };
    assert_eq!(result, "large");

    // Test multiple && conditions
    let maybe_number = Some(5);
    let result = cond! {
        let Some(x) = maybe_number && x < 10 && x > 0 => "valid",
        _ => "invalid",
    };
    assert_eq!(result, "valid");

    // Test that pattern fails with failed guard
    let maybe_number = Some(15);
    let result = cond! {
        let Some(x) = maybe_number && x < 10 => "small",
        _ => "not small",
    };
    assert_eq!(result, "not small");
}
