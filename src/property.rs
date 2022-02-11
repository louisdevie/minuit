/// A string property
pub struct StringProperty {
    value: String,
    changed: bool,
}

impl StringProperty {
    /// Creates a new StringProperty.
    /// # Arguments
    /// * `value` : the initial value of the property
    pub fn initial(value: &str) -> Self {
        Self {
            value: String::from(value),
            changed: true,
        }
    }

    /// Check if the property was modified.
    /// # Returns
    /// A boolean, wether the property was modified or not
    pub fn has_changed(&self) -> bool {
        self.changed
    }

    /// Mark the property as handled.
    /// The property won't be considered as modified until `set` is called again.
    pub fn handled(&mut self) {
        self.changed = false;
    }

    /// Get a *copy* of the value of the property
    pub fn get(&self) -> String {
        String::from(&self.value)
    }

    /// Modify the value of the property.
    /// The property will be considered as modified until `handled` is called again.
    pub fn set(&mut self, value: &str) {
        self.value = String::from(value);
    }
}
