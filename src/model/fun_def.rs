use super::*;
use crate::error::*;
use crate::expr::*;
use crate::parser::Position;
use crate::typing::*;
use crate::*;
use std::collections::HashMap;

//------------------------- Id -------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct FunDefId(pub usize);

impl Id for FunDefId {
    fn empty() -> Self {
        Self(0)
    }
    fn index(&self) -> usize {
        self.0
    }
}

//------------------------- FunDef -------------------------

#[derive(Clone)]
pub struct FunDef {
    id: FunDefId,
    name: String,
    parameters: Vec<Parameter>,
    return_type: Type,
    expr: Expr,
    position: Option<Position>,
}

impl FunDef {
    pub fn new<S: Into<String>>(
        name: S,
        parameters: Vec<Parameter>,
        return_type: Type,
        expr: Expr,
        position: Option<Position>,
    ) -> Self {
        let id = FunDefId::empty();
        let name = name.into();
        Self {
            id,
            name,
            parameters,
            return_type,
            expr,
            position,
        }
    }

    pub fn return_type(&self) -> &Type {
        &self.return_type
    }

    pub fn expr(&self) -> &Expr {
        &self.expr
    }

    //---------- Parameter ----------

    pub fn add_parameter(&mut self, parameter: Parameter) {
        self.parameters.push(parameter);
    }

    pub fn parameters(&self) -> &Vec<Parameter> {
        &self.parameters
    }

    //---------- Unicity ----------

    pub fn check_unicity(&self) -> Result<(), Error> {
        for i in 0..self.parameters.len() - 1 {
            let x = &self.parameters[i];
            for j in i + 1..self.parameters.len() {
                let y = &self.parameters[j];
                if x.name() == y.name() {
                    return Err(Error::Duplicate {
                        name: x.name().to_string(),
                        first: x.position().clone(),
                        second: y.position().clone(),
                    });
                }
            }
        }
        Ok(())
    }

    //---------- Resolve Expr ----------

    pub fn resolve_expr(&self, model: &Model, entries: &Vec<Entry>) -> Result<Self, Error> {
        let mut v = entries.clone();
        for p in self.parameters.iter() {
            v.push(p.into());
        }

        Ok(Self {
            id: self.id,
            name: self.name.clone(),
            parameters: self.parameters.clone(),
            return_type: self.return_type.clone(),
            expr: self.expr.resolve(model, &v)?,
            position: self.position.clone(),
        })
    }

    //---------- Check Type ----------

    pub fn check_type(&self, model: &Model) -> Result<(), Error> {
        self.expr.check_type(model)?;
        let def_type = self.return_type.get_type(model);
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

    //---------- Duplicate ----------

    // pub fn duplicate(&self) -> Result<(), Error> {
    //     for i in 0..self.parameters.len() - 1 {
    //         let x = &self.parameters[i];
    //         for j in i + 1..self.parameters.len() {
    //             let y = &self.parameters[j];
    //             if x.name() == y.name() {
    //                 return Err(Error::Duplicate {
    //                     name: x.name().to_string(),
    //                     first: x.position().clone(),
    //                     second: y.position().clone(),
    //                 });
    //             }
    //         }
    //     }
    //     Ok(())
    // }

    //---------- Bounded ----------

    // pub fn check_bounded(&self, problem: &Problem) -> Result<(), Error> {
    //     for p in self.parameters.iter() {
    //         p.check_bounded(problem)?;
    //     }
    //     Ok(())
    // }

    //---------- Propagate Expr ----------

    pub fn propagate_expr(&self, model: &Model) -> Self {
        Self {
            id: self.id,
            name: self.name.clone(),
            parameters: self.parameters.clone(),
            return_type: self.return_type.clone(),
            expr: self.expr.propagate(model),
            position: self.position.clone(),
        }
    }
}

//------------------------- Postion -------------------------

impl WithPosition for FunDef {
    fn position(&self) -> &Option<Position> {
        &self.position
    }
}

//------------------------- Named -------------------------

impl Named<FunDefId> for FunDef {
    fn id(&self) -> FunDefId {
        self.id
    }

    fn set_id(&mut self, id: FunDefId) {
        self.id = id;
    }

    fn name(&self) -> &str {
        &self.name
    }
}

//------------------------- With Type -------------------------

impl WithType for FunDef {
    fn get_type(&self, model: &Model) -> Type {
        let ret = self.return_type.get_type(model);
        let mut params = Vec::new();
        for p in self.parameters.iter() {
            params.push(p.get_type(model));
        }
        Type::Function(params, Box::new(ret))
    }

    fn resolve_type(&mut self, types: &HashMap<String, Type>) -> Result<(), Error> {
        self.return_type = self.return_type.resolve(types)?;
        for p in self.parameters.iter_mut() {
            p.resolve_type(types)?;
        }
        let e = self.expr.resolve_type(types)?;
        self.expr = e;
        Ok(())
    }
}

//------------------------- ToLang -------------------------

impl ToLang for FunDef {
    fn to_lang(&self, model: &Model) -> String {
        let mut res = format!("let {}(", self.name());
        if let Some((first, others)) = self.parameters.split_first() {
            res += &first.to_lang(model);
            for p in others.iter() {
                res += &format!(", {}", p.to_lang(model));
            }
        }
        res += &format!(
            "): {} = {}",
            self.return_type.to_lang(model),
            self.expr.to_lang(model)
        );
        res
    }
}

//------------------------- ToDebug -------------------------

impl ToDebug for FunDef {
    fn to_debug(&self, model: &Model) -> String {
        let mut res = format!("// {:?}\n", self.id);
        res += &format!(" {}(", self.name());
        if let Some((first, others)) = self.parameters.split_first() {
            res += &first.to_lang(model);
            for p in others.iter() {
                res += &format!(", {}", p.to_lang(model));
            }
        }
        res += &format!(
            "): {} = {}",
            self.return_type.to_lang(model),
            self.expr.to_debug(model)
        );
        res
    }
}

//------------------------- Display -------------------------

impl std::fmt::Display for FunDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
