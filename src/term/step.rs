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

fn eval_ite(cond: Box<Term>, if_true: Box<Term>, if_false: Box<Term>) -> Term {
    match *cond {
        True => *if_true,
        False => *if_false,
        _ => panic!("attempted to if-then-else with non boolean condition"),
    }
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
            } => step_op1(
                |cond| Ite {
                    cond,
                    if_true: if_true.clone(),
                    if_false: if_false.clone(),
                },
                |cond| eval_ite(cond, if_true.clone(), if_false.clone()),
                cond,
            ),
            /*

            Add(t1, t2) => todo!(),
            Sub(t1, t2) => todo!(),
            Mul(t1, t2) => todo!(),

            Eq(t1, t2) => todo!(),
            Ne(t1, t2) => todo!(),
            Lt(t1, t2) => todo!(),
            Le(t1, t2) => todo!(),
            Gt(t1, t2) => todo!(),
            Ge(t1, t2) => todo!(),

            */
            Add(t1, t2) => step_op2(
                Add,
                |t1, t2| match (*t1, *t2) {
                    (Int(i1), Int(i2)) => Int(i1 + i2),
                    (other1, other2) => Add(Box::new(other1), Box::new(other2)),
                },
                t1,
                t2,
            ),
            Sub(t1, t2) => step_op2(
                Sub,
                |t1, t2| match (*t1, *t2) {
                    (Int(i1), Int(i2)) => Int(i1 - i2),
                    (other1, other2) => Sub(Box::new(other1), Box::new(other2)),
                },
                t1,
                t2,
            ),
            Mul(t1, t2) => step_op2(
                Mul,
                |t1, t2| match (*t1, *t2) {
                    (Int(i1), Int(i2)) => Int(i1 * i2),
                    (other1, other2) => Mul(Box::new(other1), Box::new(other2)),
                },
                t1,
                t2,
            ),
            Eq(t1, t2) => step_op2(
                Eq,
                |t1, t2| match (*t1, *t2) {
                    (Int(i1), Int(i2)) => {
                        if i1 == i2 {
                            True
                        } else {
                            False
                        }
                    }
                    (other1, other2) => Eq(Box::new(other1), Box::new(other2)),
                },
                t1,
                t2,
            ),
            Ne(t1, t2) => step_op2(
                Ne,
                |t1, t2| match (*t1, *t2) {
                    (Int(i1), Int(i2)) => {
                        if i1 != i2 {
                            True
                        } else {
                            False
                        }
                    }
                    (other1, other2) => Ne(Box::new(other1), Box::new(other2)),
                },
                t1,
                t2,
            ),
            Lt(t1, t2) => step_op2(
                Lt,
                |t1, t2| match (*t1, *t2) {
                    (Int(i1), Int(i2)) => {
                        if i1 < i2 {
                            True
                        } else {
                            False
                        }
                    }
                    (other1, other2) => Lt(Box::new(other1), Box::new(other2)),
                },
                t1,
                t2,
            ),
            Le(t1, t2) => step_op2(
                Le,
                |t1, t2| match (*t1, *t2) {
                    (Int(i1), Int(i2)) => {
                        if i1 <= i2 {
                            True
                        } else {
                            False
                        }
                    }
                    (other1, other2) => Le(Box::new(other1), Box::new(other2)),
                },
                t1,
                t2,
            ),
            Gt(t1, t2) => step_op2(
                Gt,
                |t1, t2| match (*t1, *t2) {
                    (Int(i1), Int(i2)) => {
                        if i1 > i2 {
                            True
                        } else {
                            False
                        }
                    }
                    (other1, other2) => Gt(Box::new(other1), Box::new(other2)),
                },
                t1,
                t2,
            ),
            Ge(t1, t2) => step_op2(
                Ge,
                |t1, t2| match (*t1, *t2) {
                    (Int(i1), Int(i2)) => {
                        if i1 >= i2 {
                            True
                        } else {
                            False
                        }
                    }
                    (other1, other2) => Ge(Box::new(other1), Box::new(other2)),
                },
                t1,
                t2,
            ),

            // ============================Pair stuff============================

            Pair(t1, t2) => {
                if !t1.is_value() {
                    Pair(Box::new(t1.step()), t2)
                } else if !t2.is_value() {
                    Pair(t1, Box::new(t2.step()))
                } else {
                    panic!("attempted to step pair of values")
                }
            }
            Fst(t1) => {
                let inner = *t1;
                if !inner.is_value() {
                    Fst(Box::new(inner.step()))
                } else {
                    match inner {
                        Pair(v1, v2) if v1.is_value() && v2.is_value() => *v1,
                        Pair(_, _) => panic!("fst applied to pair with non-value elements"),
                        _ => panic!("fst applied to non-pair value"),
                    }
                }
            }
            Snd(t1) => {
                let inner = *t1;
                if !inner.is_value() {
                    Snd(Box::new(inner.step()))
                } else {
                    match inner {
                        Pair(v1, v2) if v1.is_value() && v2.is_value() => *v2,
                        Pair(_, _) => panic!("snd applied to pair with non-value elements"),
                        _ => panic!("snd applied to non-pair value"),
                    }
                }
            }

            // ============================List stuff============================

            Cons(h, t) => {
                if !h.is_value() {
                    Cons(Box::new(h.step()), t)
                } else if !t.is_value() {
                    Cons(h, Box::new(t.step()))
                } else {
                    panic!("attempted to step cons of values")
                }
            }

            LCase { t, nil_t, head_var, tail_var, cons_t } => {
                let t = *t;
                if !t.is_value() {
                    LCase {
                        t: Box::new(t.step()),
                        nil_t,
                        head_var,
                        tail_var,
                        cons_t,
                    }
                } else {
                    match t {
                        Nil(_) => *nil_t,
                        Cons(h, t) if h.is_value() && t.is_value() => {
                            cons_t.subst(&head_var, *h).subst(&tail_var, *t)
                        }
                        Cons(_, _) => panic!("lcase on cons with non-value parts"),
                        _ => panic!("lcase on non-list"),
                    }
                }
            }

            // ============================Sum stuff============================
            // ===== Inl Evaluation Rule =====
            Inl(t, ty) => {
                if !t.is_value() {
                    Inl(Box::new(t.step()), ty)
                } else {
                    panic!("attempted to step inl of value")
                }
            }

            // ===== Inr Evaluation Rule =====
            Inr(t, ty) => {
                if !t.is_value() {
                    Inr(Box::new(t.step()), ty)
                } else {
                    panic!("attempted to step inr of value")
                }
            }

            // ===== Case Evaluation Rule =====
            Case { t, inl_var, inl_t, inr_var, inr_t } => {
                let t = *t;
                if !t.is_value() {
                    Case {
                        t: Box::new(t.step()),
                        inl_var,
                        inl_t,
                        inr_var,
                        inr_t,
                    }
                } else {
                    match t {
                        Inl(v, _) if v.is_value() => inl_t.subst(&inl_var, *v),
                        Inr(v, _) if v.is_value() => inr_t.subst(&inr_var, *v),
                        Inl(_, _) | Inr(_, _) => panic!("case on non-value inl/inr"),
                        _ => panic!("case on non-sum value"),
                    }
                }
            }

            // ============================Fix stuff============================

            Fix(inner) => {
                match *inner {
                    // Fix1: Reduce the inner term first if it's not a value
                    ref t if !t.is_value() => Fix(Box::new(t.clone().step())),

                    // Fix2: fix (\x:T. t) => [x -> fix (\x:T. t)] t
                    Abs { var, ty, body } => {
                        let clone = Fix(Box::new(Abs {
                            var: var.clone(),
                            ty: ty.clone(),
                            body: body.clone(),
                        }));
                        body.subst(&var, clone)
                    }
                    _ => panic!("fix applied to non-abstraction value"),
                }
            }


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
