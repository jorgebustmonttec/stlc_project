use super::Term::{self, *};

impl Term {
    /// Performs substitution of a variable `x` with a given term `v`.
    ///
    /// # Incomplete Terms
    ///
    /// If `v` contains free variables, the function's behavior remains well-defined
    /// but the correctness of the result is not guaranteed (i.e. this function assumes that `v`
    /// is complete), though it must not panic even if it is not.
    ///
    /// # Examples
    ///
    /// **Variable substitution:**
    /// ```rust
    /// # use application::term::util::*;
    /// assert_eq!(var("x").subst("x", var("y")), var("y"));
    /// ```
    ///
    /// **Inside an abstraction (bound variable remains unchanged):**
    /// ```rust
    /// # use application::term::util::*;
    /// assert_eq!(abs("x", Boolean, var("x")).subst("x", var("y")), abs("x", Boolean, var("x")));
    /// ```
    ///
    /// **Using a let expression:**
    /// In the let expression below, the bound variable is `"y"`. Substituting `"x"` will
    /// affect both the value part and the body.
    /// ```rust
    /// # use application::term::util::*;
    /// let let_expr = letin("y", var("x"), app(var("x"), var("y")));
    /// let expected  = letin("y", var("z"), app(var("z"), var("y")));
    /// assert_eq!(let_expr.subst("x", var("z")), expected);
    /// ```
    ///
    /// Substituting inside the body of a let should only substitute if the var is different.
    /// - `[x ↦ id](let x = id in x) = let x = id in x`, i.e. should be invariant, as `x` is bound by the let.
    /// - `[x ↦ id](let x = x in x) = let x = id in x`, i.e. `val_t` is substituted, as it's not quantified by the let.
    ///
    /// ```rust
    /// # use application::term::util::*;
    /// assert_eq!(
    ///     letin("x", id2(), var("x")).subst("x", id2()),
    ///     letin("x", id2(), var("x"))
    /// );
    /// assert_eq!(
    ///     letin("x", var("x"), var("x")).subst("x", id2()),
    ///     letin("x", id2(), var("x"))
    /// );
    /// ```
    pub fn subst(self, x: &str, v: Self) -> Self {
        match self {
            Var(y) if y == x => v,
            Abs { var, ty, body } if var != x => Abs {
                var,
                ty,
                body: Box::new(body.subst(x, v)),
            },
            App(t1, t2) => App(Box::new(t1.subst(x, v.clone())), Box::new(t2.subst(x, v))),

            Let { var, val_t, body } => Let {
                body: if &var != x {
                    Box::new(body.subst(x, v.clone()))
                } else {
                    body
                },
                var,
                val_t: Box::new(val_t.subst(x, v)),
            },

            Ite {
                cond,
                if_true,
                if_false,
            } => Ite {
                cond: Box::new(cond.subst(x, v.clone())),
                if_true: Box::new(if_true.subst(x, v.clone())),
                if_false: Box::new(if_false.subst(x, v)),
            },

            Add(t1, t2) => todo!(),
            Sub(t1, t2) => todo!(),
            Mul(t1, t2) => todo!(),

            Eq(t1, t2) => todo!(),
            Ne(t1, t2) => todo!(),
            Lt(t1, t2) => todo!(),
            Le(t1, t2) => todo!(),
            Gt(t1, t2) => todo!(),
            Ge(t1, t2) => todo!(),
            _ => self,
        }
    }
}
