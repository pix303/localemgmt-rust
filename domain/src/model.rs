use std::{collections::hash_map::DefaultHasher, fmt::Display, hash::Hasher};

use chrono::prelude::*;
use uuid::Uuid;

#[derive(Clone)]
pub struct TranslationItem {
    pub id: Uuid,
    pub content: String,
    pub lang: String,
    pub context: String,
    pub created_by: String,
    pub created_at: String,
    pub updated_by: String,
    pub updated_at: String,
}

pub struct TranslationDto {
    pub content: String,
    pub lang: String,
    pub context: String,
}

impl TranslationItem {
    pub fn new(content_item: &TranslationDto, user: &String) -> TranslationItem {
        let current_date: DateTime<Utc> = Utc::now();
        let result = TranslationItem {
            id: Uuid::new_v4(),
            content: content_item.content.clone(),
            lang: content_item.lang.clone(),
            context: content_item.context.clone(),
            created_by: user.clone(),
            created_at: current_date.to_rfc3339(),
            updated_by: user.clone(),
            updated_at: current_date.to_rfc3339(),
        };

        return result;
    }

    pub fn get_content_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        let content = format!("{}{}{}", self.lang, self.context, self.content);
        hasher.write(&content.as_bytes());
        return hasher.finish();
    }
}

impl Display for TranslationItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} - {}]: {}", self.context, self.lang, self.content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_dto() -> TranslationDto {
        let dto = TranslationDto {
            content: "test".to_string(),
            context: "test".to_string(),
            lang: "it".to_string(),
        };
        dto
    }

    fn get_item() -> TranslationItem {
        let item = TranslationItem::new(&get_dto(), &"test".to_string());
        item
    }

    #[test]
    fn new_translation_item() {
        let item = get_item();
        assert_eq!(item.content, "test".to_string());
        assert_eq!(item.context, "test".to_string());
        assert_eq!(item.lang, "it".to_string());
        assert_eq!(item.created_by, "test".to_string());
        assert_eq!(item.updated_by, "test".to_string());
        assert_eq!(item.updated_by, item.created_by);
        assert_ne!(item.id.to_string().len(), 0);
    }

    #[test]
    fn translation_hash() {
        let mut test_hasher = DefaultHasher::new();
        test_hasher.write("ittesttest".as_bytes());
        let test_hash = test_hasher.finish();

        let item = get_item();
        let hash = item.get_content_hash();
        assert_eq!(hash, test_hash);
    }

    #[test]
    fn translation_display() {
        let item = get_item();
        let item_display = format!("{}", item);
        assert_eq!(item_display, "[test - it]: test");
    }
}
