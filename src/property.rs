pub struct StringProperty {
    value: String,
    changed: bool,
}

impl StringProperty {
    pub fn initial(value: &str) -> Self {
        Self {
            value: String::from(value),
            changed: true,
        }
    }

    pub fn has_changed(&self) -> bool {
        self.changed
    }

    pub fn get(&self) -> String {
        String::from(&self.value)
    }

    pub fn set(&mut self, value: &str) {
        self.value = String::from(value);
    }
}
