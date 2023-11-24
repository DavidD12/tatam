use super::*;
use crate::common::*;
use crate::error::*;
use crate::model::*;

#[derive(Clone, Debug)]
pub struct Search {
    transitions: TransitionNumber,
    path_type: PathType,
    search_type: SearchType,
}

impl Search {
    pub fn new(
        transitions: TransitionNumber,
        path_type: PathType,
        search_type: SearchType,
    ) -> Self {
        Self {
            transitions,
            path_type,
            search_type,
        }
    }

    pub fn transitions(&self) -> TransitionNumber {
        self.transitions
    }

    pub fn path_type(&self) -> PathType {
        self.path_type
    }

    pub fn search_type(&self) -> &SearchType {
        &self.search_type
    }

    //---------- Resolve Expr ----------

    pub fn resolve_expr(&self, model: &Model, entries: &Vec<Entry>) -> Result<Self, Error> {
        let search_type = self.search_type.resolve_expr(model, entries)?;
        Ok(Self {
            transitions: self.transitions,
            path_type: self.path_type,
            search_type,
        })
    }

    //---------- Check Type ----------

    pub fn check_type(&self, model: &Model) -> Result<(), Error> {
        self.search_type.check_type(model)?;
        Ok(())
    }

    //---------- Check Time ----------

    pub fn check_time(&self) -> Result<(), Error> {
        self.search_type.check_time()?;
        Ok(())
    }

    //---------- Propagate Expr ----------

    pub fn propagate_expr(&self, model: &Model) -> Self {
        Self {
            transitions: self.transitions,
            path_type: self.path_type,
            search_type: self.search_type.propagate_expr(model),
        }
    }
}

//------------------------- ToLang -------------------------

impl ToLang for Search {
    fn to_lang(&self, model: &Model) -> String {
        format!(
            "search{} {} {}",
            self.transitions,
            self.path_type,
            self.search_type().to_lang(model)
        )
    }
}

//------------------------- ToDebug -------------------------

impl ToDebug for Search {
    fn to_debug(&self, model: &Model) -> String {
        format!(
            "search{} {} {}",
            self.transitions,
            self.path_type,
            self.search_type().to_debug(model)
        )
    }
}
