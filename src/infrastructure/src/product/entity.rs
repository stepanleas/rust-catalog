use crate::category::entity::CategoryEntity;
use chrono::NaiveDateTime;
use diesel::internal::derives::multiconnection::bigdecimal::BigDecimal;
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable, Selectable};
use domain::Product;
use shared::domain::value_objects::Money;
use uuid::Uuid;

#[derive(
    Queryable, Selectable, Identifiable, Associations, Insertable, AsChangeset, PartialEq, Debug,
)]
#[diesel(belongs_to(CategoryEntity, foreign_key=category_id))]
#[diesel(table_name = crate::schema::products)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(crate) struct ProductEntity {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub quantity: i32,
    pub price: BigDecimal,
    pub category_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl ProductEntity {
    pub fn into(self, category_entity: CategoryEntity) -> Product {
        Product {
            id: self.id,
            title: self.title,
            description: self.description,
            quantity: self.quantity,
            price: Money::new(self.price),
            category: category_entity.into(),
        }
    }
}

impl From<Product> for ProductEntity {
    fn from(product: Product) -> Self {
        ProductEntity {
            id: product.id,
            title: product.title,
            description: product.description,
            quantity: product.quantity,
            price: product.price.value(),
            category_id: product.category.id,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
