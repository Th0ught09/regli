#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct ProcessedLine<T> {
    pub item: T,
    selected: bool,
    is_match: bool,
}

impl<T: Clone> ProcessedLine<T> {
    pub fn with_item(item: T) -> Self {
        Self {
            item,
            selected: false,
            is_match: false,
        }
    }
    pub fn selected(&mut self) {
        self.selected = true;
    }
    pub fn unselected(&mut self) {
        self.selected = false;
    }
    pub fn matches(&mut self) {
        self.is_match = true;
    }
    pub fn missed(&mut self) {
        self.is_match = false;
    }
    pub fn has_matched(&self) -> bool {
        self.is_match
    }
    pub fn set_item(&mut self, item: T) {
        self.item = item;
    }
    pub fn get_item(&self) -> T {
        self.item.clone()
    }
}
