use super::Expr;
use crate::model::*;
use crate::*;
use fraction::Fraction;

//------------------------- Prefix Unary Operator -------------------------

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PrefixUnaryOperator {
    Not,
    Neg,
}

impl PrefixUnaryOperator {
    pub fn new(&self, e: Expr) -> Expression {
        Expression::PrefixUnary(*self, Box::new(e))
    }
}

impl std::fmt::Display for PrefixUnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrefixUnaryOperator::Not => write!(f, "not"),
            PrefixUnaryOperator::Neg => write!(f, "-"),
        }
    }
}

//-------------------------------------------------- Binary Operator --------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BinaryOperator {
    Eq,
    Ne,
    Lt,
    Le,
    Ge,
    Gt,
    //
    Implies,
    Min,
    Max,
}

impl BinaryOperator {
    pub fn new(&self, left: Expr, right: Expr) -> Expression {
        Expression::Binary(Box::new(left), *self, Box::new(right))
    }
}

impl std::fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eq => write!(f, "="),
            Self::Ne => write!(f, "!="),
            Self::Lt => write!(f, "<"),
            Self::Le => write!(f, "<="),
            Self::Ge => write!(f, ">="),
            Self::Gt => write!(f, ">"),
            //
            Self::Implies => write!(f, "=>"),
            //
            Self::Min => write!(f, "min"),
            Self::Max => write!(f, "max"),
        }
    }
}

//-------------------------------------------------- Nary Operator --------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum NaryOperator {
    And,
    Or,
    //
    Add,
    Sub,
    Mul,
}

impl NaryOperator {
    pub fn new(&self, v: Vec<Expr>) -> Expression {
        Expression::Nary(*self, v)
    }
}

impl std::fmt::Display for NaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::And => write!(f, "and"),
            Self::Or => write!(f, "or"),
            //
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
        }
    }
}

//-------------------------------------------------- Qt --------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum QtOperator {
    Forall,
    Exists,
    Sum,
    Prod,
    Min,
    Max,
}

impl QtOperator {
    pub fn new(&self, params: Vec<Parameter>, e: Expr) -> Expression {
        Expression::Quantifier(*self, params, Box::new(e))
    }
}

impl std::fmt::Display for QtOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QtOperator::Forall => write!(f, "forall"),
            QtOperator::Exists => write!(f, "exists"),
            QtOperator::Sum => write!(f, "sum"),
            QtOperator::Prod => write!(f, "prod"),
            QtOperator::Min => write!(f, "min"),
            QtOperator::Max => write!(f, "max"),
        }
    }
}

//-------------------------------------------------- LTL Operator --------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum LTLUnaryOperator {
    X,
    F,
    G,
    _F_,
    _G_,
}

impl LTLUnaryOperator {
    pub fn new(&self, e: Expr) -> Expression {
        Expression::LTLunary(*self, Box::new(e))
    }
}

impl std::fmt::Display for LTLUnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LTLUnaryOperator::X => write!(f, "X"),
            LTLUnaryOperator::F => write!(f, "F"),
            LTLUnaryOperator::G => write!(f, "G"),
            LTLUnaryOperator::_F_ => write!(f, "_F_"),
            LTLUnaryOperator::_G_ => write!(f, "_G_"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum LTLBinaryOperator {
    U,
    R,
    _U_,
    _R_,
}

impl LTLBinaryOperator {
    pub fn new(&self, left: Expr, right: Expr) -> Expression {
        Expression::LTLbinary(Box::new(left), *self, Box::new(right))
    }
}

impl std::fmt::Display for LTLBinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LTLBinaryOperator::U => write!(f, "U"),
            LTLBinaryOperator::R => write!(f, "R"),
            LTLBinaryOperator::_U_ => write!(f, "_U_"),
            LTLBinaryOperator::_R_ => write!(f, "_R_"),
        }
    }
}

//-------------------------------------------------- State Expression --------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum State {
    First,
    Current,
    Last,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::First => write!(f, "first"),
            State::Current => write!(f, "current"),
            State::Last => write!(f, "Last"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct StateIndex(pub State, pub isize);

impl StateIndex {
    pub fn state(&self) -> State {
        self.0
    }
    pub fn shift(&self) -> isize {
        self.1
    }
}

impl std::fmt::Display for StateIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.shift() < 0 {
            write!(f, "{} - {}", self.state(), -self.shift())
        } else if self.shift() == 0 {
            write!(f, "{}", self.state())
        } else {
            write!(f, "{} + {}", self.state(), self.shift())
        }
    }
}

