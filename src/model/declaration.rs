use super::*;
use crate::error::*;
use crate::parser::Position;
use crate::typing::*;
use crate::*;
use std::collections::HashMap;

//------------------------- Id -------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct DeclarationId(pub usize);

impl Id for DeclarationId {
    fn empty() -> Self {
        Self(0)
    }
    fn index(&self) -> usize {
        self.0
    }
}

//------------------------- Declaration -------------------------

#[derive(Clone)]
pub struct Declaration {
    id: DeclarationId,
    constant: bool,
    name: String,
    typ: Type,
    position: Option<Position>,
}

impl Declaration {
    pub fn new<S: Into<String>>(
        constant: bool,
        name: S,
        typ: Type,
        position: Option<Position>,
    ) -> Self {
        let id = DeclarationId::empty();
        let name = name.into();
        Self {
            id,
            constant,
            name,
            typ,
            position,
        }
    }

    pub fn is_constant(&self) -> bool {
        self.constant
    }
}

//------------------------- Postion -------------------------

impl WithPosition for Declaration {
    fn position(&self) -> &Option<Position> {
        &self.position
    }
}

//------------------------- Named -------------------------

impl Named<DeclarationId> for Declaration {
    fn id(&self) -> DeclarationId {
        self.id
    }

    fn set_id(&mut self, id: DeclarationId) {
        self.id = id;
    }

    fn name(&self) -> &str {
        &self.name
    }
}

//------------------------- With Type -------------------------

impl WithType for Declaration {
    fn get_type(&self, model: &Model) -> Type {
        self.typ.get_type(model)
    }

    fn resolve_type(&mut self, types: &HashMap<String, Type>) -> Result<(), Error> {
        self.typ = self.typ.resolve(types)?;
        Ok(())
    }
}

//------------------------- ToLang -------------------------

impl ToLang for Declaration {
    fn to_lang(&self, model: &Model) -> String {
        let mut res = if self.is_constant() {
            "cst".to_string()
        } else {
            "var".to_string()
        };
        res.push_str(&format!(" {}: {}", self.name(), self.typ.to_lang(model)));
        res
    }
}

//------------------------- ToDebug -------------------------

impl ToDebug for Declaration {
    fn to_debug(&self, model: &Model) -> String {
        let mut res = format!("// {:?}\n", self.id);
        if self.is_constant() {
            res += "cst";
        } else {
            res += "var";
        };
        res += &format!(" {}: {}", self.name(), self.typ.to_lang(model));
        res
    }
}

//------------------------- Display -------------------------

impl std::fmt::Display for Declaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
