use super::*;
use crate::expr::*;

impl<'a> Smt<'a> {
    pub fn to_real(&self, expr: &Expr, state: usize) -> z3::ast::Real<'a> {
        match expr.expression() {
            Expression::Real(value) => z3::ast::Real::from_real(
                self.ctx(),
                *value.numer().unwrap() as i32,
                *value.denom().unwrap() as i32,
            ),
            Expression::PrefixUnary(op, kid) => match op {
                PrefixUnaryOperator::Not => panic!(),
                PrefixUnaryOperator::Neg => {
                    let e = self.to_real(&kid, state);
                    e.unary_minus()
                }
            },
            Expression::Nary(op, list) => {
                let v = list
                    .iter()
                    .map(|e| self.to_real(e, state))
                    .collect::<Vec<_>>();
                let a = &v.iter().collect::<Vec<_>>();
                match op {
                    NaryOperator::Add => z3::ast::Real::add(self.ctx(), a),
                    NaryOperator::Sub => z3::ast::Real::sub(self.ctx(), a),
                    NaryOperator::Mul => z3::ast::Real::mul(self.ctx(), a),
                    NaryOperator::And => panic!(),
                    NaryOperator::Or => panic!(),
                }
            }
            Expression::Declaration(id) => self.real_dec(*id, state).clone(),
            Expression::Following(e) => self.to_real(e, state + 1),
            Expression::State(e, state) => self.to_real(e, *state),

            Expression::IfThenElse(ce, te, list, ee) => {
                let c = self.to_bool(ce, state);
                let t = self.to_real(te, state);
                let l = list
                    .iter()
                    .map(|(ce, ee)| (self.to_bool(ce, state), self.to_real(ee, state)))
                    .collect::<Vec<_>>();
                let ee = self.to_real(ee, state);
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