//-------------------------------------------------- Expression --------------------------------------------------

#[derive(Clone, Debug)]
pub enum Expression {
    // Values
    Bool(bool),
    Int(i64),
    Real(Fraction),
    //
    PrefixUnary(PrefixUnaryOperator, Box<Expr>),
    Binary(Box<Expr>, BinaryOperator, Box<Expr>),
    Nary(NaryOperator, Vec<Expr>),
    //
    EnumerateElement(EnumerateElementId),
    Declaration(DeclarationId),
    Definition(DefinitionId),
    FunDec(FunDecId),
    FunDef(FunDefId),
    //
    Parameter(Parameter),
    //
    Apply(Box<Expr>, Vec<Expr>),
    //
    As(Box<Expr>, Type, Box<Expr>),
    //
    Following(Box<Expr>),
    State(Box<Expr>, StateIndex, Option<Box<Expr>>),
    Scope(Vec<Expr>, Box<Expr>),
    //
    IfThenElse(Box<Expr>, Box<Expr>, Vec<(Expr, Expr)>, Box<Expr>),
    Quantifier(QtOperator, Vec<Parameter>, Box<Expr>),
    //
    // LTL
    LTLunary(LTLUnaryOperator, Box<Expr>),
    LTLbinary(Box<Expr>, LTLBinaryOperator, Box<Expr>),
    //
    LTLVariable(LTLVariableId),
    //
    Unresolved(String),
}

//------------------------- ToLang -------------------------

impl ToLang for Expression {
    fn to_lang(&self, model: &Model) -> String {
        match self {
            Expression::Bool(value) => format!("{}", value),
            Expression::Int(value) => format!("{}", value),
            Expression::Real(value) => format!("{}", value),
            //
            Expression::PrefixUnary(op, e) => format!("({} {})", op, e.to_lang(model)),
            Expression::Binary(l, o, r) => {
                format!("({} {} {})", l.to_lang(model), o, r.to_lang(model))
            }
            Expression::Nary(op, exprs) => {
                if let Some((first, others)) = exprs.split_first() {
                    format!(
                        "({})",
                        others.iter().fold(first.to_lang(model), |prev, x| format!(
                            "{} {} {}",
                            prev,
                            *op,
                            x.to_lang(model)
                        ))
                    )
                } else {
                    panic!("empty exprs in {:?}", self)
                }
            }
            //
            Expression::EnumerateElement(id) => model.get(*id).unwrap().name().into(),
            Expression::Declaration(id) => model.get(*id).unwrap().name().into(),
            Expression::Definition(id) => model.get(*id).unwrap().name().into(),
            Expression::FunDec(id) => model.get(*id).unwrap().name().into(),
            Expression::FunDef(id) => model.get(*id).unwrap().name().into(),
            //
            Expression::Parameter(param) => param.name().into(),
            //
            Expression::Apply(f, params) => {
                let mut res = format!("{}(", f.to_lang(model));
                if let Some((first, others)) = params.split_first() {
                    res += &first.to_lang(model);
                    for p in others.iter() {
                        res += &format!(", {}", p.to_lang(model));
                    }
                }
                res += ")";
                res
            }
            //
            Expression::As(kid, typ, default) => format!(
                "{} as {} default {}",
                kid.to_lang(model),
                typ.to_lang(model),
                default.to_lang(model)
            ),
            //
            Expression::Following(kid) => format!("{}'", kid.to_lang(model)),
            Expression::State(expr, state, default) => match default {
                Some(default) => {
                    format!(
                        "({} at {} default {})",
                        expr.to_lang(model),
                        state,
                        default.to_lang(model)
                    )
                }
                None => format!("({} at {})", expr.to_lang(model), state),
            },
            Expression::Scope(l, e) => {
                let mut res = "|".to_string();
                if let Some((first, others)) = l.split_first() {
                    res += &first.to_lang(model);
                    for p in others.iter() {
                        res += &format!(", {}", p.to_lang(model));
                    }
                }
                res += &format!("|{}", e.to_lang(model));
                res
            }
            //
            Expression::IfThenElse(c, t, l, e) => {
                let mut s = format!("if {} then {}", c.to_lang(model), t.to_lang(model));
                for (x, y) in l.iter() {
                    s.push_str(&format!(
                        " elif {} then {}",
                        x.to_lang(model),
                        y.to_lang(model)
                    ));
                }
                s.push_str(&format!(" else {} end", e.to_lang(model)));
                s
            }
            Expression::Quantifier(op, p, e) => {
                let mut s = format!("{} ", op);
                if let Some((first, others)) = p.split_first() {
                    s.push_str(&first.to_lang(model));
                    for x in others.iter() {
                        s.push_str(&format!(", {}", x.to_lang(model)));
                    }
                }
                s.push_str(&format!(" | {} end", e.to_lang(model)));
                s
            }
            //
            Expression::LTLunary(op, e) => format!("({} {})", op, e.to_lang(model)),
            Expression::LTLbinary(l, o, r) => {
                format!("({} {} {})", l.to_lang(model), o, r.to_lang(model))
            }
            //
            Expression::LTLVariable(id) => model.get(*id).unwrap().name(),
            //
            Expression::Unresolved(name) => format!("?{}", name),
        }
    }
}

