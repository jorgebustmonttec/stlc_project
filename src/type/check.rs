use super::Type::{self, *};
use super::{
    Context,
    TypeError::{self, *},
};
use crate::term::Term::{self, *};

impl Term {
    /// Infers the type of the term `self`.
    ///
    /// # Errors
    ///
    /// - If the context doesn't contain the required variable, returns an [`UndefinedVariable`] error with the name of the variable inside.
    /// - If the left hand side of an application is not an arrow type, returns the [`WrongAppTypeLeft`] error with the actual type inside.
    /// - If the right hand side of an application is not of the expected type, returns the [`WrongAppTypeRight`] error with the actual type inside.
    /// - In other typing failures, the error [`Fail`] is returned.
    pub fn infer_type(&self, mut ctx: Context) -> Result<Type, TypeError> {
        match self {
            Var(x) => ctx.get(x).cloned().ok_or(UndefinedVariable(x.clone())),
            Abs { var, ty, body } => {
                ctx.insert(var.clone(), ty.clone());
                Ok(Arrow(
                    Box::new(ty.clone()),
                    Box::new(body.as_ref().infer_type(ctx)?),
                ))
            }
            App(term, term1) => {
                let ty1 = term.infer_type(ctx.clone())?;
                if let Arrow(dom, cod) = ty1 {
                    let ty2 = term1.infer_type(ctx)?;
                    if ty2 == *dom {
                        return Ok(*(cod));
                    } else {
                        return Err(WrongAppTypeRight(ty2));
                    }
                }
                Err(WrongAppTypeLeft(ty1))
            }

            Let { var, val_t, body } => {
                ctx.insert(var.clone(), val_t.infer_type(ctx.clone())?);
                body.infer_type(ctx)
            }

            True | False => Ok(Boolean),
            Ite {
                cond,
                if_true,
                if_false,
            } => {
                if let Boolean = cond.infer_type(ctx.clone())? {
                    let ty1 = if_true.infer_type(ctx.clone())?;
                    let ty2 = if_false.infer_type(ctx.clone())?;
                    if ty1 == ty2 {
                        return Ok(ty1);
                    }
                }
                Err(Fail)
            }
            Int(_) => Ok(Integer),
            Add(term1, term2) => {
                let ty1 = term1.infer_type(ctx.clone())?;
                let ty2 = term2.infer_type(ctx.clone())?;
                if ty1 == Integer && ty2 == Integer {
                    return Ok(Integer);
                }
                Err(Fail)
            }
            Sub(term1, term2) => {
                let ty1 = term1.infer_type(ctx.clone())?;
                let ty2 = term2.infer_type(ctx.clone())?;
                if ty1 == Integer && ty2 == Integer {
                    return Ok(Integer);
                }
                Err(Fail)
            }
            Mul(term1, term2) => {
                let ty1 = term1.infer_type(ctx.clone())?;
                let ty2 = term2.infer_type(ctx.clone())?;
                if ty1 == Integer && ty2 == Integer {
                    return Ok(Integer);
                }
                Err(Fail)
            }
            Eq(term1, term2) => {
                let ty1 = term1.infer_type(ctx.clone())?;
                let ty2 = term2.infer_type(ctx.clone())?;
                if ty1 == ty2 {
                    return Ok(Boolean);
                }
                Err(Fail)
            }
            Ne(term1, term2) => {
                let ty1 = term1.infer_type(ctx.clone())?;
                let ty2 = term2.infer_type(ctx.clone())?;
                if ty1 == ty2 {
                    return Ok(Boolean);
                }
                Err(Fail)
            }
            Lt(term1, term2) => {
                let ty1 = term1.infer_type(ctx.clone())?;
                let ty2 = term2.infer_type(ctx.clone())?;
                if ty1 == Integer && ty2 == Integer {
                    return Ok(Boolean);
                }
                Err(Fail)
            }
            Le(term1, term2) => {
                let ty1 = term1.infer_type(ctx.clone())?;
                let ty2 = term2.infer_type(ctx.clone())?;
                if ty1 == Integer && ty2 == Integer {
                    return Ok(Boolean);
                }
                Err(Fail)
            }
            Gt(term1, term2) => {
                let ty1 = term1.infer_type(ctx.clone())?;
                let ty2 = term2.infer_type(ctx.clone())?;
                if ty1 == Integer && ty2 == Integer {
                    return Ok(Boolean);
                }
                Err(Fail)
            }
            Ge(term1, term2) => {
                let ty1 = term1.infer_type(ctx.clone())?;
                let ty2 = term2.infer_type(ctx.clone())?;
                if ty1 == Integer && ty2 == Integer {
                    return Ok(Boolean);
                }
                Err(Fail)
            }
            Pair(t1, t2) => {
                let ty1 = t1.infer_type(ctx.clone())?;
                let ty2 = t2.infer_type(ctx)?;
                Ok(Prod(Box::new(ty1), Box::new(ty2)))
            }
            Fst(t) => {
                match t.infer_type(ctx)? {
                    Prod(ty1, _) => Ok(*ty1),
                    _ => Err(Fail),
                }
            }
            Snd(t) => {
                match t.infer_type(ctx)? {
                    Prod(_, ty2) => Ok(*ty2),
                    _ => Err(Fail),
                }
            }
            

            _ => todo!(),
        }
    }

    pub fn type_check(&self) -> Result<Type, TypeError> {
        self.infer_type(Context::new())
    }
}
