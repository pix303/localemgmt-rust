use std::collections::HashMap;

use chrono::Utc;
use uuid::Uuid;

use crate::model::{TranslationContent, TranslationItem};
use crate::repository::{TranslationRepository, TranslationRepositoryError};

pub struct MemoryRepository {
    items: HashMap<Uuid, TranslationItem>,
    content_set: HashMap<u64, Uuid>,
}

impl MemoryRepository {
    pub fn new() -> MemoryRepository {
        let items: HashMap<Uuid, TranslationItem> = HashMap::new();
        let content_set: HashMap<u64, Uuid> = HashMap::new();
        let repo: MemoryRepository = MemoryRepository { items, content_set };
        repo
    }
}

impl TranslationRepository for MemoryRepository {
    fn add_translation(
        &mut self,
        content_item: TranslationContent,
        user: String,
    ) -> Result<TranslationItem, TranslationRepositoryError> {
        let item = TranslationItem::new(&content_item, &user);
        let item_hash = item.get_content_hash();

        if let Some(_id) = self.content_set.get(&item_hash) {
            return Err(TranslationRepositoryError::ContentAlreadyPresent);
        }

        self.content_set.insert(item_hash, item.id);
        self.items.insert(item.id, item.clone());
        Ok(item)
    }

    fn get_translation_by_id(&self, id: &Uuid) -> Option<TranslationItem> {
        self.items.get(id).cloned()
    }

    fn get_translations_by_partial_content(
        &self,
        content_to_search: TranslationContent,
    ) -> Option<Vec<TranslationItem>> {
        let result: Vec<TranslationItem> = self
            .items
            .values()
            .filter(|item| {
                item.translation
                    .content
                    .contains(&content_to_search.content)
                    && item.translation.context == content_to_search.context
                    && (!content_to_search.lang.is_empty()
                        && item.translation.lang == content_to_search.lang)
            })
            .cloned()
            .collect();

        if !result.is_empty() {
            return Some(result);
        }
        None
    }

    fn get_translation_by_exact_match(
        &self,
        content_item: TranslationContent,
    ) -> Option<TranslationItem> {
        let content_hash = content_item.get_content_hash();
        if let Some(result) = self.content_set.get(&content_hash).cloned() {
            let item = self.items.get(&result).cloned();
            item
        } else {
            None
        }
    }

    fn update_translation(
        &mut self,
        id: Uuid,
        content_item: TranslationContent,
        user: String,
    ) -> Result<TranslationItem, TranslationRepositoryError> {
        if let Some(item) = self.items.get_mut(&id) {
            let current_date = Utc::now();
            item.translation.content = content_item.content;
            item.translation.context = content_item.context;
            item.translation.lang = content_item.lang;
            item.updated_by = user;
            item.updated_at = current_date.to_rfc3339();
            return Ok(item.clone());
        }
        Err(TranslationRepositoryError::ItemIdNotPresent)
    }

    fn remove_translation(
        &mut self,
        id: Uuid,
    ) -> Result<TranslationItem, TranslationRepositoryError> {
        if let Some(item) = self.items.remove(&id) {
            let item_hash = item.get_content_hash();
            self.content_set.remove(&item_hash);
            return Ok(item.clone());
        }
        Err(TranslationRepositoryError::ItemIdNotPresent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::test_utils::*;

    fn fill_repository(repo: &mut impl TranslationRepository) {
        let _r = repo.add_translation(get_content_item("the tree", "default", "en"), get_user());
        let _r = repo.add_translation(
            get_content_item("the tree is high", "default", "en"),
            get_user(),
        );
        let _r = repo.add_translation(
            get_content_item("the tree is green", "default", "en"),
            get_user(),
        );
    }

    #[test]
    fn add_translation_to_repository() {
        let mut repo = MemoryRepository::new();
        let result = repo.add_translation(get_basic_testable_content_item(), get_user());
        assert_eq!(result.unwrap().translation.content, "test");
        let result = repo.add_translation(get_basic_testable_content_item(), get_user());
        assert!(matches!(
            result,
            Err(TranslationRepositoryError::ContentAlreadyPresent)
        ))
    }

    #[test]
    fn get_translation_by_content_some() {
        let mut repo = MemoryRepository::new();
        fill_repository(&mut repo);

        let result = repo
            .get_translations_by_partial_content(TranslationContent::new("tree", "default", "en"));
        assert!(&result.is_some());
        assert_eq!(&result.unwrap().len(), &3);

        let result = repo
            .get_translations_by_partial_content(TranslationContent::new("brown", "default", "en"));
        assert!(&result.is_none());
    }

    #[test]
    fn get_translation_by_content_none() {
        let mut repo = MemoryRepository::new();
        fill_repository(&mut repo);

        let result = repo.get_translations_by_partial_content(TranslationContent::new(
            "tree",
            "unknown-context",
            "en",
        ));
        assert!(&result.is_none());
    }

    #[test]
    fn get_tranlation_by_content() {
        let mut repo = MemoryRepository::new();
        fill_repository(&mut repo);

        let result = repo
            .get_translations_by_partial_content(TranslationContent::new("tree", "default", "it"));
        assert!(&result.is_none());
        let result = repo
            .get_translations_by_partial_content(TranslationContent::new("tree", "default", "en"));
        assert!(&result.is_some());
        let result = repo
            .get_translations_by_partial_content(TranslationContent::new("grass", "default", "en"));
        assert!(&result.is_none());
    }

    #[test]
    fn get_translation_by_id() {
        let mut repo = MemoryRepository::new();
        let item = repo.add_translation(get_basic_testable_content_item(), get_user());
        let result = repo.get_translation_by_id(&item.unwrap().id);
        assert_eq!(&result.unwrap().translation.content, "test");
    }

    #[test]
    fn update_translation() {
        let mut repo = MemoryRepository::new();
        fill_repository(&mut repo);
        let item = repo.get_translation_by_exact_match(TranslationContent::new(
            "the tree is green",
            "default",
            "en",
        ));

        let result = repo.update_translation(
            item.unwrap().id,
            TranslationContent::new("the grass is green", "default", "en"),
            "mantainer".to_string(),
        );
        let item = result.unwrap();
        let content = item.translation.content;
        let user = item.updated_by;
        assert_eq!(content, "the grass is green");
        assert_eq!(user, "mantainer");
    }

    #[test]
    fn remove_translation() {
        let mut repo = MemoryRepository::new();
        fill_repository(&mut repo);

        let item = repo.get_translation_by_exact_match(TranslationContent::new(
            "the tree is green",
            "default",
            "en",
        ));

        let result = repo.remove_translation(item.unwrap().id);
        assert!(&result.is_ok());
        let result = repo.remove_translation(Uuid::new_v4());
        assert_eq!(
            result.err().unwrap(),
            TranslationRepositoryError::ItemIdNotPresent
        );
    }
}
