use super::*;
use crate::{model::*, WithPosition};

impl Expr {
    pub fn flatten_ltl(&self, model: &mut Model) -> Expr {
        match self.expression() {
            Expression::Bool(_) => self.clone(),
            Expression::Int(_) => self.clone(),
            Expression::Real(_) => self.clone(),
            //
            Expression::PrefixUnary(op, kid) => op.new(kid.flatten_ltl(model)).into(),
            Expression::Binary(left, op, right) => op
                .new(left.flatten_ltl(model), right.flatten_ltl(model))
                .into(),
            Expression::Nary(op, list) => {
                let list = list.iter().map(|x| x.flatten_ltl(model)).collect();
                op.new(list).into()
            }
            //
            Expression::EnumerateElement(_) => self.clone(),
            Expression::Declaration(_) => self.clone(),
            Expression::Definition(_) => self.clone(),
            Expression::FunDec(_) => self.clone(),
            Expression::FunDef(_) => self.clone(),
            Expression::Parameter(_) => self.clone(),
            Expression::LtlDefinition(_) => self.clone(),
            //
            Expression::Apply(fun, params) => {
                let fun = fun.flatten_ltl(model);
                let params = params.iter().map(|x| x.flatten_ltl(model)).collect();
                let expression = Expression::Apply(Box::new(fun), params);
                Expr::new(expression, self.position().clone())
            }
            Expression::As(kid, typ, default) => {
                let kid = kid.flatten_ltl(model);
                let default = default.flatten_ltl(model);
                let expression = Expression::As(Box::new(kid), typ.clone(), Box::new(default));
                Expr::new(expression, self.position().clone())
            }
            Expression::Following(kid) => {
                let kid = kid.flatten_ltl(model);
                let expression = Expression::Following(Box::new(kid));
                Expr::new(expression, self.position().clone())
            }
            Expression::State(kid, state, default) => {
                let kid = kid.flatten_ltl(model);
                let default = match default {
                    Some(default) => Some(Box::new(default.flatten_ltl(model))),
                    None => None,
                };
                let expression = Expression::State(Box::new(kid), *state, default);
                Expr::new(expression, self.position().clone())
            }
            Expression::Scope(l, e) => {
                let list = l.iter().map(|x| x.flatten_ltl(model)).collect();
                let expr = e.flatten_ltl(model);
                let expression = Expression::Scope(list, Box::new(expr));
                Expr::new(expression, self.position().clone())
            }
            //
            Expression::IfThenElse(ce, te, list, ee) => {
                let ce = ce.flatten_ltl(model);
                let te = te.flatten_ltl(model);
                let list = list
                    .iter()
                    .map(|(c, e)| (c.flatten_ltl(model), e.flatten_ltl(model)))
                    .collect();
                let ee = ee.flatten_ltl(model);
                let expression =
                    Expression::IfThenElse(Box::new(ce), Box::new(te), list, Box::new(ee));
                Expr::new(expression, self.position().clone())
            }
            Expression::Quantifier(op, params, e) => {
                let e = e.flatten_ltl(model);
                let expression = op.new(params.clone(), e);
                Expr::new(expression, self.position().clone())
            }
            //
            Expression::LTLunary(op, kid) => {
                let kid = kid.flatten_ltl(model);
                let e = op.new(kid);
                let id = model.insert_ltl_variable(e.into());
                id.into()
            }
            Expression::LTLbinary(left, op, right) => {
                let left = left.flatten_ltl(model);
                let right = right.flatten_ltl(model);
                let e = op.new(left, right);
                let id = model.insert_ltl_variable(e.into());
                id.into()
            }
            Expression::LTLVariable(_) => self.clone(),
            //
            Expression::Unresolved(_) => self.clone(),
        }
    }
}
