use crate::http_models;



pub fn request_cat() -> http_models::CatFact {
    let cat = reqwest::blocking::get("https://catfact.ninja/fact")
        .expect("Error while requesting fact")
        .json::<http_models::CatFact>().expect("Error");
    cat
}

