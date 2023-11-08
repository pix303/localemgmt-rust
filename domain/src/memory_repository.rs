use std::collections::HashMap;

use uuid::Uuid;

use crate::model::{TranslationDto, TranslationItem};
use crate::repository::TranslationRepository;

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
        content_item: TranslationDto,
        user: String,
    ) -> Result<TranslationItem, String> {
        let item = TranslationItem::new(&content_item, &user);
        let item_hash = item.get_content_hash();
        // search for same content by hash
        self.content_set.insert(item_hash.clone(), item.id);
        self.items.insert(item.id, item.clone());
        return Ok(item);
    }
}
