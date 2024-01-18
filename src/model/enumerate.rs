use super::*;
use crate::parser::Position;
use crate::*;

//------------------------- Enumerate Id -------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct EnumerateId(pub usize);

impl Id for EnumerateId {
    fn empty() -> Self {
        Self(0)
    }
    fn index(&self) -> usize {
        self.0
    }
}

//------------------------- Enumerate Element Id -------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct EnumerateElementId(pub EnumerateId, pub usize);

impl Id for EnumerateElementId {
    fn empty() -> Self {
        Self(EnumerateId::empty(), 0)
    }
    fn index(&self) -> usize {
        self.1
    }
}

impl EnumerateElementId {
    pub fn enumerate_id(&self) -> EnumerateId {
        self.0
    }
}

//------------------------- Enumerate -------------------------

#[derive(Clone)]
pub struct Enumerate {
    id: EnumerateId,
    name: String,
    elements: Vec<EnumerateElement>,
    position: Option<Position>,
}

impl Enumerate {
    pub fn new<S: Into<String>>(name: S, position: Option<Position>) -> Self {
        let id = EnumerateId::empty();
        let name = name.into();
        let elements = Vec::new();
        Self {
            id,
            name,
            elements,
            position,
        }
    }

    pub fn add_element(&mut self, element: EnumerateElement) -> EnumerateElementId {
        let id = EnumerateElementId(self.id, self.elements.len());
        let mut element = element;
        element.set_id(id);
        self.elements.push(element);
        id
    }

    pub fn elements(&self) -> &Vec<EnumerateElement> {
        &self.elements
    }

    //---------- Naming ----------

    pub fn namings(&self) -> Vec<Naming> {
        let mut v = Vec::new();
        v.push(self.naming());
        v.extend(self.elements.iter().map(|x| x.naming()));
        v
    }

    //---------- Entries ----------

    pub fn entries(&self) -> Vec<Entry> {
        let mut entries = Vec::new();
        for e in self.elements.iter() {
            entries.push(e.into());
        }
        entries
    }
}

impl Named<EnumerateId> for Enumerate {
    fn id(&self) -> EnumerateId {
        self.id
    }

    fn set_id(&mut self, id: EnumerateId) {
        self.id = id;
        for e in self.elements.iter_mut() {
            e.set_enumerate_id(id)
        }
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl WithPosition for Enumerate {
    fn position(&self) -> &Option<Position> {
        &self.position
    }
}

impl std::fmt::Display for Enumerate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl ToLang for Enumerate {
    fn to_lang(&self, _: &super::Model) -> String {
        let mut res = format!("enum {} = {{", self.name());
        if let Some((first, others)) = self.elements.split_first() {
            res.push_str(&format!("{}", first));
            for e in others {
                res.push_str(&format!(" {}", e));
            }
        }
        res.push('}');
        res
    }
}

impl ToDebug for Enumerate {
    fn to_debug(&self, model: &super::Model) -> String {
        let mut res = format!("// {:?}\n", self.id);
        res.push_str(&format!("enum {} = {{", self.name));
        if let Some((first, others)) = self.elements.split_first() {
            res.push_str(&format!("{}", first.to_debug(model)));
            for e in others {
                res.push_str(&format!(" {}", e.to_debug(model)));
            }
        }
        res.push('}');
        res
    }
}

impl GetFromId<EnumerateElementId, EnumerateElement> for Enumerate {
    fn get(&self, id: EnumerateElementId) -> Option<&EnumerateElement> {
        self.elements.get(id.index())
    }
}

//------------------------- From Name -------------------------

impl FromName<EnumerateElement> for Enumerate {
    fn from_name(&self, name: &str) -> Option<&EnumerateElement> {
        for e in self.elements.iter() {
            if e.name() == name {
                return Some(e);
            }
        }
        None
    }
}

//------------------------- Enumerate Element -------------------------

#[derive(Clone)]
pub struct EnumerateElement {
    id: EnumerateElementId,
    name: String,
    position: Option<Position>,
}

impl EnumerateElement {
    pub fn new<S: Into<String>>(name: S, position: Option<Position>) -> Self {
        let id = EnumerateElementId::empty();
        let name = name.into();
        Self { id, name, position }
    }

    pub fn set_enumerate_id(&mut self, id: EnumerateId) {
        self.id.0 = id
    }
}

impl Named<EnumerateElementId> for EnumerateElement {
    fn id(&self) -> EnumerateElementId {
        self.id
    }

    fn set_id(&mut self, id: EnumerateElementId) {
        self.id = id
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl WithPosition for EnumerateElement {
    fn position(&self) -> &Option<Position> {
        &self.position
    }
}

impl std::fmt::Display for EnumerateElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl ToLang for EnumerateElement {
    fn to_lang(&self, _: &super::Model) -> String {
        self.name.clone()
    }
}

impl ToDebug for EnumerateElement {
    fn to_debug(&self, _: &super::Model) -> String {
        format!("{}/* {:?} */", self.name, self.id)
    }
}
