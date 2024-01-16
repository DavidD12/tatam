use super::*;
use crate::error::*;
use crate::model::*;
use crate::*;

impl Expr {
    pub fn check_time(&self, model: &Model) -> Result<(), Error> {
        match self.expression() {
            Expression::Bool(_) => Ok(()),
            Expression::Int(_) => Ok(()),
            Expression::Real(_) => Ok(()),
            //
            Expression::PrefixUnary(_, kid) => kid.check_time(model),
            Expression::Binary(left, _, right) => {
                left.check_time(model)?;
                right.check_time(model)
            }
            Expression::Nary(_, kids) => {
                for x in kids.iter() {
                    x.check_time(model)?;
                }
                Ok(())
            }
            //
            Expression::EnumerateElement(_) => Ok(()),
            Expression::Declaration(_) => Ok(()),
            Expression::Definition(_) => Ok(()),
            Expression::FunDec(_) => Ok(()),
            Expression::FunDef(_) => Ok(()),
            //
            Expression::Parameter(_) => Ok(()),
            //
            Expression::Apply(fun, params) => {
                fun.check_time(model)?;
                for p in params.iter() {
                    p.check_time(model)?;
                }
                if let Some(expr) = fun.get_following() {
                    let message = "Following not allowed in 'Function'".into();
                    let name = self.to_lang(model);
                    let position = self.position().clone();
                    let expr = expr.clone();
                    return Err(Error::Time {
                        message,
                        name,
                        position,
                        expr,
                    });
                }
                Ok(())
            }
            Expression::As(kid, _, default) => {
                kid.check_time(model)?;
                default.check_time(model)
            }
            //
            Expression::Following(kid) => {
                if let Some(expr) = kid.get_following() {
                    let message = "Following not allowed in 'Following'".into();
                    let name = self.to_lang(model);
                    let position = self.position().clone();
                    let expr = expr.clone();
                    Err(Error::Time {
                        message,
                        name,
                        position,
                        expr,
                    })
                } else {
                    Ok(())
                }
            }
            Expression::State(_, _) => {
                let message = "State expression not allowed".into();
                let name = self.to_lang(model);
                let position = self.position().clone();
                let expr = self.clone();
                Err(Error::Time {
                    message,
                    name,
                    position,
                    expr,
                })
            }
            Expression::Scope(l, e) => {
                for p in l.iter() {
                    p.check_time(model)?;
                }
                e.check_time(model)
            }
            //
            Expression::IfThenElse(ce, te, list, ee) => {
                ce.check_time(model)?;
                te.check_time(model)?;
                for (c, e) in list.iter() {
                    c.check_time(model)?;
                    e.check_time(model)?;
                }
                ee.check_time(model)?;
                Ok(())
            }
            Expression::Quantifier(_, _, e) => e.check_time(model),
            //
            Expression::LTLunary(_, kid) => {
                if let Some(expr) = kid.get_following() {
                    let message = "Following not allowed in LTL formula".into();
                    let name = self.to_lang(model);
                    let position = self.position().clone();
                    let expr = expr.clone();
                    Err(Error::Time {
                        message,
                        name,
                        position,
                        expr,
                    })
                } else {
                    Ok(())
                }
            }
            Expression::LTLbinary(left, _, right) => {
                if let Some(expr) = left.get_following().or(right.get_following()) {
                    let message = "Following not allowed in LTL formula".into();
                    let name = self.to_lang(model);
                    let position = self.position().clone();
                    let expr = expr.clone();
                    Err(Error::Time {
                        message,
                        name,
                        position,
                        expr,
                    })
                } else {
                    Ok(())
                }
            } //
            Expression::LTLVariable(_) => Ok(()),
            Expression::Unresolved(_) => Ok(()),
        }
    }

    pub fn get_following<'a>(&'a self) -> Option<&'a Expr> {
        match self.expression() {
            Expression::Bool(_) => None,
            Expression::Int(_) => None,
            Expression::Real(_) => None,
            //
            Expression::PrefixUnary(_, kid) => kid.get_following(),
            Expression::Binary(left, _, right) => left.get_following().or(right.get_following()),
            Expression::Nary(_, kids) => {
                for x in kids.iter() {
                    if let Some(e) = x.get_following() {
                        return Some(e);
                    }
                }
                None
            }
            //
            Expression::EnumerateElement(_) => None,
            Expression::Declaration(_) => None,
            Expression::Definition(_) => None,
            Expression::FunDec(_) => None,
            Expression::FunDef(_) => None,
            Expression::Parameter(_) => None,
            //
            Expression::Apply(fun, params) => fun
                .get_following()
                .or(params.iter().find_map(|p| p.get_following())),
            Expression::As(kid, _, default) => kid.get_following().or(default.get_following()),
            //
            Expression::Following(_) => Some(self),
            Expression::State(kid, _) => kid.get_following(),
            Expression::Scope(_, e) => e.get_following(),
            //
            Expression::IfThenElse(ce, te, list, ee) => ce
                .get_following()
                .or(te.get_following())
                .or(list
                    .iter()
                    .find_map(|(c, e)| c.get_following().or(e.get_following())))
                .or(ee.get_following()),
            Expression::Quantifier(_, _, e) => e.get_following(),
            //
            Expression::LTLunary(_, kid) => kid.get_following(),
            Expression::LTLbinary(left, _, right) => left.get_following().or(right.get_following()),
            Expression::LTLVariable(_) => None,
            Expression::Unresolved(_) => None,
        }
    }

    pub fn get_ltl<'a>(&'a self) -> Option<&'a Expr> {
        match self.expression() {
            Expression::Bool(_) => None,
            Expression::Int(_) => None,
            Expression::Real(_) => None,

            Expression::PrefixUnary(_, e) => e.get_ltl(),
            Expression::Binary(l, _, r) => l.get_ltl().or(r.get_ltl()),
            Expression::Nary(_, kids) => {
                for x in kids.iter() {
                    if let Some(e) = x.get_ltl() {
                        return Some(e);
                    }
                }
                None
            }
            //
            Expression::EnumerateElement(_) => None,
            Expression::Declaration(_) => None,
            Expression::Definition(_) => None,
            Expression::FunDec(_) => None,
            Expression::FunDef(_) => None,
            Expression::Parameter(_) => None,
            //
            Expression::Apply(fun, params) => {
                fun.get_ltl().or(params.iter().find_map(|p| p.get_ltl()))
            }
            Expression::As(kid, _, default) => kid.get_ltl().or(default.get_ltl()),
            Expression::Following(e) => e.get_ltl(),
            Expression::State(e, _) => e.get_ltl(),
            Expression::Scope(_, e) => e.get_ltl(),
            Expression::IfThenElse(ce, te, list, ee) => ce
                .get_ltl()
                .or(te.get_ltl())
                .or(list.iter().find_map(|(c, e)| c.get_ltl().or(e.get_ltl())))
                .or(ee.get_ltl()),
            Expression::Quantifier(_, _, e) => e.get_ltl(),
            Expression::LTLunary(_, _) => Some(self),
            Expression::LTLbinary(_, _, _) => Some(self),
            Expression::LTLVariable(_) => None,
            Expression::Unresolved(_) => None,
        }
    }
}
