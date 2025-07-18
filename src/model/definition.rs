use super::*;
use crate::error::*;
use crate::expr::*;
use crate::parser::Position;
use crate::typing::*;
use crate::*;
use std::collections::HashMap;

//------------------------- Id -------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct DefinitionId(pub usize);

impl Id for DefinitionId {
    fn empty() -> Self {
        Self(0)
    }
    fn index(&self) -> usize {
        self.0
    }
}

//------------------------- Definition -------------------------

#[derive(Debug, Clone)]
pub struct Definition {
    id: DefinitionId,
    name: String,
    typ: Type,
    expr: Expr,
    position: Option<Position>,
}

impl Definition {
    pub fn new<S: Into<String>>(
        name: S,
        typ: Type,
        expr: Expr,
        position: Option<Position>,
    ) -> Self {
        let id = DefinitionId::empty();
        let name = name.into();
        Self {
            id,
            name,
            typ,
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
            typ: self.typ.clone(),
            expr: self.expr.resolve(model, entries)?,
            position: self.position.clone(),
        })
    }

    //---------- Check Type ----------

    pub fn check_type(&self, model: &Model) -> Result<(), Error> {
        self.expr.check_type(model)?;
        let def_type = self.typ.get_type(model);
        self.expr.check_subtype(model, &def_type)?;
        //
        Ok(())
    }

    //---------- Check Time ----------

    pub fn check_time(&self) -> Result<(), Error> {
        if let Some(expr) = self.expr.get_following() {
            let message = "Following not allowed in 'definition'".into();
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
            typ: self.typ.clone(),
            expr: self.expr.propagate(model),
            position: self.position.clone(),
        }
    }
}

//------------------------- Postion -------------------------

impl WithPosition for Definition {
    fn position(&self) -> &Option<Position> {
        &self.position
    }
}

//------------------------- Named -------------------------

impl Named<DefinitionId> for Definition {
    fn id(&self) -> DefinitionId {
        self.id
    }

    fn set_id(&mut self, id: DefinitionId) {
        self.id = id;
    }

    fn name(&self) -> &str {
        &self.name
    }
}

//------------------------- With Type -------------------------

impl WithType for Definition {
    fn get_type(&self, model: &Model) -> Type {
        self.typ.get_type(model)
    }

    fn resolve_type(&mut self, types: &HashMap<String, Type>) -> Result<(), Error> {
        self.typ = self.typ.resolve(types)?;
        let e = self.expr.resolve_type(types)?;
        self.expr = e;
        Ok(())
    }
}

//------------------------- ToLang -------------------------

impl ToLang for Definition {
    fn to_lang(&self, model: &Model) -> String {
        format!(
            "let {}: {} = {}",
            self.name(),
            self.typ.to_lang(model),
            self.expr.to_lang(model)
        )
    }
}

//------------------------- ToDebug -------------------------

impl ToDebug for Definition {
    fn to_debug(&self, model: &Model) -> String {
        let mut res = format!("// {:?}\n", self.id);
        res += &format!(
            "let {}: {} = {}",
            self.name(),
            self.typ.to_lang(model),
            self.expr.to_debug(model)
        );
        res
    }
}

//------------------------- Display -------------------------

impl std::fmt::Display for Definition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
