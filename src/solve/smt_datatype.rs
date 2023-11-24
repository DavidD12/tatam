use super::*;
use crate::expr::*;

impl<'a> Smt<'a> {
    pub fn to_datatype(&self, expr: &Expr, state: usize) -> z3::ast::Datatype<'a> {
        match expr.expression() {
            Expression::EnumerateElement(id) => self.enum_elt(*id).clone(),
            Expression::Declaration(id) => self.enum_dec(*id, state).clone(),

            Expression::Following(e) => self.to_datatype(e, state + 1),
            Expression::State(e, state) => self.to_datatype(e, *state),

            Expression::IfThenElse(ce, te, list, ee) => {
                let c = self.to_bool(ce, state);
                let t = self.to_datatype(te, state);
                let l = list
                    .iter()
                    .map(|(ce, ee)| (self.to_bool(ce, state), self.to_datatype(ee, state)))
                    .collect::<Vec<_>>();
                let ee = self.to_datatype(ee, state);
                let mut res = ee;
                for (x, y) in l.iter().rev() {
                    res = x.ite(y, &res);
                }
                res = c.ite(&t, &res);
                res
            }

            Expression::Apply(_, _) => todo!(),

            Expression::Bool(_) => panic!(),
            Expression::Int(_) => panic!(),
            Expression::Real(_) => panic!(),
            Expression::PrefixUnary(_, _) => panic!(),
            Expression::Binary(_, _, _) => panic!(),
            Expression::Nary(_, _) => panic!(),
            Expression::Definition(_) => panic!(),
            Expression::FunDec(_) => panic!(),
            Expression::FunDef(_) => panic!(),
            Expression::Parameter(_) => panic!(),

            Expression::LTLunary(_, _) => panic!(),
            Expression::LTLbinary(_, _, _) => panic!(),
            Expression::LTLVariable(_) => panic!(),

            Expression::Unresolved(_) => panic!(),
        }
    }
}
