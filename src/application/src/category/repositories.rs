use domain::Category;
use uuid::Uuid;

pub trait CategoryRepository {
    fn list_all(&self) -> anyhow::Result<Vec<Category>>;
    fn find_by_id(&self, entity_id: Uuid) -> anyhow::Result<Category>;
    fn save(&self, entity: Category) -> anyhow::Result<Category>;
    fn delete(&self, entity_id: Uuid) -> anyhow::Result<()>;
}
