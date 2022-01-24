use serde::{Deserialize};
#[derive(Deserialize)]
pub struct CatFact {
    pub fact: String,
    pub length: i32
}