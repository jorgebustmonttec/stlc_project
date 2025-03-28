use super::Term::{self, *};

fn step_op1(
    ctor: impl FnOnce(Box<Term>) -> Term,
    eval: impl FnOnce(Box<Term>) -> Term,
    t: Box<Term>,
) -> Term {
    if t.is_value() {
        eval(t)
    } else {
        ctor(Box::new(t.step()))
    }
}

fn step_op2(
    ctor: impl FnOnce(Box<Term>, Box<Term>) -> Term,
    eval: impl FnOnce(Box<Term>, Box<Term>) -> Term,
    t1: Box<Term>,
    t2: Box<Term>,
) -> Term {
    match (t1.is_value(), t2.is_value()) {
        (false, _) => ctor(Box::new(t1.step()), t2).into(),
        (true, false) => ctor(t1, Box::new(t2.step())).into(),
        (true, true) => eval(t1, t2),
    }
}

fn eval_app(t1: Box<Term>, t2: Box<Term>) -> Term {
    if let Abs { var, ty: _, body } = *t1 {
        body.subst(&var, *t2)
    } else {
        panic!("attempted to apply non abstraction to a value");
    }
}

fn eval_let(val_t: Box<Term>, var: impl AsRef<str>, body: Box<Term>) -> Term {
    body.subst(var.as_ref(), *val_t)
}

impl Term {
    pub fn step(self) -> Self {
        match self {
            Var(y) => panic!("cannot evaluate a variable: {y}"),
            App(t1, t2) => step_op2(App, eval_app, t1, t2),

            Let { var, val_t, body } => step_op1(
                |val_t| Let {
                    var: var.clone(),
                    val_t,
                    body: body.clone(),
                },
                |val_t| eval_let(val_t, &var, body.clone()),
                val_t,
            ),

            Ite {
                cond,
                if_true,
                if_false,
                /*} => todo!(),*/
            } => match *cond {
                True => *if_true,
                False => *if_false,
                _ => Ite {
                    cond: Box::new(cond.step()),
                    if_true: if_true.clone(),
                    if_false: if_false.clone(),
                },
            },
            _ => panic!("cannot step a value"),
        }
    }

    pub fn multistep(mut self) -> Self {
        while !self.is_value() {
            self = self.step()
        }
        self
    }
}
