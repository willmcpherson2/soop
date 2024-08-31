use crate::{
    Bexp,
    Exp::{self, *},
    Op, Pat, Side,
};

pub fn print(exp: Exp) -> String {
    print_bexp(print_exp(exp))
}

fn print_exp(exp: Exp) -> Bexp {
    match exp {
        Let(var, exp, body) => Bexp::Binary(
            Box::new(Bexp::Binary(
                Box::new(Bexp::Pat(Pat::Var(var))),
                Op::Equals,
                Box::new(with_parens(*exp, Op::Equals, Side::Right)),
            )),
            Op::Semicolon,
            Box::new(with_parens(*body, Op::Semicolon, Side::Right)),
        ),
        Cons(l, r) => Bexp::Binary(
            Box::new(with_parens(*l, Op::Comma, Side::Left)),
            Op::Comma,
            Box::new(with_parens(*r, Op::Comma, Side::Right)),
        ),
        Fun(pat, body) => Bexp::Binary(
            Box::new(Bexp::Pat(pat)),
            Op::Arrow,
            Box::new(with_parens(*body, Op::Arrow, Side::Right)),
        ),
        App(l, r) => Bexp::Binary(
            Box::new(with_parens(*l, Op::Empty, Side::Left)),
            Op::Empty,
            Box::new(with_parens(*r, Op::Empty, Side::Right)),
        ),
        Pat(pat) => Bexp::Pat(pat),
        Error(e) => Bexp::Error(Box::new(e)),
    }
}

fn with_parens(exp: Exp, parent: Op, side: Side) -> Bexp {
    let bexp = print_exp(exp);
    match bexp {
        Bexp::Binary(_, op, _) => {
            if op < parent || op == parent && op.assoc() != side {
                Bexp::Parens(Box::new(bexp))
            } else {
                bexp
            }
        }
        _ => bexp,
    }
}

fn print_bexp(exp: Bexp) -> String {
    match exp {
        Bexp::Binary(l, op, r) => {
            format!("{}{}{}", print_bexp(*l), print_op(op), print_bexp(*r))
        }
        Bexp::Parens(bexp) => format!("({})", print_bexp(*bexp),),
        Bexp::Pat(Pat::Var(var)) => var.to_string(),
        Bexp::Pat(Pat::Sym(sym)) => format!(":{}", sym),
        Bexp::Error(e) => format!("{:?}", e),
    }
}

fn print_op(op: Op) -> &'static str {
    match op {
        Op::Semicolon => "; ",
        Op::Equals => " == ",
        Op::Comma => ", ",
        Op::Arrow => " -> ",
        Op::Empty => " ",
    }
}
