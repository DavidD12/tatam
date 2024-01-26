use super::*;
use crate::common::*;
use crate::error::*;
use crate::model::*;

#[derive(Clone, Debug)]
pub enum SearchType {
    Solve,
    Optimize(Optimization),
}

impl SearchType {
    //---------- Resolve Expr ----------

    pub fn resolve_expr(&self, model: &Model, entries: &Vec<Entry>) -> Result<Self, Error> {
        match self {
            SearchType::Solve => Ok(self.clone()),
            SearchType::Optimize(optimization) => {
                let optimization = optimization.resolve_expr(model, entries)?;
                Ok(SearchType::Optimize(optimization))
            }
        }
    }

    pub fn is_optimization(&self) -> bool {
        match self {
            SearchType::Solve => false,
            SearchType::Optimize(_) => true,
        }
    }

    pub fn optimization(&self) -> Option<&Optimization> {
        match self {
            SearchType::Solve => None,
            SearchType::Optimize(o) => Some(o),
        }
    }

    //---------- Check Type ----------

    pub fn check_type(&self, model: &Model) -> Result<(), Error> {
        match self {
            SearchType::Solve => Ok(()),
            SearchType::Optimize(opt) => opt.check_type(model),
        }
    }

    //---------- Check Time ----------

    pub fn check_time(&self) -> Result<(), Error> {
        match self {
            SearchType::Solve => Ok(()),
            SearchType::Optimize(opt) => opt.check_time(),
        }
    }

    //---------- Propagate Expr ----------

    pub fn propagate_expr(&self, model: &Model) -> Self {
        match self {
            SearchType::Solve => self.clone(),
            SearchType::Optimize(opt) => Self::Optimize(opt.propagate_expr(model)),
        }
    }
}

//------------------------- ToLang -------------------------

impl ToLang for SearchType {
    fn to_lang(&self, model: &Model) -> String {
        match self {
            SearchType::Solve => "solve".to_string(),
            SearchType::Optimize(opt) => opt.to_lang(model),
        }
    }
}

//------------------------- ToDebug -------------------------

impl ToDebug for SearchType {
    fn to_debug(&self, model: &Model) -> String {
        match self {
            SearchType::Solve => "solve".to_string(),
            SearchType::Optimize(opt) => opt.to_debug(model),
        }
    }
}
