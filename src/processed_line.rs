#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct ProcessedLine<T> {
    pub item: T,
    pub selected: bool,
}

impl<T: Clone> ProcessedLine<T> {
    pub fn with_item(item: T) -> Self {
        Self {
            item,
            selected: false,
        }
    }
    pub fn selected(&mut self) {
        self.selected = true;
    }
    pub fn unselected(&mut self) {
        self.selected = false;
    }
    pub fn set_item(&mut self, item: T) {
        self.item = item;
    }
    pub fn get_item(&self) -> T {
        self.item.clone()
    }
}
