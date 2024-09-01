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
fn test_examples() {
    run!("bool.soop", ":true");
    run!("function.soop", ":d");
    run!(
        "inheritance.soop",
        "((:alice, :says, :hello), :to, :robert), :and, :alice, :says, :YEAH"
    );
    run!("list.soop", ":true");
    run!("map-free.soop", ":alex, :bob, :charlie");
    run!("map-managed.soop", ":alice, :bob, :none");
    run!("monad.soop", ":some, :alice_wins");
    run!("number.soop", ":s, :s, :s, :0");
    run!("option.soop", ":false");
    run!("state.soop", ":c");
}
