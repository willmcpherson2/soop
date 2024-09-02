use soop::{deep_to_exp, eval, parse, print, Env};

macro_rules! run {
    ($filename:expr, $expected_output:expr) => {
        let text = std::fs::read_to_string(format!("examples/{}", $filename)).unwrap();
        let exp = parse(&text);
        let deep = eval(Env::new(), exp);
        let exp = deep_to_exp(deep);
        let output = print(exp);
        println!("output should match expected output");
        assert_eq!(output, $expected_output);

        let exp = parse(&output);
        let deep = eval(Env::new(), exp);
        let exp = deep_to_exp(deep);
        let eval_output = print(exp);
        println!("eval(output) should match output");
        assert_eq!(eval_output, output);
    };
}

#[test]
fn test_bool() {
    run!("bool.soop", ":true");
}

#[test]
fn test_function() {
    run!("function.soop", ":d");
}

#[test]
fn test_inheritance() {
    run!(
        "inheritance.soop",
        "((:alice, :says, :hello), :to, :robert), :and, :alice, :says, :YEAH"
    );
}

#[test]
fn test_list() {
    run!("list.soop", ":false, :true, :nil");
}

#[test]
fn test_map() {
    run!("map.soop", ":alex, :bob, :charlie");
}

#[test]
fn test_number() {
    run!("number.soop", ":s, :s, :s, :0");
}

#[test]
fn test_option() {
    run!("option.soop", ":some, :yes");
}

#[test]
fn test_state() {
    run!("state.soop", ":c");
}
