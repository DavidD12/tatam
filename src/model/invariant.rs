use std::collections::HashMap;

use super::*;
use crate::error::*;
use crate::expr::*;
use crate::parser::Position;
use crate::*;

//------------------------- Id -------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct InvariantId(pub usize);

impl Id for InvariantId {
    fn empty() -> Self {
        Self(0)
    }
    fn index(&self) -> usize {
        self.0
    }
}

//------------------------- Invariant -------------------------

#[derive(Clone)]
pub struct Invariant {
    id: InvariantId,
    name: String,
    expr: Expr,
    position: Option<Position>,
}

impl Invariant {
    pub fn new<S: Into<String>>(name: S, expr: Expr, position: Option<Position>) -> Self {
        let id = InvariantId::empty();
        let name = name.into();
        Self {
            id,
            name,
            expr,
            position,
        }
    }

    pub fn expr(&self) -> &Expr {
        &self.expr
    }

    //---------- Resolve Type ----------

    pub fn resolve_type(&mut self, types: &HashMap<String, Type>) -> Result<(), Error> {
        let e = self.expr.resolve_type(types)?;
        self.expr = e;
        Ok(())
    }

    //---------- Resolve Expr ----------

    pub fn resolve_expr(&self, model: &Model, entries: &Vec<Entry>) -> Result<Self, Error> {
        Ok(Self {
            id: self.id,
            name: self.name.clone(),
            expr: self.expr.resolve(model, entries)?,
            position: self.position.clone(),
        })
    }

    //---------- Check Type ----------

    pub fn check_type(&self, model: &Model) -> Result<(), Error> {
        self.expr.check_type(model)?;
        self.expr.check_is_bool(model)?;
        //
        Ok(())
    }

    //---------- Check Time ----------

    pub fn check_time(&self) -> Result<(), Error> {
        if let Some(expr) = self.expr.get_following() {
            let message = "Following not allowed in 'invariant'".into();
            let name = self.name.clone();
            let position = self.position.clone();
            let expr = expr.clone();
            Err(Error::Time {
                message,
                name,
                position,
                expr,
            })
        } else {
            Ok(())
        }
    }

    //---------- Propagate Expr ----------

    pub fn propagate_expr(&self, model: &Model) -> Self {
        Self {
            id: self.id,
            name: self.name.clone(),
            expr: self.expr.propagate(model),
            position: self.position.clone(),
        }
    }
}

//------------------------- Postion -------------------------

impl WithPosition for Invariant {
    fn position(&self) -> &Option<Position> {
        &self.position
    }
}

//------------------------- Named -------------------------

impl Named<InvariantId> for Invariant {
    fn id(&self) -> InvariantId {
        self.id
    }

    fn set_id(&mut self, id: InvariantId) {
        self.id = id;
    }

    fn name(&self) -> &str {
        &self.name
    }
}

//------------------------- ToLang -------------------------

impl ToLang for Invariant {
    fn to_lang(&self, model: &Model) -> String {
        format!("inv {} {{\n  {}\n}}", self.name(), self.expr.to_lang(model))
    }
}

//------------------------- ToDebug -------------------------

impl ToDebug for Invariant {
    fn to_debug(&self, model: &Model) -> String {
        let mut res = format!("// {:?}\n", self.id);
        res += &format!(
            "inv {} {{\n  {}\n}}",
            self.name(),
            self.expr.to_debug(model)
        );
        res
    }
}

//------------------------- Display -------------------------

impl std::fmt::Display for Invariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
