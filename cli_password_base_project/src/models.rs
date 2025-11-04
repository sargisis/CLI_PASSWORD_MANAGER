pub struct Entry {
    pub service: String,
    pub login: String,
    pub password: String,
}

impl Entry {
    pub fn new(service: String, login: String, password: String) -> Self {
        Self { service, login, password }
    }
}