use soop::{deep_to_exp, eval, parse, print, Env};

macro_rules! run {
    ($filename:expr, $output:expr) => {
        let text = std::fs::read_to_string(format!("examples/{}", $filename)).unwrap();
        let exp = parse(&text);
        let deep = eval(Env::new(), exp);
        let exp = deep_to_exp(deep);
        let out = print(exp);
        assert_eq!(out, $output);
    };
}

#[test]
fn test_examples() {
    run!("bool.soop", ":true");
    run!(
        "inheritance.soop",
        "(:alice, :says, :WOO), :and, (:alice, :says, :hello), :to, :bob"
    );
    run!("list.soop", ":true");
    run!("map-free.soop", ":alex, :bob, :charlie");
    run!("map-managed.soop", ":alice, :bob, :none");
    run!("monad.soop", ":some, :alice_wins");
    run!("number.soop", ":s, :s, :s, :z");
    run!("option.soop", ":false");
    run!("state.soop", ":true");
}
