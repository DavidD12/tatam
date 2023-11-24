use crate::model::Model;

pub struct Parser<'a> {
    current: Option<String>,
    todo: Vec<String>,
    done: Vec<String>,
    pub model: &'a mut Model,
}

impl<'a> Parser<'a> {
    pub fn new(model: &'a mut Model) -> Self {
        Self {
            current: None,
            todo: vec![],
            done: vec![],
            model,
        }
    }

    pub fn current(&self) -> &Option<String> {
        &self.current
    }

    pub fn file(&self) -> &str {
        self.current.as_ref().unwrap()
    }

    pub fn todo(&self) -> &Vec<String> {
        &self.todo
    }

    pub fn done(&self) -> &Vec<String> {
        &self.done
    }

    //------------------------- -------------------------

    pub fn files(&self) -> Vec<String> {
        let mut v = vec![];
        if let Some(f) = &self.current {
            v.push(f.clone());
        }
        v.extend(self.todo.clone().into_iter());
        v.extend(self.done.clone().into_iter());
        v
    }

    pub fn add<S: Into<String>>(&mut self, file: S) -> bool {
        let file: String = file.into();
        if self.files().contains(&file) {
            false
        } else {
            self.todo.push(file);
            true
        }
    }

    pub fn next(&mut self) -> Option<String> {
        if let Some(file) = &self.current {
            self.done.push(file.clone());
        }
        match self.todo.pop() {
            None => None,
            Some(file) => {
                self.current = Some(file.clone());
                Some(file)
            }
        }
    }
}
