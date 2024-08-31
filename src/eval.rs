use crate::{Data, Deep, Env, Error, Exp, Pat, Thunk};

pub fn eval(exp: Exp, env: Env) -> Deep {
    eval_deep(eval_lazy(exp, env))
}

fn eval_deep(data: Data) -> Deep {
    match data {
        Data::Cons(env, l, r) => {
            Deep::Cons(Box::new(eval(*l, env.clone())), Box::new(eval(*r, env)))
        }
        Data::Fun(_, l, r) => Deep::Fun(l, r),
        Data::Sym(sym) => Deep::Sym(sym),
        Data::Error(e) => Deep::Error(e),
    }
}

fn eval_lazy(exp: Exp, mut env: Env) -> Data {
    match exp {
        Exp::Let(var, exp, body) => eval_lazy(*body, bind(env, var, *exp)),
        Exp::Cons(l, r) => Data::Cons(env, l, r),
        Exp::Fun(param, body) => Data::Fun(env, param, body),
        Exp::App(fun, arg) => match eval_lazy(*fun, env.clone()) {
            Data::Cons(cons_env, l, r) => {
                env.extend(cons_env);
                match eval_lazy(Exp::App(l, arg.clone()), env.clone()) {
                    Data::Error(_) => eval_lazy(Exp::App(r, arg), env),
                    data => data,
                }
            }
            Data::Fun(fun_env, param, body) => {
                env.extend(fun_env);
                match param {
                    Pat::Var(var) => eval_lazy(*body, bind(env.clone(), var, *arg)),
                    Pat::Sym(param) => match eval_lazy(*arg, env.clone()) {
                        Data::Sym(arg) if param == arg => eval_lazy(*body, env),
                        Data::Sym(arg) => Data::Error(Error::SymMismatch(param, arg)),
                        Data::Error(e) => Data::Error(e),
                        data => Data::Error(Error::ExpectedSym(Box::new(data))),
                    },
                }
            }
            Data::Sym(sym) => Data::Error(Error::ApplySym(Box::new(Data::Sym(sym)))),
            Data::Error(e) => Data::Error(e),
        },
        Exp::Pat(Pat::Var(var)) => resolve(env, var),
        Exp::Pat(Pat::Sym(sym)) => Data::Sym(sym),
        Exp::Error(e) => Data::Error(e),
    }
}

fn bind(mut env: Env, var: String, exp: Exp) -> Env {
    env.insert(var, Thunk(env.clone(), exp));
    env
}

fn resolve(env: Env, var: String) -> Data {
    match env.get(&var).cloned() {
        Some(Thunk(env, exp)) => eval_lazy(exp, env),
        None => Data::Error(Error::Undefined(var)),
    }
}
