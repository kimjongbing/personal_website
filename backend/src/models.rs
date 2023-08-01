use serde::Serialize;

#[derive(Serialize)]
pub struct Blog {
    pub name: String,
    pub path: String,
}
