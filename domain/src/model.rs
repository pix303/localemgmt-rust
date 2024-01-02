use std::{collections::hash_map::DefaultHasher, fmt::Display, hash::Hasher};

use chrono::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct TranslationContent {
    pub content: String,
    pub lang: String,
    pub context: String,
}

impl TranslationContent {
    pub fn new(content: &str, context: &str, lang: &str) -> TranslationContent {
        let tc = TranslationContent {
            content: content.to_string(),
            context: context.to_string(),
            lang: lang.to_string(),
        };
        tc.clone()
    }

    pub fn get_complete_content(&self) -> String {
        format!("[{} - {}]: {}", self.context, self.lang, self.content)
    }

    pub fn get_content_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        let cc = self.get_complete_content();
        hasher.write(cc.as_bytes());
        hasher.finish()
    }
}

#[derive(Clone, Debug)]
pub struct TranslationItem {
    pub id: Uuid,
    pub translation: TranslationContent,
    pub created_by: String,
    pub created_at: String,
    pub updated_by: String,
    pub updated_at: String,
}

impl TranslationItem {
    pub fn new(content_item: &TranslationContent, user: &str) -> TranslationItem {
        let current_date: DateTime<Utc> = Utc::now();
        TranslationItem {
            id: Uuid::new_v4(),
            translation: content_item.clone(),
            created_by: user.to_string(),
            created_at: current_date.to_rfc3339(),
            updated_by: user.to_string(),
            updated_at: current_date.to_rfc3339(),
        }
    }

    pub fn get_content_hash(&self) -> u64 {
        self.translation.get_content_hash()
    }
}

impl Display for TranslationItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.translation.get_complete_content())
    }
}

#[allow(dead_code)]
pub mod test_utils {

    use super::*;

    pub fn get_content_item(content: &str, context: &str, lang: &str) -> TranslationContent {
        TranslationContent::new(content, context, lang)
    }

    pub fn get_basic_testable_content_item() -> TranslationContent {
        get_content_item("test", "test", "it")
    }

    pub fn get_basic_testable_item() -> TranslationItem {
        TranslationItem::new(&get_basic_testable_content_item(), &get_user())
    }

    pub fn get_user() -> String {
        "user@mail.com".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::*;

    #[test]
    fn new_translation_item() {
        let item = get_basic_testable_item();
        assert_eq!(item.translation.content, "test".to_string());
        assert_eq!(item.translation.context, "test".to_string());
        assert_eq!(item.translation.lang, "it".to_string());
        assert_eq!(item.created_by, "user@mail.com".to_string());
        assert_eq!(item.updated_by, "user@mail.com".to_string());
        assert_eq!(item.updated_by, item.created_by);
        assert_ne!(item.id.to_string().len(), 0);
    }

    #[test]
    fn translation_hash() {
        let mut test_hasher = DefaultHasher::new();
        test_hasher.write("[test - it]: test".as_bytes());
        let test_hash = test_hasher.finish();

        let item = get_basic_testable_item();
        let hash = item.get_content_hash();
        assert_eq!(hash, test_hash);
    }

    #[test]
    fn translation_display() {
        let item = get_basic_testable_item();
        let item_display = format!("{}", item);
        assert_eq!(item_display, "[test - it]: test");
    }
}
