use super::*;
use crate::common::*;
use crate::error::Error;
use crate::typing::*;
use std::collections::HashMap;

impl Expr {
    pub fn resolve_type(&self, types: &HashMap<String, Type>) -> Result<Expr, Error> {
        let expression = match self.expression() {
            e @ Expression::Bool(_) => e.clone(),
            e @ Expression::Int(_) => e.clone(),
            e @ Expression::Real(_) => e.clone(),
            //
            Expression::PrefixUnary(op, e) => {
                let e = e.resolve_type(types)?;
                op.new(e)
            }
            Expression::Binary(l, op, r) => {
                let l = l.resolve_type(types)?;
                let r = r.resolve_type(types)?;
                op.new(l, r)
            }
            Expression::Nary(op, list) => {
                let mut l = vec![];
                for e in list.iter() {
                    l.push(e.resolve_type(types)?);
                }
                op.new(l)
            }
            //
            e @ Expression::EnumerateElement(_) => e.clone(),
            e @ Expression::Declaration(_) => e.clone(),
            e @ Expression::Definition(_) => e.clone(),
            e @ Expression::FunDec(_) => e.clone(),
            e @ Expression::FunDef(_) => e.clone(),
            //
            Expression::Parameter(param) => {
                let mut param = param.clone();
                param.resolve_type(types)?;
                Expression::Parameter(param)
            }
            //
            Expression::Apply(e, params) => {
                let e = e.resolve_type(types)?;
                let mut l = vec![];
                for p in params.iter() {
                    l.push(p.resolve_type(types)?);
                }
                Expression::Apply(Box::new(e), l)
            }
            Expression::As(kid, typ, default) => {
                let kid = kid.resolve_type(types)?;
                let typ = typ.resolve(types)?;
                let default = default.resolve_type(types)?;
                Expression::As(Box::new(kid), typ, Box::new(default))
            }
            Expression::Following(e) => {
                let e = e.resolve_type(types)?;
                Expression::Following(Box::new(e))
            }
            Expression::State(e, state) => {
                let e = e.resolve_type(types)?;
                Expression::State(Box::new(e), *state)
            }
            Expression::Scope(l, e) => {
                let mut v = vec![];
                for p in l.iter() {
                    v.push(p.resolve_type(types)?);
                }
                let e = e.resolve_type(types)?;
                Expression::Scope(v, Box::new(e))
            }
            Expression::IfThenElse(c, t, ei, e) => {
                let c = c.resolve_type(types)?;
                let t = t.resolve_type(types)?;
                let mut l = vec![];
                for (c, e) in ei.iter() {
                    let c = c.resolve_type(types)?;
                    let e = e.resolve_type(types)?;
                    l.push((c, e));
                }
                let e = e.resolve_type(types)?;
                Expression::IfThenElse(Box::new(c), Box::new(t), l, Box::new(e))
            }
            Expression::Quantifier(op, params, e) => {
                let mut params = params.clone();
                for p in params.iter_mut() {
                    p.resolve_type(types)?;
                }
                let e = e.resolve_type(types)?;
                op.new(params, e)
            }
            Expression::LTLunary(op, e) => {
                let e = e.resolve_type(types)?;
                op.new(e)
            }
            Expression::LTLbinary(left, op, right) => {
                let left = left.resolve_type(types)?;
                let right = right.resolve_type(types)?;
                op.new(left, right)
            }
            e @ Expression::LTLVariable(_) => e.clone(),
            e @ Expression::Unresolved(_) => e.clone(),
        };
        Ok(Expr::new(expression, self.position().clone()))
    }
}
