use crate::common::*;
use crate::error::*;
use crate::expr::Expr;
use crate::model::*;

#[derive(Clone, Debug)]
pub enum SearchType {
    Solve,
    Optimize {
        minimize: bool,
        objective: Expr,
        bound: Expr,
    }, // Bound must be a value
}

impl SearchType {
    //---------- Resolve Expr ----------

    pub fn resolve_expr(&self, model: &Model, entries: &Vec<Entry>) -> Result<Self, Error> {
        match self {
            SearchType::Solve => Ok(self.clone()),
            SearchType::Optimize {
                minimize,
                objective,
                bound,
            } => {
                let o = objective.resolve(model, entries)?;
                let b = bound.resolve(model, entries)?;
                Ok(SearchType::Optimize {
                    minimize: *minimize,
                    objective: o,
                    bound: b,
                })
            }
        }
    }

    //---------- Check Type ----------

    pub fn check_type(&self, model: &Model) -> Result<(), Error> {
        match self {
            SearchType::Solve => Ok(()),
            SearchType::Optimize {
                minimize: _,
                objective,
                bound,
            } => {
                objective.check_type(model)?;
                bound.check_type(model)?;
                objective.check_is_number(model)?;
                bound.check_is_number(model)?;
                // TODO check bound is Value
                Ok(())
            }
        }
    }

    //---------- Check Time ----------

    pub fn check_time(&self) -> Result<(), Error> {
        match self {
            SearchType::Solve => Ok(()),
            SearchType::Optimize {
                minimize: _,
                objective,
                bound,
            } => {
                if let Some(expr) = objective.get_following() {
                    let message = "Following not allowed in 'search'".into();
                    let name = "ojective".to_string();
                    let position = expr.position().clone();
                    let expr = expr.clone();
                    return Err(Error::Time {
                        message,
                        name,
                        position,
                        expr,
                    });
                }
                if let Some(expr) = bound.get_following() {
                    let message = "Following not allowed in 'search'".into();
                    let name = "bound".to_string();
                    let position = expr.position().clone();
                    let expr = expr.clone();
                    return Err(Error::Time {
                        message,
                        name,
                        position,
                        expr,
                    });
                }
                Ok(())
            }
        }
    }

    //---------- Propagate Expr ----------

    pub fn propagate_expr(&self, model: &Model) -> Self {
        match self {
            SearchType::Solve => self.clone(),
            SearchType::Optimize {
                minimize,
                objective,
                bound,
            } => {
                let o = objective.propagate(model);
                let b = bound.propagate(model);
                SearchType::Optimize {
                    minimize: *minimize,
                    objective: o,
                    bound: b,
                }
            }
        }
    }
}

//------------------------- ToLang -------------------------

impl ToLang for SearchType {
    fn to_lang(&self, model: &Model) -> String {
        match self {
            SearchType::Solve => "solve".to_string(),
            SearchType::Optimize {
                minimize,
                objective,
                bound,
            } => format!(
                "{} {} until {}",
                if *minimize { "minimize" } else { "maximize" },
                objective.to_lang(model),
                bound.to_lang(model)
            ),
        }
    }
}

//------------------------- ToDebug -------------------------

impl ToDebug for SearchType {
    fn to_debug(&self, model: &Model) -> String {
        match self {
            SearchType::Solve => "solve".to_string(),
            SearchType::Optimize {
                minimize,
                objective,
                bound,
            } => format!(
                "{} {} until {}",
                if *minimize { "minimize" } else { "maximize" },
                objective.to_debug(model),
                bound.to_debug(model)
            ),
        }
    }
}
