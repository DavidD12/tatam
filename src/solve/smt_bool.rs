use super::*;
use crate::expr::*;
use z3::ast::Ast;

impl<'a> Smt<'a> {
    pub fn to_bool(&self, expr: &Expr, state: usize) -> z3::ast::Bool<'a> {
        match expr.expression() {
            Expression::Bool(value) => z3::ast::Bool::from_bool(self.ctx(), *value),
            Expression::PrefixUnary(op, kid) => match op {
                PrefixUnaryOperator::Not => z3::ast::Bool::not(&self.to_bool(kid, state)),
                PrefixUnaryOperator::Neg => panic!(),
            },
            Expression::Binary(left, op, right) => {
                let t = left.get_type(self.model());
                if t.is_enumerate() {
                    let l = self.to_datatype(left, state);
                    let r = self.to_datatype(right, state);
                    match op {
                        BinaryOperator::Eq => l._eq(&r),
                        BinaryOperator::Ne => z3::ast::Bool::not(&l._eq(&r)),
                        BinaryOperator::Lt => panic!(),
                        BinaryOperator::Le => panic!(),
                        BinaryOperator::Ge => panic!(),
                        BinaryOperator::Gt => panic!(),
                        BinaryOperator::Implies => panic!(),
                    }
                } else if t.is_bool() {
                    let l = self.to_bool(left, state);
                    let r = self.to_bool(right, state);
                    match op {
                        BinaryOperator::Eq => l._eq(&r),
                        BinaryOperator::Ne => z3::ast::Bool::not(&l._eq(&r)),
                        BinaryOperator::Implies => z3::ast::Bool::implies(&l, &r),
                        BinaryOperator::Lt => panic!(),
                        BinaryOperator::Le => panic!(),
                        BinaryOperator::Ge => panic!(),
                        BinaryOperator::Gt => panic!(),
                    }
                } else if t.is_integer() {
                    let l = self.to_int(left, state);
                    let r = self.to_int(right, state);
                    match op {
                        BinaryOperator::Eq => l._eq(&r),
                        BinaryOperator::Ne => z3::ast::Bool::not(&l._eq(&r)),
                        BinaryOperator::Lt => l.lt(&r),
                        BinaryOperator::Le => l.le(&r),
                        BinaryOperator::Ge => l.ge(&r),
                        BinaryOperator::Gt => l.gt(&r),
                        BinaryOperator::Implies => panic!(),
                    }
                } else if t.is_real() {
                    let l = self.to_real(left, state);
                    let r = self.to_real(right, state);
                    match op {
                        BinaryOperator::Eq => l._eq(&r),
                        BinaryOperator::Ne => z3::ast::Bool::not(&l._eq(&r)),
                        BinaryOperator::Lt => l.lt(&r),
                        BinaryOperator::Le => l.le(&r),
                        BinaryOperator::Ge => l.ge(&r),
                        BinaryOperator::Gt => l.gt(&r),
                        BinaryOperator::Implies => panic!(),
                    }
                } else {
                    panic!()
                }
            }
            Expression::Nary(op, list) => {
                let v = list
                    .iter()
                    .map(|e| self.to_bool(e, state))
                    .collect::<Vec<_>>();
                let a = &v.iter().collect::<Vec<_>>();
                match op {
                    NaryOperator::And => z3::ast::Bool::and(self.ctx(), a),
                    NaryOperator::Or => z3::ast::Bool::or(self.ctx(), a),
                    NaryOperator::Add => panic!(),
                    NaryOperator::Sub => panic!(),
                    NaryOperator::Mul => panic!(),
                }
            }
            Expression::Declaration(id) => self.bool_dec(*id, state).clone(),
            Expression::Following(e) => self.to_bool(e, state + 1),
            Expression::State(e, state) => self.to_bool(e, *state),

            Expression::IfThenElse(ce, te, list, ee) => {
                let c = self.to_bool(ce, state);
                let t = self.to_bool(te, state);
                let l = list
                    .iter()
                    .map(|(ce, ee)| (self.to_bool(ce, state), self.to_bool(ee, state)))
                    .collect::<Vec<_>>();
                let ee = self.to_bool(ee, state);
                let mut res = ee;
                for (x, y) in l.iter().rev() {
                    res = x.ite(y, &res);
                }
                res = c.ite(&t, &res);
                res
            }

            Expression::LTLunary(_, _) => todo!(),
            Expression::LTLbinary(_, _, _) => todo!(),
            Expression::LTLVariable(id) => self.ltl_var(*id, state).clone(),

            Expression::Apply(_, _) => todo!(),

            Expression::Int(_) => panic!(),
            Expression::Real(_) => panic!(),
            Expression::EnumerateElement(_) => panic!(),
            Expression::Parameter(_) => panic!(),
            Expression::Definition(_) => panic!(),
            Expression::FunDec(_) => panic!(),
            Expression::FunDef(_) => panic!(),

            Expression::Unresolved(_) => panic!(),
        }
    }
}
