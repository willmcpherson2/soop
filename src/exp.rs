use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Exp {
    Let(String, Box<Exp>, Box<Exp>),
    Cons(Box<Exp>, Box<Exp>),
    Fun(Pat, Box<Exp>),
    App(Box<Exp>, Box<Exp>),
    Pat(Pat),
    Error(Error),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pat {
    Var(String),
    Sym(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Data {
    Cons(Env, Box<Exp>, Box<Exp>),
    Fun(Env, Pat, Box<Exp>),
    Sym(String),
    Error(Error),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Deep {
    Cons(Box<Deep>, Box<Deep>),
    Fun(Env, Pat, Box<Exp>),
    Sym(String),
    Error(Error),
}

pub type Env = HashMap<String, Thunk>;

#[derive(Debug, Clone, PartialEq)]
pub struct Thunk(pub Env, pub Exp);

#[derive(Debug, Clone, PartialEq)]
pub enum Bexp {
    Binary(Box<Bexp>, Op, Box<Bexp>),
    Parens(Box<Bexp>),
    Pat(Pat),
    Error(Box<Error>),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
pub enum Op {
    Semicolon,
    Equals,
    Comma,
    Arrow,
    Empty,
}

impl Op {
    pub fn assoc(&self) -> Side {
        match *self {
            Op::Semicolon => Side::Right,
            Op::Equals => Side::Right,
            Op::Comma => Side::Right,
            Op::Arrow => Side::Right,
            Op::Empty => Side::Left,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    ParseError(String),
    ExpectedPat(Box<Exp>),
    UnexpectedEquals(Bexp),
    ExpectedVar(Bexp),
    ExpectedEquals(Bexp),
    Undefined(String),
    ApplySym(Box<Data>),
    ExpectedSym(Box<Data>),
    SymMismatch(String, String),
}
