mod model;

mod repository;
use repository::TranslationRepository;

mod memory_repository;
use memory_repository::*;

use crate::model::TranslationDto;

pub fn raw_test() {
    let mut repo = MemoryRepository::new();

    let tdto: TranslationDto = TranslationDto {
        content: "primo contenuto".to_string(),
        lang: "it".to_string(),
        context: "default".to_string(),
    };

    let insert_result = repo.add_translation(tdto, "pix@gmail.com".to_string());
    println!("{}", insert_result.unwrap());
}
