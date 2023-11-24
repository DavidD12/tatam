use super::Expression;
use super::*;
use crate::model::*;
use crate::parser::Position;
use crate::*;

#[derive(Clone, Debug)]
pub struct Expr {
    expression: Expression,
    position: Option<Position>,
    // type
    // is_value
    // is_constant
    // has_step
    // has_temp
}

impl Expr {
    pub fn new(expression: Expression, position: Option<Position>) -> Self {
        Self {
            expression,
            position,
        }
    }

    pub fn new_bool(value: bool, position: Option<Position>) -> Self {
        Self {
            expression: Expression::Bool(value),
            position,
        }
    }
    pub fn new_int(value: i64, position: Option<Position>) -> Self {
        Self {
            expression: Expression::Int(value),
            position,
        }
    }
    pub fn new_real(value: fraction::Fraction, position: Option<Position>) -> Self {
        Self {
            expression: Expression::Real(value),
            position,
        }
    }
    pub fn new_unresolved<S: Into<String>>(name: S, position: Option<Position>) -> Self {
        Self {
            expression: Expression::Unresolved(name.into()),
            position,
        }
    }

    pub fn expression(&self) -> &Expression {
        &self.expression
    }

    pub fn combine_all(model: &Model, parameters: &Vec<Parameter>, expr: &Expr) -> Vec<Expr> {
        let params_all = parameters
            .iter()
            .map(|p| p.get_type(model).all(model))
            .collect();
        let params_exp = parameters
            .iter()
            .map(|p| Expr::new(Expression::Parameter(p.clone()), None))
            .collect::<Vec<_>>();
        let mut combine = Combine::new(params_all);
        let mut v = Vec::new();
        loop {
            let values = combine.values();
            let all = params_exp.clone().into_iter().zip(values.clone()).collect();
            let e = expr.substitute_all(all);
            v.push(e);
            if !combine.step() {
                break;
            }
        }
        v
    }
}

//------------------------- From -------------------------

impl From<Expression> for Expr {
    fn from(value: Expression) -> Self {
        Expr::new(value, None)
    }
}

//------------------------- ToLang -------------------------

impl ToLang for Expr {
    fn to_lang(&self, model: &Model) -> String {
        self.expression.to_lang(model)
    }
}

//------------------------- ToDebug -------------------------

impl ToDebug for Expr {
    fn to_debug(&self, model: &Model) -> String {
        format!(
            "{} /* {} */",
            self.expression.to_debug(model),
            self.get_type(model).to_lang(model)
        )
    }
}

//------------------------- With Position -------------------------

impl WithPosition for Expr {
    fn position(&self) -> &Option<Position> {
        &self.position
    }
}
