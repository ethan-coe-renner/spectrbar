pub struct Bar(Vec<Widget>);

impl fmt::Display for Bar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bar: String = self.0.iter().map(|x| x.to_string() + " | ").collect();

        write!(f, "{}", bar)
    }
}

impl Bar {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add_widget(&mut self, id: &'static str, retriever: DataRetriever, is_num: bool) {
        self.0.push(Widget::new(id, retriever, is_num));
    }
}
