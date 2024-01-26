use super::*;
use crate::common::*;
use crate::error::Error;
use crate::model::*;

impl Expr {
    //---------- Resolve ----------

    pub fn resolve(&self, model: &Model, entries: &Vec<Entry>) -> Result<Self, Error> {
        let expression = match self.expression() {
            e @ Expression::Bool(_) => e.clone(),
            e @ Expression::Int(_) => e.clone(),
            e @ Expression::Real(_) => e.clone(),
            //
            Expression::PrefixUnary(op, e) => {
                let e = e.resolve(model, entries)?;
                Expression::PrefixUnary(*op, Box::new(e))
            }
            Expression::Binary(left, op, right) => {
                let left = left.resolve(model, entries)?;
                let right = right.resolve(model, entries)?;
                Expression::Binary(Box::new(left), *op, Box::new(right))
            }
            Expression::Nary(op, kids) => {
                let mut v = Vec::new();
                for e in kids.iter() {
                    v.push(e.resolve(model, entries)?);
                }
                Expression::Nary(*op, v)
            }
            //
            e @ Expression::EnumerateElement(_) => e.clone(),
            e @ Expression::Declaration(_) => e.clone(),
            e @ Expression::Definition(_) => e.clone(),
            e @ Expression::FunDec(_) => e.clone(),
            e @ Expression::FunDef(_) => e.clone(),
            //
            Expression::Apply(fun, params) => {
                let f = fun.resolve(model, entries)?;
                let mut v = Vec::new();
                for p in params.iter() {
                    v.push(p.resolve(model, entries)?);
                }
                Expression::Apply(Box::new(f), v)
            }
            Expression::As(kid, typ, default) => {
                let kid = kid.resolve(model, entries)?;
                let default = default.resolve(model, entries)?;
                Expression::As(Box::new(kid), typ.clone(), Box::new(default))
            }
            //
            e @ Expression::Parameter(_) => e.clone(),
            //
            Expression::Following(kid) => {
                let kid = kid.resolve(model, entries)?;
                Expression::Following(Box::new(kid))
            }
            Expression::State(kid, state, default) => {
                let kid = kid.resolve(model, entries)?;
                let default = match default {
                    Some(default) => Some(Box::new(default.resolve(model, entries)?)),
                    None => None,
                };
                Expression::State(Box::new(kid), *state, default)
            }
            Expression::Scope(l, e) => {
                let mut v = Vec::new();
                for p in l.iter() {
                    v.push(p.resolve(model, entries)?);
                }
                let expr = e.resolve(model, entries)?;
                Expression::Scope(v, Box::new(expr))
            }
            //
            Expression::IfThenElse(ie, te, list, ee) => {
                let ie = ie.resolve(model, entries)?;
                let te = te.resolve(model, entries)?;
                let mut v = Vec::new();
                for (c, e) in list.iter() {
                    let c = c.resolve(model, entries)?;
                    let e = e.resolve(model, entries)?;
                    v.push((c, e));
                }
                let ee = ee.resolve(model, entries)?;
                Expression::IfThenElse(Box::new(ie), Box::new(te), v, Box::new(ee))
            }
            Expression::Quantifier(op, params, e) => {
                let mut entries = entries.clone();
                for p in params.iter() {
                    entries.push(p.into());
                }
                //
                let params = params.clone();
                let e = e.resolve(model, &entries)?;
                op.new(params, e)
            }
            //
            Expression::LTLunary(op, e) => {
                let e = e.resolve(model, entries)?;
                Expression::LTLunary(*op, Box::new(e))
            }
            Expression::LTLbinary(left, op, right) => {
                let left = left.resolve(model, entries)?;
                let right = right.resolve(model, entries)?;
                Expression::LTLbinary(Box::new(left), *op, Box::new(right))
            }
            e @ Expression::LTLVariable(_) => e.clone(),
            //
            Expression::Unresolved(name) => match &get_entry(name, entries) {
                Some(entry) => entry.into(),
                None => {
                    return Err(Error::Resolve {
                        category: "identifier".to_string(),
                        name: name.clone(),
                        position: self.position().clone(),
                    });
                }
            },
        };
        Ok(Self::new(expression, self.position().clone()))
    }
}
