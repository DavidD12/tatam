use crate::{
    error::Error,
    expr::*,
    model::*,
    model::{EnumerateId, IntervalId},
    parser::Position,
    *,
};
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Type {
    //
    Undefined,
    Unresolved(String, Option<Position>),
    //
    Bool,
    Int,
    Real,
    //
    Enumerate(EnumerateId),
    Interval(IntervalId),
    //
    IntInterval(i64, i64),
    //
    Function(Vec<Type>, Box<Type>),
}

impl Type {
    pub fn is_enumerate(&self) -> bool {
        match self {
            Type::Enumerate(_) => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            Type::Bool => true,
            _ => false,
        }
    }

    pub fn is_integer(&self) -> bool {
        match self {
            Type::Int => true,
            Type::IntInterval(_, _) => true,
            _ => false,
        }
    }

    pub fn is_real(&self) -> bool {
        match self {
            Type::Real => true,
            _ => false,
        }
    }

    pub fn is_bounded(&self) -> bool {
        match self {
            Type::Undefined => false,
            Type::Unresolved(_, _) => false,
            Type::Bool => true,
            Type::Int => false,
            Type::Real => false,
            Type::Enumerate(_) => true,
            Type::Interval(_) => true,
            Type::IntInterval(_, _) => true,
            Type::Function(_, _) => false,
        }
    }

    pub fn resolve(&self, types: &HashMap<String, Type>) -> Result<Type, Error> {
        match self {
            Type::Unresolved(name, position) => match types.get(name) {
                Some(typ) => Ok(typ.clone()),
                None => Err(Error::Resolve {
                    category: "type".to_string(),
                    name: name.clone(),
                    position: position.clone(),
                }),
            },
            _ => Ok(self.clone()),
        }
    }

    pub fn get_type(&self, model: &Model) -> Type {
        match self {
            Type::Interval(id) => {
                let interval = model.get(*id).unwrap();
                Type::IntInterval(interval.min(), interval.max())
            }
            Type::Function(params, ret) => {
                let params = params.iter().map(|t| t.get_type(model)).collect();
                let ret = ret.get_type(model);
                Type::Function(params, Box::new(ret))
            }
            _ => self.clone(),
        }
    }

    pub fn is_subtype_of(&self, other: &Self) -> bool {
        if self == other {
            true
        } else {
            match (self, other) {
                (Type::IntInterval(_, _), Type::Int) => true,
                (Type::IntInterval(min1, max1), Type::IntInterval(min2, max2)) => {
                    min1 >= min2 && max1 <= max2
                }
                _ => false,
            }
        }
    }

    pub fn is_compatible_with(&self, other: &Self) -> bool {
        match (self, other) {
            (Type::IntInterval(_, _), Type::IntInterval(_, _)) => true,
            (Type::IntInterval(_, _), Type::Int) => true,
            (Type::Int, Type::IntInterval(_, _)) => true,
            (x, y) => x == y,
        }
    }

    pub fn common_type(&self, _model: &Model, other: &Self) -> Type {
        if self == other {
            self.clone()
        } else {
            match (self, other) {
                (Type::IntInterval(_, _), Type::Int) => Type::Int,
                (Type::Int, Type::IntInterval(_, _)) => Type::Int,
                (Type::IntInterval(min1, max1), Type::IntInterval(min2, max2)) => {
                    Type::IntInterval(*min1.min(min2), *max1.max(max2))
                }
                // (Type::Class(i1), Type::Class(i2)) => {
                //     let c1 = problem.get(*i1).unwrap();
                //     match c1.common_class(problem, *i2) {
                //         Some(id) => Type::Class(id),
                //         _ => Type::Undefined,
                //     }
                // }
                _ => Type::Undefined,
            }
        }
    }

    pub fn all(&self, model: &Model) -> Vec<Expr> {
        match self {
            Type::Enumerate(id) => model
                .get(*id)
                .unwrap()
                .elements()
                .iter()
                .map(|e| e.into())
                .collect(),
            // Type::Structure(id) => problem
            //     .get(*id)
            //     .unwrap()
            //     .instances(model)
            //     .iter()
            //     .map(|i| Expr::Instance(*i, None))
            //     .collect(),
            // Type::Class(id) => model
            //     .get(*id)
            //     .unwrap()
            //     .all_instances(model)
            //     .iter()
            //     .map(|i| Expr::Instance(*i, None))
            //     .collect(),
            Type::Bool => vec![false.into(), true.into()],
            Type::IntInterval(min, max) => (*min..=*max).into_iter().map(|i| i.into()).collect(),
            _ => vec![],
        }
    }
}

impl ToLang for Type {
    fn to_lang(&self, model: &crate::model::Model) -> String {
        match self {
            Type::Unresolved(name, _) => format!("{}?", name),
            Type::Undefined => "undef".into(),
            //
            Type::Bool => "Bool".into(),
            Type::Int => "Int".into(),
            Type::Real => "Real".into(),
            //
            Type::Enumerate(id) => model.get(*id).unwrap().name().to_string(),
            Type::Interval(id) => model.get(*id).unwrap().name().to_string(),
            //
            Type::IntInterval(min, max) => format!("{}..{}", min, max),
            //
            Type::Function(params, ret) => {
                let mut res = "(".to_string();
                if let Some((first, others)) = params.split_first() {
                    res += &first.to_lang(model);
                    for p in others.iter() {
                        res += &format!(", {}", p.to_lang(model));
                    }
                }
                res += &format!("): {}", ret.to_lang(model));
                res
            }
        }
    }
}
