use super::*;
use crate::error::*;
use crate::parser::Position;
use crate::typing::*;
use crate::*;

//------------------------- Parameter -------------------------

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Parameter {
    name: String,
    typ: Type,
    position: Option<Position>,
}

impl Parameter {
    pub fn new<S: Into<String>>(name: S, typ: Type, position: Option<Position>) -> Self {
        let name = name.into();
        Self {
            name,
            typ,
            position,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_same(&self, other: &Parameter) -> bool {
        self.name == other.name && self.typ == other.typ
    }

    //---------- Bounded ----------

    pub fn check_bounded(&self, model: &Model) -> Result<(), Error> {
        if self.get_type(model).is_bounded() {
            Ok(())
        } else {
            Err(Error::Bounded {
                name: self.to_lang(model),
                position: self.position.clone(),
            })
        }
    }
}

//------------------------- Postion -------------------------

impl WithPosition for Parameter {
    fn position(&self) -> &Option<Position> {
        &self.position
    }
}

//------------------------- With Type -------------------------

impl WithType for Parameter {
    fn get_type(&self, model: &Model) -> Type {
        self.typ.get_type(model)
    }

    fn resolve_type(
        &mut self,
        types: &std::collections::HashMap<String, Type>,
    ) -> Result<(), Error> {
        self.typ = self.typ.resolve(types)?;

        Ok(())
    }
}

//------------------------- ToLang -------------------------

impl ToLang for Parameter {
    fn to_lang(&self, model: &Model) -> String {
        format!("{}: {}", self.name(), self.typ.to_lang(model))
    }
}
