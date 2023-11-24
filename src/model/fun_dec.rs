use super::*;
use crate::error::*;
use crate::parser::Position;
use crate::typing::*;
use crate::*;
use std::collections::HashMap;

//------------------------- Id -------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct FunDecId(pub usize);

impl Id for FunDecId {
    fn empty() -> Self {
        Self(0)
    }
    fn index(&self) -> usize {
        self.0
    }
}

//------------------------- FunDec -------------------------

#[derive(Clone)]
pub struct FunDec {
    id: FunDecId,
    constant: bool,
    name: String,
    parameters: Vec<Parameter>,
    return_type: Type,
    position: Option<Position>,
}

impl FunDec {
    pub fn new<S: Into<String>>(
        constant: bool,
        name: S,
        parameters: Vec<Parameter>,
        return_type: Type,
        position: Option<Position>,
    ) -> Self {
        let id = FunDecId::empty();
        let name = name.into();
        Self {
            id,
            constant,
            name,
            parameters,
            return_type,
            position,
        }
    }

    pub fn is_constant(&self) -> bool {
        self.constant
    }

    pub fn return_type(&self) -> &Type {
        &self.return_type
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

    //---------- Bounded ----------

    pub fn check_bounded_parameters(&self, model: &Model) -> Result<(), Error> {
        for p in self.parameters.iter() {
            p.check_bounded(model)?;
        }
        Ok(())
    }
}

//------------------------- Postion -------------------------

impl WithPosition for FunDec {
    fn position(&self) -> &Option<Position> {
        &self.position
    }
}

//------------------------- Named -------------------------

impl Named<FunDecId> for FunDec {
    fn id(&self) -> FunDecId {
        self.id
    }

    fn set_id(&mut self, id: FunDecId) {
        self.id = id;
    }

    fn name(&self) -> &str {
        &self.name
    }
}

//------------------------- With Type -------------------------

impl WithType for FunDec {
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

        Ok(())
    }
}

//------------------------- ToLang -------------------------

impl ToLang for FunDec {
    fn to_lang(&self, model: &Model) -> String {
        let mut res = if self.is_constant() {
            "cst".to_string()
        } else {
            "var".to_string()
        };
        res.push_str(&format!(" {}(", self.name()));
        if let Some((first, others)) = self.parameters.split_first() {
            res += &first.to_lang(model);
            for p in others.iter() {
                res += &format!(", {}", p.to_lang(model));
            }
        }
        res += &format!("): {}", self.return_type.to_lang(model));
        res
    }
}

//------------------------- ToDebug -------------------------

impl ToDebug for FunDec {
    fn to_debug(&self, model: &Model) -> String {
        let mut res = format!("// {:?}\n", self.id);
        if self.is_constant() {
            res += "cst";
        } else {
            res += "var";
        };
        res.push_str(&format!(" {}(", self.name()));
        if let Some((first, others)) = self.parameters.split_first() {
            res += &first.to_lang(model);
            for p in others.iter() {
                res += &format!(", {}", p.to_lang(model));
            }
        }
        res += &format!("): {}", self.return_type.to_lang(model));
        res
    }
}

//------------------------- Display -------------------------

impl std::fmt::Display for FunDec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
