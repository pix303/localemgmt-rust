use std::collections::HashMap;

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
        return repo;
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

        self.content_set.insert(item_hash.clone(), item.id);
        self.items.insert(item.id, item.clone());
        return Ok(item);
    }

    fn get_translation_by_id(&self, id: &Uuid) -> Option<TranslationItem> {
        self.items.get(id).cloned()
    }

    //     fn get_translation_by_content(&self, content_item: TranslationContent) -> Option<TranslationItem[]> {
    //         let hash_key = content_item.get_content_hash();

    // let result =         self.content_set.get(&hash_key);

    //     }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::test_utils::*;

    #[test]
    fn add_translation_to_repository() {
        let mut repo = MemoryRepository::new();
        let result = repo.add_translation(get_content_item(), get_user());
        assert_eq!(result.unwrap().content.content, "test");
        let result = repo.add_translation(get_content_item(), get_user());
        assert!(matches!(
            result,
            Err(TranslationRepositoryError::ContentAlreadyPresent)
        ))
    }
}
