#[derive(Copy, Clone)]
pub(crate) struct Summary {
    requests: u64
}

impl Summary {
    pub(crate) fn new() -> Self {
        Self { requests: 0 }
    }
    pub(crate) fn count(mut self) -> Self {
        self.requests = self.requests + 1;
        self
    }
    pub(crate) fn get(&self) -> String {
        String::from(&self.requests.to_string())
    }
}