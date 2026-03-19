use domain::product::entity::Product;
use mockall::automock;
use shared::domain::value_objects::ProductId;

#[automock]
pub trait ProductRepository {
    fn list_all(&self) -> anyhow::Result<Vec<Product>>;
    fn find_by_id(&self, entity_id: ProductId) -> anyhow::Result<Product>;
    fn save(&self, entity: Product) -> anyhow::Result<Product>;
    fn delete(&self, entity_id: ProductId) -> anyhow::Result<()>;
}
