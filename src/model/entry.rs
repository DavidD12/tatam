use super::*;
use crate::expr::*;
use crate::*;

//------------------------- Entry Type -------------------------

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum EntryRef {
    EnumerateElement(EnumerateElementId),
    Declaration(DeclarationId),
    Definition(DefinitionId),
    FunDec(FunDecId),
    FunDef(FunDefId),
    //
    Parameter(Parameter),
    //
    // Instance(InstanceId),
    // Variable(VariableId),
    // //
    // StrucSelf(StructureId),
    // //
    // ClassSelf(ClassId),
}

//------------------------- Entry -------------------------

#[derive(Clone, Debug)]
pub struct Entry {
    name: String,
    reference: EntryRef,
}

impl Entry {
    pub fn new(name: String, typ: EntryRef) -> Self {
        Self {
            name,
            reference: typ,
        }
    }

    // pub fn new_declaration(declaration: &Declaration) -> Self {
    //     Self {
    //         name: declaration.name().into(),
    //         reference: EntryRef::Declaration(declaration.id()),
    //     }
    // }

    // pub fn new_definition(definition: &Definition) -> Self {
    //     Self {
    //         name: definition.name().into(),
    //         reference: EntryRef::Definition(definition.id()),
    //     }
    // }

    // pub fn new_parameter(parameter: &Parameter) -> Self {
    //     let name = parameter.name().to_string();
    //     let typ = EntryRef::Parameter(parameter.clone());
    //     Self { name, typ }
    // }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reference(&self) -> &EntryRef {
        &self.reference
    }
}

//------------------------- From -------------------------

impl From<&EnumerateElement> for Entry {
    fn from(value: &EnumerateElement) -> Self {
        Self {
            name: value.name().into(),
            reference: EntryRef::EnumerateElement(value.id()),
        }
    }
}

impl From<&Declaration> for Entry {
    fn from(value: &Declaration) -> Self {
        Self {
            name: value.name().into(),
            reference: EntryRef::Declaration(value.id()),
        }
    }
}

impl From<&Definition> for Entry {
    fn from(value: &Definition) -> Self {
        Self {
            name: value.name().into(),
            reference: EntryRef::Definition(value.id()),
        }
    }
}

impl From<&FunDec> for Entry {
    fn from(value: &FunDec) -> Self {
        Self {
            name: value.name().into(),
            reference: EntryRef::FunDec(value.id()),
        }
    }
}

impl From<&FunDef> for Entry {
    fn from(value: &FunDef) -> Self {
        Self {
            name: value.name().into(),
            reference: EntryRef::FunDef(value.id()),
        }
    }
}

impl From<&Parameter> for Entry {
    fn from(value: &Parameter) -> Self {
        Self {
            name: value.name().into(),
            reference: EntryRef::Parameter(value.clone()),
        }
    }
}

//------------------------- Into Expression -------------------------

impl Into<Expression> for &Entry {
    fn into(self) -> Expression {
        match &self.reference {
            EntryRef::EnumerateElement(id) => Expression::EnumerateElement(*id),
            EntryRef::Declaration(id) => Expression::Declaration(*id),
            EntryRef::Definition(id) => Expression::Definition(*id),
            EntryRef::FunDec(id) => Expression::FunDec(*id),
            EntryRef::FunDef(id) => Expression::FunDef(*id),
            EntryRef::Parameter(param) => Expression::Parameter(param.clone()),
        }
    }
}

//------------------------- Entries -------------------------

pub fn get_entry(name: &str, entries: &Vec<Entry>) -> Option<Entry> {
    for e in entries.iter().rev() {
        if e.name() == name {
            return Some(e.clone());
        }
    }
    None
}

// #[derive(Clone, Debug)]
// pub struct Entries(Vec<NamedEntry>);

// pub type Entries = Vec<Entry>;

// impl Entries {
//     pub fn new(entries: Vec<Entry>) -> Self {
//         Entries(entries)
//     }

// fn entries(&self) -> &Vec<Entry> {
//     let Entries(entries) = self;
//     entries
// }

// pub fn add(&self, entry: Entry) -> Entries {
//     let mut v = self.entries().clone();
//     v.push(entry);
//     Entries(v)
// }

// pub fn add_all(&self, entries: Entries) -> Entries {
//     let Entries(others) = entries;
//     let mut v = self.entries().clone();
//     for entry in others {
//         v.push(entry);
//     }
//     Entries(v)
// }

//     pub fn get(&self, name: &str) -> Option<Entry> {
//         for e in self.entries().iter().rev() {
//             if e.name() == name {
//                 return Some(e.clone());
//             }
//         }
//         None
//     }
// }
