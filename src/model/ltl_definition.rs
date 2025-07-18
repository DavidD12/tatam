use super::*;
use crate::error::*;
use crate::expr::*;
use crate::parser::Position;
use crate::typing::*;
use crate::*;
use std::collections::HashMap;

//------------------------- Id -------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct LtlDefinitionId(pub usize);

impl Id for LtlDefinitionId {
    fn empty() -> Self {
        Self(0)
    }
    fn index(&self) -> usize {
        self.0
    }
}

//------------------------- LTL Definition -------------------------

#[derive(Clone)]
pub struct LtlDefinition {
    id: LtlDefinitionId,
    name: String,
    expr: Expr,
    position: Option<Position>,
}

impl LtlDefinition {
    pub fn new<S: Into<String>>(name: S, expr: Expr, position: Option<Position>) -> Self {
        let id = LtlDefinitionId::empty();
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

    pub fn check_time(&self, model: &Model) -> Result<(), Error> {
        self.expr.check_time(model)?;
        //
        Ok(())
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

impl WithPosition for LtlDefinition {
    fn position(&self) -> &Option<Position> {
        &self.position
    }
}

//------------------------- Named -------------------------

impl Named<LtlDefinitionId> for LtlDefinition {
    fn id(&self) -> LtlDefinitionId {
        self.id
    }

    fn set_id(&mut self, id: LtlDefinitionId) {
        self.id = id;
    }

    fn name(&self) -> &str {
        &self.name
    }
}

//------------------------- With Type -------------------------

impl WithType for LtlDefinition {
    fn get_type(&self, _: &Model) -> Type {
        Type::Bool
    }

    fn resolve_type(&mut self, types: &HashMap<String, Type>) -> Result<(), Error> {
        let e = self.expr.resolve_type(types)?;
        self.expr = e;
        Ok(())
    }
}

//------------------------- ToLang -------------------------

impl ToLang for LtlDefinition {
    fn to_lang(&self, model: &Model) -> String {
        format!("ltl {} = {}", self.name(), self.expr.to_lang(model))
    }
}

//------------------------- ToDebug -------------------------

impl ToDebug for LtlDefinition {
    fn to_debug(&self, model: &Model) -> String {
        let mut res = format!("// {:?}\n", self.id);
        res += &format!("ltl {} = {}", self.name(), self.expr.to_debug(model));
        res
    }
}

//------------------------- Display -------------------------

impl std::fmt::Display for LtlDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
