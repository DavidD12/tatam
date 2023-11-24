#[derive(Clone, Copy, Debug)]
pub struct TransitionNumber {
    min: usize,
    max: Option<usize>,
}

impl TransitionNumber {
    pub fn new(min: usize, max: Option<usize>) -> Self {
        Self { min, max }
    }

    pub fn min(&self) -> usize {
        self.min
    }

    pub fn max(&self) -> Option<usize> {
        self.max
    }
}

impl std::fmt::Display for TransitionNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}..", self.min)?;
        if let Some(max) = self.max {
            write!(f, "{}", max)?;
        }
        write!(f, "]")
    }
}
