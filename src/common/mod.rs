use crate::error::Error;
use crate::model::Model;
use crate::parser::Position;
use crate::typing::Type;
use std::collections::HashMap;

//------------------------- Id -------------------------

pub trait Id: Clone + Copy + PartialEq + Eq + core::hash::Hash + std::fmt::Debug {
    fn empty() -> Self;
    fn index(&self) -> usize;
}

pub trait GetFromId<I: Id, T> {
    fn get(&self, id: I) -> Option<&T>;
}

pub trait FromId<I: Id> {
    fn from_id(model: &Model, id: I) -> Self;
}

//------------------------- Position -------------------------

pub trait WithPosition {
    fn position(&self) -> &Option<Position>;
}

//------------------------- Named -------------------------

pub trait Named<I: Id>: WithPosition {
    fn id(&self) -> I;
    fn set_id(&mut self, id: I);
    //
    fn name(&self) -> &str;
    fn naming(&self) -> Naming {
        Naming::new(self.name(), self.position().clone())
    }
}

pub trait FromName<T> {
    fn from_name(&self, name: &str) -> Option<&T>;
}

//------------------------- Naming -------------------------

pub struct Naming {
    name: String,
    position: Option<Position>,
}

impl Naming {
    pub fn new<S: Into<String>>(name: S, position: Option<Position>) -> Self {
        let name = name.into();
        Self { name, position }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn position(&self) -> &Option<Position> {
        &self.position
    }
}

pub fn check_unicity(names: Vec<Naming>) -> Result<(), Error> {
    for (i, x) in names.iter().enumerate() {
        for y in names.iter().skip(i + 1) {
            if x.name() == y.name() {
                return Err(Error::Duplicate {
                    name: x.name().to_string(),
                    first: x.position.clone(),
                    second: x.position.clone(),
                });
            }
        }
    }
    Ok(())
}

//------------------------- With Type -------------------------

pub trait WithType: WithPosition {
    // fn typ(&self) -> &Type;
    // fn set_type(&mut self, typ: Type);

    fn get_type(&self, model: &Model) -> Type;

    //---------- Resolve ----------

    fn resolve_type(&mut self, types: &HashMap<String, Type>) -> Result<(), Error>;
    //  {
    //     self.set_type(self.typ().resolve(types)?);
    //     Ok(())
    // }
}

//------------------------- ToLang -------------------------

pub trait ToLang {
    fn to_lang(&self, model: &Model) -> String;
}

//------------------------- ToDebug -------------------------

pub trait ToDebug {
    fn to_debug(&self, model: &Model) -> String;
}

//------------------------- ToEntry -------------------------

pub trait ToEntry {
    fn to_entry(&self, model: &Model) -> d_stuff::Entry;
}
