pub(crate) struct Summary {
    requests: u64
}

impl Summary {
    pub(crate) fn new() -> Self {
        Self { requests: 0 }
    }
    pub(crate) fn count(self) -> Self {
        Self { requests: self.requests + 1 }
    }
    pub(crate) fn get(&self) -> String {
        String::from(&self.requests.to_string())
    }
}