//------------------------- ToDebug -------------------------

impl ToDebug for Expression {
    fn to_debug(&self, model: &Model) -> String {
        match self {
            Expression::Bool(value) => format!("{}", value),
            Expression::Int(value) => format!("{}", value),
            Expression::Real(value) => format!("{}", value),
            //
            Expression::PrefixUnary(op, e) => format!("({} {})", op, e.to_debug(model)),
            Expression::Binary(l, o, r) => {
                format!("({} {} {})", l.to_debug(model), o, r.to_debug(model))
            }
            Expression::Nary(op, exprs) => {
                if let Some((first, others)) = exprs.split_first() {
                    format!(
                        "({})",
                        others.iter().fold(first.to_debug(model), |prev, x| format!(
                            "{} {} {}",
                            prev,
                            *op,
                            x.to_debug(model)
                        ))
                    )
                } else {
                    panic!("empty exprs in {:?}", self)
                }
            }
            //
            Expression::EnumerateElement(id) => model.get(*id).unwrap().name().into(),
            Expression::Declaration(id) => model.get(*id).unwrap().name().into(),
            Expression::Definition(id) => model.get(*id).unwrap().name().into(),
            Expression::FunDec(id) => model.get(*id).unwrap().name().into(),
            Expression::FunDef(id) => model.get(*id).unwrap().name().into(),
            //
            Expression::Parameter(param) => param.name().into(),
            //
            Expression::Apply(f, params) => {
                let mut res = format!("{}(", f.to_debug(model));
                if let Some((first, others)) = params.split_first() {
                    res += &first.to_debug(model);
                    for p in others.iter() {
                        res += &format!(", {}", p.to_debug(model));
                    }
                }
                res += ")";
                res
            }
            //
            Expression::As(kid, typ, default) => format!(
                "{} as {} default {}",
                kid.to_debug(model),
                typ.to_lang(model),
                default.to_debug(model)
            ),
            //
            Expression::Following(kid) => format!("{}'", kid.to_debug(model)),
            Expression::State(expr, state, default) => match default {
                Some(default) => {
                    format!(
                        "({} at {} default {})",
                        expr.to_debug(model),
                        state,
                        default.to_debug(model)
                    )
                }
                None => format!("({} at {})", expr.to_debug(model), state),
            },
            Expression::Scope(l, e) => {
                let mut res = "|".to_string();
                if let Some((first, others)) = l.split_first() {
                    res += &first.to_lang(model);
                    for p in others.iter() {
                        res += &format!(", {}", p.to_debug(model));
                    }
                }
                res += &format!("|{}", e.to_debug(model));
                res
            } //
            Expression::IfThenElse(c, t, l, e) => {
                let mut s = format!("if {} then {}", c.to_debug(model), t.to_debug(model));
                for (x, y) in l.iter() {
                    s.push_str(&format!(
                        " elif {} then {}",
                        x.to_debug(model),
                        y.to_debug(model)
                    ));
                }
                s.push_str(&format!(" else {} end", e.to_debug(model)));
                s
            }
            Expression::Quantifier(op, p, e) => {
                let mut s = format!("{} ", op);
                if let Some((first, others)) = p.split_first() {
                    s.push_str(&first.to_lang(model));
                    for x in others.iter() {
                        s.push_str(&format!(", {}", x.to_lang(model)));
                    }
                }
                s.push_str(&format!(" | {} end", e.to_debug(model)));
                s
            }
            //
            Expression::LTLunary(op, e) => format!("({} {})", op, e.to_debug(model)),
            Expression::LTLbinary(l, o, r) => {
                format!("({} {} {})", l.to_debug(model), o, r.to_debug(model))
            }
            //
            Expression::LTLVariable(id) => model.get(*id).unwrap().to_debug(model),
            //
            Expression::Unresolved(name) => format!("?{}", name),
        }
    }
}
