use uuid::Uuid;

use crate::model::{TranslationContent, TranslationItem};

pub trait TranslationRepository {
    fn add_translation(
        &mut self,
        content_item: TranslationContent,
        user: String,
    ) -> Result<TranslationItem, TranslationRepositoryError>;

    // fn update_translation(
    //     &mut self,
    //     id: Uuid,
    //     content_item: TranslationDto,
    //     user: String,
    // ) -> Result<TranslationItem, String>;

    fn get_translation_by_id(&self, id: &Uuid) -> Option<TranslationItem>;

    // fn get_translation_by_content(&self, content_item: TranslationDto) -> Option<TranslationItem[]>;

    // fn remove_translation(&self, id: Uuid) -> Result<TranslationItem, String>;
}

#[derive(Debug, PartialEq, Eq)]
pub enum TranslationRepositoryError {
    ContentAlreadyPresent,
}

// #[derive(Debug)]
// pub struct TranslationRepositoryError {
//     pub kind: TranslationRepositoryErrorKind,
// }

// impl TranslationRepositoryError {
//     pub fn new(kind: TranslationRepositoryErrorKind) -> TranslationRepositoryError {
//         return TranslationRepositoryError { kind };
//     }
// }

// impl Display for TranslationRepositoryError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self.kind {
//             TranslationRepositoryErrorKind::ContentAlreadyPresent => {
//                 write!(f, "content already present")
//             }
//         }
//     }
// }
