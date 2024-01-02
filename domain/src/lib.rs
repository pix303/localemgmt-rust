mod model;

mod repository;

mod memory_repository;
use memory_repository::*;

use crate::{model::TranslationContent, repository::TranslationRepository};

pub fn raw_test() {
    let mut repo = MemoryRepository::new();

    let tdto = TranslationContent::new("primo contenuto", "it", "default");
    let insert_result = repo.add_translation(tdto, "pix@gmail.com".to_string());
    println!("{}", insert_result.unwrap());
}
