use super::*;

impl Expr {
    pub fn is_same(&self, other: &Expr) -> bool {
        match (self.expression(), other.expression()) {
            (Expression::Bool(x), Expression::Bool(y)) => x == y,
            (Expression::Int(x), Expression::Int(y)) => x == y,
            (Expression::Real(x), Expression::Real(y)) => x == y,
            //
            (Expression::PrefixUnary(o1, k1), Expression::PrefixUnary(o2, k2)) => {
                (o1 == o2) && k1.is_same(k2)
            }
            (Expression::Binary(l1, o1, r1), Expression::Binary(l2, o2, r2)) => {
                o1 == o2 && l1.is_same(l2) && r1.is_same(r2)
            }
            (Expression::Nary(o1, v1), Expression::Nary(o2, v2)) => {
                o1 == o2 && Expr::all_same(v1, v2)
            }
            //
            (Expression::EnumerateElement(i1), Expression::EnumerateElement(i2)) => i1 == i2,
            (Expression::Declaration(i1), Expression::Declaration(i2)) => i1 == i2,
            (Expression::Definition(i1), Expression::Definition(i2)) => i1 == i2,
            (Expression::FunDec(i1), Expression::FunDec(i2)) => i1 == i2,
            (Expression::FunDef(i1), Expression::FunDef(i2)) => i1 == i2,
            //
            (Expression::Parameter(p1), Expression::Parameter(p2)) => p1.name() == p2.name(),
            (Expression::Apply(f1, p1), Expression::Apply(f2, p2)) => {
                f1.is_same(f2) && Expr::all_same(p1, p2)
            }
            //
            (Expression::As(k1, t1, d1), Expression::As(k2, t2, d2)) => {
                k1.is_same(k2) && t1 == t2 && d1.is_same(d2)
            }
            //
            (Expression::Following(k1), Expression::Following(k2)) => k1.is_same(k2),
            (Expression::State(k1, s1), Expression::State(k2, s2)) => s1 == s2 && k1.is_same(k2),
            (Expression::Scope(l1, e1), Expression::Scope(l2, e2)) => {
                Expr::all_same(l1, l2) && e1.is_same(e2)
            }
            //
            (Expression::IfThenElse(c1, t1, l1, e1), Expression::IfThenElse(c2, t2, l2, e2)) => {
                c1.is_same(c2)
                    && t1.is_same(t2)
                    && l1.len() == l2.len()
                    && l1
                        .iter()
                        .zip(l2.iter())
                        .all(|((x1, y1), (x2, y2))| x1.is_same(x2) && y1.is_same(y2))
                    && e1.is_same(e2)
            }
            (Expression::Quantifier(op1, p1, e1), Expression::Quantifier(op2, p2, e2)) => {
                op1 == op2
                    && p1.len() == p2.len()
                    && p1.iter().zip(p2.iter()).all(|(x1, x2)| x1.is_same(x2))
                    && e1.is_same(e2)
            }
            //
            (Expression::LTLVariable(i1), Expression::LTLVariable(i2)) => i1 == i2,
            (Expression::LTLunary(o1, k1), Expression::LTLunary(o2, k2)) => {
                o1 == o2 && k1.is_same(k2)
            }
            (Expression::LTLbinary(l1, o1, r1), Expression::LTLbinary(l2, o2, r2)) => {
                o1 == o2 && l1.is_same(l2) && r1.is_same(r2)
            }
            //
            _ => false,
        }
    }

    pub fn all_same(v1: &Vec<Expr>, v2: &Vec<Expr>) -> bool {
        v1.iter().zip(v2.iter()).all(|(x, y)| x.is_same(y))
    }
}
