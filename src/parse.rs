use crate::{
    Bexp, Error,
    Exp::{self, *},
    Op,
    Pat::*,
    Side,
};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    character::complete::{alphanumeric1, char, multispace1},
    combinator::{all_consuming, map, recognize, success, value},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};

pub fn parse(input: &str) -> Exp {
    match parse_bexp(input) {
        Ok((_, bexp)) => parse_exp(bexp),
        Err(e) => Error(Error::ParseError(e.to_string())),
    }
}

fn parse_exp(bexp: Bexp) -> Exp {
    match bexp.clone() {
        Bexp::Binary(l, op, r) => match op {
            Op::Semicolon => match *l {
                Bexp::Binary(var, Op::Equals, exp) => match *var {
                    Bexp::Pat(Var(var)) => {
                        Let(var, Box::new(parse_exp(*exp)), Box::new(parse_exp(*r)))
                    }
                    bexp => Error(Error::ExpectedVar(bexp)),
                },
                bexp => Error(Error::ExpectedEquals(bexp)),
            },
            Op::Equals => Error(Error::UnexpectedEquals(bexp)),
            Op::Comma => Cons(Box::new(parse_exp(*l)), Box::new(parse_exp(*r))),
            Op::Arrow => match parse_exp(*l) {
                Pat(pat) => Fun(pat, Box::new(parse_exp(*r))),
                exp => Error(Error::ExpectedPat(Box::new(exp))),
            },
            Op::Empty => App(Box::new(parse_exp(*l)), Box::new(parse_exp(*r))),
        },
        Bexp::Parens(bexp) => parse_exp(*bexp),
        Bexp::Pat(pat) => Pat(pat),
        Bexp::Error(e) => Error(*e),
    }
}

fn parse_bexp(input: &str) -> IResult<&str, Bexp> {
    all_consuming(parse_bexp_non_greedy)(input)
}

fn parse_bexp_non_greedy(input: &str) -> IResult<&str, Bexp> {
    let (input, _) = junk(input)?;
    let (input, first) = parse_atom(input)?;
    let (input, rest) = many0(pair(preceded(junk, parse_op), preceded(junk, parse_atom)))(input)?;
    let (input, _) = junk(input)?;

    let exp = re_associate(left_associate(first, rest));

    Ok((input, exp))
}

fn parse_atom(input: &str) -> IResult<&str, Bexp> {
    alt((parse_parens, parse_sym, parse_var))(input)
}

fn parse_parens(input: &str) -> IResult<&str, Bexp> {
    map(
        delimited(char('('), parse_bexp_non_greedy, char(')')),
        |exp| Bexp::Parens(Box::new(exp)),
    )(input)
}

fn parse_sym(input: &str) -> IResult<&str, Bexp> {
    map(preceded(char(':'), ident), |s| {
        Bexp::Pat(Sym(s.to_string()))
    })(input)
}

fn parse_var(input: &str) -> IResult<&str, Bexp> {
    map(ident, |s| Bexp::Pat(Var(s.to_string())))(input)
}

fn ident(input: &str) -> IResult<&str, &str> {
    recognize(many1(alt((alphanumeric1, tag("_")))))(input)
}

fn parse_op(input: &str) -> IResult<&str, Op> {
    alt((
        value(Op::Semicolon, char(';')),
        value(Op::Equals, char('=')),
        value(Op::Comma, char(',')),
        value(Op::Arrow, tag("->")),
        value(Op::Empty, success(())),
    ))(input)
}

fn left_associate(first: Bexp, rest: Vec<(Op, Bexp)>) -> Bexp {
    rest.into_iter().fold(first, |acc, (op, exp)| {
        Bexp::Binary(Box::new(acc), op, Box::new(exp))
    })
}

fn re_associate(exp: Bexp) -> Bexp {
    // (a l b) r c

    let Bexp::Binary(left, r, c) = exp else {
        return exp;
    };
    let c = re_associate(*c);
    let left = re_associate(*left);
    let Bexp::Binary(a, l, b) = left else {
        return Bexp::Binary(Box::new(left), r, Box::new(c));
    };

    if r > l || r == l && r.assoc() == Side::Right {
        // a l (b r c)
        let left = a;
        let right = Bexp::Binary(b, r, Box::new(c));
        re_associate(Bexp::Binary(left, l, Box::new(right)))
    } else {
        // (a l b) r c
        let left = Bexp::Binary(a, l, b);
        let right = c;
        Bexp::Binary(Box::new(left), r, Box::new(right))
    }
}

fn junk(input: &str) -> IResult<&str, ()> {
    value(
        (),
        many0(alt((whitespace, line_comment, multi_line_comment))),
    )(input)
}

fn whitespace(input: &str) -> IResult<&str, ()> {
    value((), multispace1)(input)
}

fn line_comment(input: &str) -> IResult<&str, ()> {
    value((), pair(tag("--"), take_while(|c| c != '\n')))(input)
}

fn multi_line_comment(input: &str) -> IResult<&str, ()> {
    value((), tuple((tag("/*"), take_until("*/"), tag("*/"))))(input)
}
