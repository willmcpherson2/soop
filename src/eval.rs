use crate::{
    Data, Deep, Env, Error,
    Exp::{self, *},
    Pat::*,
    Thunk,
};

pub fn eval(env: Env, exp: Exp) -> Deep {
    eval_deep(eval_lazy(env, exp))
}

fn eval_deep(data: Data) -> Deep {
    match data {
        Data::Cons(env, l, r) => {
            Deep::Cons(Box::new(eval(env.clone(), *l)), Box::new(eval(env, *r)))
        }
        Data::Fun(env, l, r) => Deep::Fun(env, l, r),
        Data::Sym(sym) => Deep::Sym(sym),
        Data::Error(e) => Deep::Error(e),
    }
}

fn eval_lazy(env: Env, exp: Exp) -> Data {
    match exp {
        Let(var, exp, body) => eval_lazy(bind(env, var, *exp), *body),
        Cons(l, r) => Data::Cons(env, l, r),
        Fun(param, body) => Data::Fun(env, param, body),
        App(fun, arg) => apply(env, *fun, *arg),
        Pat(Var(var)) => resolve(env, var),
        Pat(Sym(sym)) => Data::Sym(sym),
        Error(e) => Data::Error(e),
    }
}

fn apply(mut env: Env, fun: Exp, arg: Exp) -> Data {
    match eval_lazy(env.clone(), fun) {
        Data::Cons(cons_env, l, r) => {
            env.extend(cons_env);
            match apply(env.clone(), *l, arg.clone()) {
                Data::Error(_) => apply(env, *r, arg),
                data => data,
            }
        }
        Data::Fun(fun_env, param, body) => {
            env.extend(fun_env);
            match param {
                Var(var) => eval_lazy(bind(env, var, arg), *body),
                Sym(param) => match eval_lazy(env.clone(), arg) {
                    Data::Sym(arg) if param == arg => eval_lazy(env, *body),
                    Data::Sym(arg) => Data::Error(Error::SymMismatch(param, arg)),
                    Data::Error(e) => Data::Error(e),
                    data => Data::Error(Error::ExpectedSym(Box::new(data))),
                },
            }
        }
        Data::Sym(sym) => Data::Error(Error::ApplySym(Box::new(Data::Sym(sym)))),
        Data::Error(e) => Data::Error(e),
    }
}

fn bind(mut env: Env, var: String, exp: Exp) -> Env {
    env.insert(var, Thunk(env.clone(), exp));
    env
}

fn resolve(mut env: Env, var: String) -> Data {
    match env.remove(&var) {
        Some(Thunk(env, exp)) => eval_lazy(env, exp),
        None => Data::Error(Error::Undefined(var)),
    }
}

pub fn deep_to_exp(deep: Deep) -> Exp {
    match deep {
        Deep::Cons(l, r) => Cons(Box::new(deep_to_exp(*l)), Box::new(deep_to_exp(*r))),
        Deep::Fun(env, l, r) => env
            .into_iter()
            .fold(Fun(l, r), |body, (var, Thunk(env, exp))| {
                let exp = deep_to_exp(eval(env, exp));
                Let(var, Box::new(exp), Box::new(body))
            }),
        Deep::Sym(sym) => Pat(Sym(sym)),
        Deep::Error(e) => Error(e),
    }
}
