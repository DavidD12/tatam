use super::*;
use crate::common::*;
use crate::model::*;
use crate::parser::Position;
use fraction::Fraction;

//-------------------- Enumerate Element --------------------
impl From<&EnumerateElement> for Expr {
    fn from(value: &EnumerateElement) -> Self {
        Expr::new(Expression::EnumerateElement(value.id()), None)
    }
}
impl From<(&EnumerateElement, Position)> for Expr {
    fn from(value: (&EnumerateElement, Position)) -> Self {
        let (elt, position) = value;
        Expr::new(Expression::EnumerateElement(elt.id()), Some(position))
    }
}

//-------------------- Bool Value --------------------
impl From<bool> for Expr {
    fn from(value: bool) -> Self {
        Expr::new(Expression::Bool(value), None)
    }
}
impl From<(bool, Position)> for Expr {
    fn from(tuple: (bool, Position)) -> Self {
        let (value, position) = tuple;
        Expr::new(Expression::Bool(value), Some(position))
    }
}

//-------------------- Int Value --------------------
impl From<i64> for Expr {
    fn from(value: i64) -> Self {
        Expr::new(Expression::Int(value), None)
    }
}
impl From<(i64, Position)> for Expr {
    fn from(tuple: (i64, Position)) -> Self {
        let (value, position) = tuple;
        Expr::new(Expression::Int(value), Some(position))
    }
}

//-------------------- Real Value --------------------
impl From<Fraction> for Expr {
    fn from(value: Fraction) -> Self {
        Expr::new(Expression::Real(value), None)
    }
}
impl From<(Fraction, Position)> for Expr {
    fn from(tuple: (Fraction, Position)) -> Self {
        let (value, position) = tuple;
        Expr::new(Expression::Real(value), Some(position))
    }
}

impl From<(i64, i64)> for Expr {
    fn from(value: (i64, i64)) -> Self {
        let (mut num, mut den) = value;
        if den < 0 {
            num = -num;
            den = -den;
        }
        let fraction = if num >= 0 {
            Fraction::new_generic(fraction::Sign::Plus, num, den).unwrap()
        } else {
            Fraction::new_generic(fraction::Sign::Minus, -num, den).unwrap()
        };
        Expr::new(Expression::Real(fraction), None)
    }
}

impl From<(i64, i64, Position)> for Expr {
    fn from(value: (i64, i64, Position)) -> Self {
        let (mut num, mut den, position) = value;
        if den < 0 {
            num = -num;
            den = -den;
        }
        let fraction = if num >= 0 {
            Fraction::new_generic(fraction::Sign::Plus, num, den).unwrap()
        } else {
            Fraction::new_generic(fraction::Sign::Minus, -num, den).unwrap()
        };
        Expr::new(Expression::Real(fraction), Some(position))
    }
}

//-------------------- Declaration --------------------
impl From<DeclarationId> for Expr {
    fn from(value: DeclarationId) -> Self {
        Expr::new(Expression::Declaration(value), None)
    }
}
impl From<(DeclarationId, Position)> for Expr {
    fn from(tuple: (DeclarationId, Position)) -> Self {
        let (value, position) = tuple;
        Expr::new(Expression::Declaration(value), Some(position))
    }
}

//-------------------- FunDec --------------------
impl From<FunDecId> for Expr {
    fn from(value: FunDecId) -> Self {
        Expr::new(Expression::FunDec(value), None)
    }
}
impl From<(FunDecId, Position)> for Expr {
    fn from(tuple: (FunDecId, Position)) -> Self {
        let (value, position) = tuple;
        Expr::new(Expression::FunDec(value), Some(position))
    }
}

//-------------------- Parameter --------------------
impl From<Parameter> for Expr {
    fn from(value: Parameter) -> Self {
        let expression = Expression::Parameter(value);
        Expr::new(expression, None)
    }
}

//-------------------- LTL Variable --------------------
impl From<LTLVariableId> for Expr {
    fn from(value: LTLVariableId) -> Self {
        Expr::new(Expression::LTLVariable(value), None)
    }
}
impl From<(LTLVariableId, Position)> for Expr {
    fn from(tuple: (LTLVariableId, Position)) -> Self {
        let (value, position) = tuple;
        Expr::new(Expression::LTLVariable(value), Some(position))
    }
}

//-------------------- Prefix Unary --------------------
impl std::ops::Not for Expr {
    type Output = Expr;

    fn not(self) -> Self::Output {
        let e = Box::new(self);
        let expression = Expression::PrefixUnary(PrefixUnaryOperator::Not, e);
        Expr::new(expression, None)
    }
}
impl std::ops::Neg for Expr {
    type Output = Expr;

    fn neg(self) -> Self::Output {
        let e = Box::new(self);
        let expression = Expression::PrefixUnary(PrefixUnaryOperator::Neg, e);
        Expr::new(expression, None)
    }
}

impl Expr {
    pub fn not(self) -> Expr {
        let e = Box::new(self);
        let expression = Expression::PrefixUnary(PrefixUnaryOperator::Not, e);
        Expr::new(expression, None)
    }
    pub fn neg(self) -> Expr {
        let e = Box::new(self);
        let expression = Expression::PrefixUnary(PrefixUnaryOperator::Neg, e);
        Expr::new(expression, None)
    }
}

