use super::*;
use crate::expr::*;

impl<'a> Smt<'a> {
    pub fn to_int(&self, expr: &Expr, state: usize) -> z3::ast::Int<'a> {
        match expr.expression() {
            Expression::Int(value) => z3::ast::Int::from_i64(self.ctx(), *value as i64),
            Expression::PrefixUnary(op, kid) => match op {
                PrefixUnaryOperator::Not => panic!(),
                PrefixUnaryOperator::Neg => {
                    let e = self.to_int(&kid, state);
                    e.unary_minus()
                }
            },
            Expression::Nary(op, list) => {
                let v = list
                    .iter()
                    .map(|e| self.to_int(e, state))
                    .collect::<Vec<_>>();
                let a = &v.iter().collect::<Vec<_>>();
                match op {
                    NaryOperator::Add => z3::ast::Int::add(self.ctx(), a),
                    NaryOperator::Sub => z3::ast::Int::sub(self.ctx(), a),
                    NaryOperator::Mul => z3::ast::Int::mul(self.ctx(), a),
                    NaryOperator::And => panic!(),
                    NaryOperator::Or => panic!(),
                }
            }
            Expression::Declaration(id) => self.int_dec(*id, state).clone(),
            Expression::Following(e) => self.to_int(e, state + 1),
            Expression::State(e, state) => self.to_int(e, *state),

            Expression::IfThenElse(ce, te, list, ee) => {
                let c = self.to_bool(ce, state);
                let t = self.to_int(te, state);
                let l = list
                    .iter()
                    .map(|(ce, ee)| (self.to_bool(ce, state), self.to_int(ee, state)))
                    .collect::<Vec<_>>();
                let ee = self.to_int(ee, state);
                let mut res = ee;
                for (x, y) in l.iter().rev() {
                    res = x.ite(y, &res);
                }
                res = c.ite(&t, &res);
                res
            }

            Expression::Apply(_, _) => todo!(),

            Expression::Bool(_) => panic!(),
            Expression::Real(_) => panic!(),
            Expression::EnumerateElement(_) => panic!(),
            Expression::Definition(_) => panic!(),
            Expression::Binary(_, _, _) => panic!(),
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
