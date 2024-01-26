use super::*;

impl Expr {
    pub fn substitute(&self, old: &Expr, new: &Expr) -> Expr {
        if self.is_same(old) {
            new.clone()
        } else {
            match self.expression() {
                Expression::Bool(_) => self.clone(),
                Expression::Int(_) => self.clone(),
                Expression::Real(_) => self.clone(),
                //
                Expression::PrefixUnary(op, e) => op.new(e.substitute(old, new)).into(),
                Expression::Binary(l, op, r) => op
                    .new(l.substitute(old, new), r.substitute(old, new))
                    .into(),
                Expression::Nary(op, v) => op
                    .new(v.iter().map(|e| e.substitute(old, new)).collect())
                    .into(),
                //
                Expression::EnumerateElement(_) => self.clone(),
                Expression::Declaration(_) => self.clone(),
                Expression::Definition(_) => self.clone(),
                Expression::FunDec(_) => self.clone(),
                Expression::FunDef(_) => self.clone(),
                Expression::Parameter(_) => self.clone(),
                //
                Expression::Apply(f, p) => {
                    let f = f.substitute(old, new);
                    let p = p.iter().map(|e| e.substitute(old, new)).collect();
                    Expression::Apply(Box::new(f), p).into()
                }
                Expression::As(kid, typ, default) => {
                    let kid = kid.substitute(old, new);
                    let default = default.substitute(old, new);
                    Expression::As(Box::new(kid), typ.clone(), Box::new(default)).into()
                }
                Expression::Following(e) => {
                    Expression::Following(Box::new(e.substitute(old, new))).into()
                }
                Expression::State(kid, state, default) => {
                    let kid = kid.substitute(old, new);
                    let default = match default {
                        Some(default) => Some(Box::new(default.substitute(old, new))),
                        None => None,
                    };
                    Expression::State(Box::new(kid), *state, default).into()
                }
                Expression::Scope(l, e) => {
                    let l = l.iter().map(|e| e.substitute(old, new)).collect();
                    let e = e.substitute(old, new);
                    Expression::Scope(l, Box::new(e)).into()
                }
                //
                Expression::LTLunary(op, kid) => {
                    let kid = kid.substitute(old, new);
                    op.new(kid).into()
                }
                Expression::LTLbinary(left, op, right) => {
                    let left = left.substitute(old, new);
                    let right = right.substitute(old, new);
                    op.new(left, right).into()
                }
                //
                Expression::IfThenElse(ce, te, list, ee) => {
                    let ce = ce.substitute(old, new);
                    let te = te.substitute(old, new);
                    let list = list
                        .iter()
                        .map(|(c, e)| (c.substitute(old, new), e.substitute(old, new)))
                        .collect();
                    let ee = ee.substitute(old, new);
                    let expression =
                        Expression::IfThenElse(Box::new(ce), Box::new(te), list, Box::new(ee));
                    Expr::new(expression, None)
                }
                Expression::Quantifier(op, p, e) => {
                    let p = p.clone();
                    let e = e.substitute(old, new);
                    op.new(p, e).into()
                }
                //
                Expression::LTLVariable(_) => self.clone(),
                //
                Expression::Unresolved(_) => self.clone(),
            }
        }
    }

    pub fn substitute_all(&self, all: Vec<(Expr, Expr)>) -> Expr {
        let mut expr = self.clone();
        for (o, e) in all.iter() {
            expr = expr.substitute(o, e);
        }
        expr
    }
}