//-------------------- Binary --------------------
impl Expr {
    pub fn eq(self, other: Expr) -> Expr {
        let left = Box::new(self);
        let right = Box::new(other);
        let op = BinaryOperator::Eq;
        let expression = Expression::Binary(left, op, right);
        Expr::new(expression, None)
    }
    pub fn ne(self, other: Expr) -> Expr {
        let left = Box::new(self);
        let right = Box::new(other);
        let op = BinaryOperator::Ne;
        let expression = Expression::Binary(left, op, right);
        Expr::new(expression, None)
    }
    pub fn lt(self, other: Expr) -> Expr {
        let left = Box::new(self);
        let right = Box::new(other);
        let op = BinaryOperator::Lt;
        let expression = Expression::Binary(left, op, right);
        Expr::new(expression, None)
    }
    pub fn le(self, other: Expr) -> Expr {
        let left = Box::new(self);
        let right = Box::new(other);
        let op = BinaryOperator::Le;
        let expression = Expression::Binary(left, op, right);
        Expr::new(expression, None)
    }
    pub fn ge(self, other: Expr) -> Expr {
        let left = Box::new(self);
        let right = Box::new(other);
        let op = BinaryOperator::Ge;
        let expression = Expression::Binary(left, op, right);
        Expr::new(expression, None)
    }
    pub fn gt(self, other: Expr) -> Expr {
        let left = Box::new(self);
        let right = Box::new(other);
        let op = BinaryOperator::Gt;
        let expression = Expression::Binary(left, op, right);
        Expr::new(expression, None)
    }
    pub fn implies(self, other: Expr) -> Expr {
        let left = Box::new(self);
        let right = Box::new(other);
        let op = BinaryOperator::Implies;
        let expression = Expression::Binary(left, op, right);
        Expr::new(expression, None)
    }
    pub fn min(self, other: Expr) -> Expr {
        let left = Box::new(self);
        let right = Box::new(other);
        let op = BinaryOperator::Min;
        let expression = Expression::Binary(left, op, right);
        Expr::new(expression, None)
    }
    pub fn max(self, other: Expr) -> Expr {
        let left = Box::new(self);
        let right = Box::new(other);
        let op = BinaryOperator::Max;
        let expression = Expression::Binary(left, op, right);
        Expr::new(expression, None)
    }

    pub fn and(v: Vec<Expr>) -> Expr {
        Expr::new(Expression::Nary(NaryOperator::And, v), None)
    }
    pub fn or(v: Vec<Expr>) -> Expr {
        Expr::new(Expression::Nary(NaryOperator::Or, v), None)
    }

    pub fn mul(v: Vec<Expr>) -> Expr {
        Expr::new(Expression::Nary(NaryOperator::Mul, v), None)
    }
    pub fn add(v: Vec<Expr>) -> Expr {
        Expr::new(Expression::Nary(NaryOperator::Add, v), None)
    }
    pub fn sub(v: Vec<Expr>) -> Expr {
        Expr::new(Expression::Nary(NaryOperator::Sub, v), None)
    }

    pub fn apply(fun: FunDecId, params: Vec<Expr>) -> Expr {
        let f = fun.into();
        let expression = Expression::Apply(Box::new(f), params);
        Expr::new(expression, None)
    }

    pub fn forall(params: Vec<Parameter>, e: Expr) -> Expr {
        QtOperator::Forall.new(params, e).into()
    }
    pub fn exists(params: Vec<Parameter>, e: Expr) -> Expr {
        QtOperator::Exists.new(params, e).into()
    }
    pub fn sum(params: Vec<Parameter>, e: Expr) -> Expr {
        QtOperator::Sum.new(params, e).into()
    }
    pub fn prod(params: Vec<Parameter>, e: Expr) -> Expr {
        QtOperator::Prod.new(params, e).into()
    }
    pub fn minimum(params: Vec<Parameter>, e: Expr) -> Expr {
        QtOperator::Min.new(params, e).into()
    }
    pub fn maximum(params: Vec<Parameter>, e: Expr) -> Expr {
        QtOperator::Max.new(params, e).into()
    }

    pub fn following(self) -> Expr {
        let kid = Box::new(self);
        let expression = Expression::Following(kid);
        Expr::new(expression, None)
    }
    pub fn state_with_default(self, state: StateIndex, default: Option<Expr>) -> Expr {
        let kid = Box::new(self);
        let default = match default {
            Some(default) => Some(Box::new(default)),
            None => None,
        };
        let expression = Expression::State(kid, state, default);
        Expr::new(expression, None)
    }
    pub fn state(self, index: usize) -> Expr {
        let kid = Box::new(self);
        let state_expr = StateIndex(State::First, index as isize);
        let expression = Expression::State(kid, state_expr, None);
        Expr::new(expression, None)
    }
}

impl std::ops::Add for Expr {
    type Output = Expr;
    fn add(self, rhs: Self) -> Self::Output {
        Expr::add(vec![self, rhs])
    }
}
impl std::ops::Sub for Expr {
    type Output = Expr;
    fn sub(self, rhs: Self) -> Self::Output {
        Expr::sub(vec![self, rhs])
    }
}
impl std::ops::Mul for Expr {
    type Output = Expr;
    fn mul(self, rhs: Self) -> Self::Output {
        Expr::mul(vec![self, rhs])
    }
}

impl std::ops::BitAnd for Expr {
    type Output = Expr;

    fn bitand(self, rhs: Self) -> Self::Output {
        Expr::and(vec![self, rhs])
    }
}
impl std::ops::BitOr for Expr {
    type Output = Expr;

    fn bitor(self, rhs: Self) -> Self::Output {
        Expr::or(vec![self, rhs])
    }
}
