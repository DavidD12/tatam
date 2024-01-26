use crate::common::*;
use crate::error::*;
use crate::expr::Expr;
use crate::model::*;

#[derive(Clone, Debug)]
pub struct Optimization {
    pub minimize: bool,
    pub objective: Expr,
    pub bound: Expr,
}

impl Optimization {
    //---------- Resolve Expr ----------

    pub fn resolve_expr(&self, model: &Model, entries: &Vec<Entry>) -> Result<Self, Error> {
        let o = self.objective.resolve(model, entries)?;
        let b = self.bound.resolve(model, entries)?;
        Ok(Self {
            minimize: self.minimize,
            objective: o,
            bound: b,
        })
    }

    //---------- Check Type ----------

    pub fn check_type(&self, model: &Model) -> Result<(), Error> {
        self.objective.check_type(model)?;
        self.bound.check_type(model)?;
        self.objective.check_is_number(model)?;
        self.bound.check_is_number(model)?;
        // TODO check bound is Value
        Ok(())
    }

    //---------- Check Time ----------

    pub fn check_time(&self) -> Result<(), Error> {
        if let Some(expr) = self.objective.get_following() {
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
        if let Some(expr) = self.bound.get_following() {
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

    //---------- Propagate Expr ----------

    pub fn propagate_expr(&self, model: &Model) -> Self {
        let o = self.objective.propagate(model);
        let b = self.bound.propagate(model);
        Self {
            minimize: self.minimize,
            objective: o,
            bound: b,
        }
    }
}

//------------------------- ToLang -------------------------

impl ToLang for Optimization {
    fn to_lang(&self, model: &Model) -> String {
        format!(
            "{} {} until {}",
            if self.minimize {
                "minimize"
            } else {
                "maximize"
            },
            self.objective.to_lang(model),
            self.bound.to_lang(model)
        )
    }
}

//------------------------- ToDebug -------------------------

impl ToDebug for Optimization {
    fn to_debug(&self, model: &Model) -> String {
        format!(
            "{} {} until {}",
            if self.minimize {
                "minimize"
            } else {
                "maximize"
            },
            self.objective.to_debug(model),
            self.bound.to_debug(model)
        )
    }
}
