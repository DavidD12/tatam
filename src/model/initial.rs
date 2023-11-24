use std::collections::HashMap;

use super::*;
use crate::error::*;
use crate::expr::*;
use crate::parser::Position;
use crate::*;

//------------------------- Id -------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct InitialId(pub usize);

impl Id for InitialId {
    fn empty() -> Self {
        Self(0)
    }
    fn index(&self) -> usize {
        self.0
    }
}

//------------------------- Initial -------------------------

#[derive(Clone)]
pub struct Initial {
    id: InitialId,
    name: String,
    expr: Expr,
    position: Option<Position>,
}

impl Initial {
    pub fn new<S: Into<String>>(name: S, expr: Expr, position: Option<Position>) -> Self {
        let id = InitialId::empty();
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
            let message = "Following not allowed in 'initial'".into();
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

impl WithPosition for Initial {
    fn position(&self) -> &Option<Position> {
        &self.position
    }
}

//------------------------- Named -------------------------

impl Named<InitialId> for Initial {
    fn id(&self) -> InitialId {
        self.id
    }

    fn set_id(&mut self, id: InitialId) {
        self.id = id;
    }

    fn name(&self) -> &str {
        &self.name
    }
}

//------------------------- ToLang -------------------------

impl ToLang for Initial {
    fn to_lang(&self, model: &Model) -> String {
        format!(
            "init {} {{\n  {}\n}}",
            self.name(),
            self.expr.to_lang(model)
        )
    }
}

//------------------------- ToDebug -------------------------

impl ToDebug for Initial {
    fn to_debug(&self, model: &Model) -> String {
        let mut res = format!("// {:?}\n", self.id);
        res += &format!(
            "init {} {{\n  {}\n}}",
            self.name(),
            self.expr.to_debug(model)
        );
        res
    }
}

//------------------------- Display -------------------------

impl std::fmt::Display for Initial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
