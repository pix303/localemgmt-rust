use crate::model::{TranslationDto, TranslationItem};

pub trait TranslationRepository {
    fn add_translation(
        &mut self,
        content_item: TranslationDto,
        user: String,
    ) -> Result<TranslationItem, String>;

    // fn update_translation(
    //     &mut self,
    //     id: Uuid,
    //     content_item: TranslationDto,
    //     user: String,
    // ) -> Result<TranslationItem, String>;

    // fn get_translation_by_id(&self, id: Uuid) -> Result<TranslationItem, String>;

    // fn get_translation_by_content(&self, content_item: TranslationDto) -> Option<TranslationItem>;

    // fn remove_translation(&self, id: Uuid) -> Result<TranslationItem, String>;
}
