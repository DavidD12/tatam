use super::*;
use crate::expr::*;
use crate::*;

//------------------------- Id -------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct LTLVariableId(pub usize);

impl Id for LTLVariableId {
    fn empty() -> Self {
        Self(0)
    }
    fn index(&self) -> usize {
        self.0
    }
}

//------------------------- LTL Variable -------------------------

#[derive(Clone)]
pub struct LTLVariable {
    id: LTLVariableId,
    expr: Expr,
}

impl LTLVariable {
    pub fn new(index: usize, expr: Expr) -> Self {
        let id = LTLVariableId(index);
        Self { id, expr }
    }

    pub fn id(&self) -> LTLVariableId {
        self.id
    }

    pub fn name(&self) -> String {
        format!("_{}", self.id.index())
    }

    pub fn expr(&self) -> &Expr {
        &self.expr
    }

    pub fn is_loop(&self) -> bool {
        match self.expr.expression() {
            Expression::LTLunary(op, _) => match op {
                LTLUnaryOperator::X => false,
                LTLUnaryOperator::F => false,
                LTLUnaryOperator::G => false,
                LTLUnaryOperator::_F_ => true,
                LTLUnaryOperator::_G_ => true,
            },
            Expression::LTLbinary(_, op, _) => match op {
                LTLBinaryOperator::U => false,
                LTLBinaryOperator::R => false,
                LTLBinaryOperator::_U_ => true,
                LTLBinaryOperator::_R_ => true,
            },
            _ => panic!(),
        }
    }
}

//------------------------- ToLang -------------------------

impl ToLang for LTLVariable {
    fn to_lang(&self, model: &Model) -> String {
        format!("{} = {}", self.name(), self.expr.to_lang(model))
    }
}

//------------------------- ToDebug -------------------------

impl ToDebug for LTLVariable {
    fn to_debug(&self, model: &Model) -> String {
        format!("{} = {}", self.name(), self.expr.to_debug(model))
    }
}

//------------------------- Display -------------------------

impl std::fmt::Display for LTLVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
