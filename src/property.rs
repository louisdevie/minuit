pub struct Property<T> {
    value: T,
    changed: bool,
}

impl<T> Property<T> {
    pub fn initial(value: T) -> Self {
        Self {
            value,
            changed: true,
        }
    }

    pub fn has_changed(&self) -> bool {
        self.changed
    }
}
