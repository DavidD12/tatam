use crate::parser::Position;
use crate::{error::*, ToDebug};
use crate::{Id, Named, ToLang, WithPosition};

//------------------------- Interval Id -------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct IntervalId(pub usize);

impl Id for IntervalId {
    fn empty() -> Self {
        Self(0)
    }
    fn index(&self) -> usize {
        self.0
    }
}

//------------------------- Interval -------------------------

#[derive(Clone)]
pub struct Interval {
    id: IntervalId,
    name: String,
    min: i64,
    max: i64,
    position: Option<Position>,
}

impl Interval {
    pub fn new<S: Into<String>>(name: S, position: Option<Position>, min: i64, max: i64) -> Self {
        let id = IntervalId::empty();
        let name = name.into();
        Self {
            id,
            name,
            min,
            max,
            position,
        }
    }

    pub fn min(&self) -> i64 {
        self.min
    }
    pub fn max(&self) -> i64 {
        self.max
    }

    pub fn check_interval(&self) -> Result<(), Error> {
        if self.min > self.max {
            Err(Error::Interval {
                name: self.name.clone(),
                position: self.position.clone(),
            })
        } else {
            Ok(())
        }
    }
}

impl Named<IntervalId> for Interval {
    fn id(&self) -> IntervalId {
        self.id
    }

    fn set_id(&mut self, id: IntervalId) {
        self.id = id;
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl WithPosition for Interval {
    fn position(&self) -> &Option<Position> {
        &self.position
    }
}

impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl ToLang for Interval {
    fn to_lang(&self, _: &super::Model) -> String {
        format!("interval {} = {}..{}", self.name(), self.min, self.max)
    }
}

impl ToDebug for Interval {
    fn to_debug(&self, _: &super::Model) -> String {
        let mut res = format!("// {:?}\n", self.id);
        res.push_str(&format!(
            "interval {} = {}..{}",
            self.name(),
            self.min,
            self.max
        ));
        res
    }
}
