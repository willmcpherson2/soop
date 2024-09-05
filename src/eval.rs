use crate::{
    Data, Deep, Env, Error,
    Exp::{self, *},
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
        Fun(pat, body) => Data::Fun(env, pat, body),
        App(fun, arg) => apply(env, *fun, *arg),
        Var(var) => resolve(env, var),
        Sym(sym) => Data::Sym(sym),
        Error(e) => Data::Error(e),
    }
}

fn apply(env: Env, fun: Exp, arg: Exp) -> Data {
    match eval_lazy(env.clone(), fun) {
        Data::Cons(cons_env, l, r) => {
            let env = extend(env, cons_env);
            match apply(env.clone(), *l, arg.clone()) {
                Data::Error(_) => apply(env, *r, arg),
                data => data,
            }
        }
        Data::Fun(fun_env, pat, body) => {
            let env = extend(env, fun_env);
            match pattern_match(env, *pat, arg) {
                Ok(env) => eval_lazy(env, *body),
                Err(e) => Data::Error(e),
            }
        }
        Data::Sym(sym) => Data::Error(Error::ApplySym(Box::new(Data::Sym(sym)))),
        Data::Error(e) => Data::Error(e),
    }
}

fn pattern_match(env: Env, pat: Exp, arg: Exp) -> Result<Env, Error> {
    match pat {
        Let(var, exp, body) => match arg {
            Let(_, arg_exp, arg_body) => Ok(extend(
                pattern_match(env.clone(), *exp, *arg_exp)?,
                pattern_match(env, *body, *arg_body)?,
            )),
            other => Err(Error::PatternMatchExp(
                Box::new(Let(var, exp, body)),
                Box::new(other),
            )),
        },
        Cons(l, r) => match eval_lazy(env.clone(), arg) {
            Data::Cons(arg_env, arg_l, arg_r) => {
                let env = extend(env, arg_env);
                Ok(extend(
                    pattern_match(env.clone(), *l, *arg_l)?,
                    pattern_match(env, *r, *arg_r)?,
                ))
            }
            other => Err(Error::PatternMatchData(
                Box::new(Cons(l, r)),
                Box::new(other),
            )),
        },
        Fun(pat, body) => match eval_lazy(env.clone(), arg) {
            Data::Fun(arg_env, arg_pat, arg_body) => {
                let env = extend(env, arg_env);
                Ok(extend(
                    pattern_match(env.clone(), *pat, *arg_pat)?,
                    pattern_match(env, *body, *arg_body)?,
                ))
            }
            other => Err(Error::PatternMatchData(
                Box::new(Fun(pat, body)),
                Box::new(other),
            )),
        },
        App(l, r) => match arg {
            App(arg_l, arg_r) => Ok(extend(
                pattern_match(env.clone(), *l, *arg_l)?,
                pattern_match(env, *r, *arg_r)?,
            )),
            other => Err(Error::PatternMatchExp(Box::new(App(l, r)), Box::new(other))),
        },
        Var(var) => Ok(bind(env, var, arg)),
        Sym(sym) => match eval_lazy(env.clone(), arg) {
            Data::Sym(arg_sym) => {
                if sym == arg_sym {
                    Ok(env)
                } else {
                    Err(Error::PatternMatchSym(sym, arg_sym))
                }
            }
            other => Err(Error::PatternMatchData(Box::new(Sym(sym)), Box::new(other))),
        },
        Exp::Error(e) => Err(e),
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

fn extend(mut env: Env, new_env: Env) -> Env {
    env.extend(new_env);
    env
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
        Deep::Sym(sym) => Sym(sym),
        Deep::Error(e) => Error(e),
    }
}
