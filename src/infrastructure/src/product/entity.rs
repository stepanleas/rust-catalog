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
    id: Uuid,
    category_id: Uuid,
    title: String,
    description: String,
    quantity: i32,
    price: BigDecimal,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl ProductEntity {
    pub fn into_domain(self, category_entity: CategoryEntity) -> Product {
        Product::new(
            self.id,
            self.title,
            self.description,
            self.quantity,
            Money::new(self.price),
            category_entity.into(),
        )
    }

    pub fn category_id(&self) -> Uuid {
        self.category_id
    }
}

impl From<Product> for ProductEntity {
    fn from(product: Product) -> Self {
        ProductEntity {
            id: product.id(),
            category_id: product.category().id(),
            title: product.title().into(),
            description: product.description().into(),
            quantity: product.quantity(),
            price: product.price().clone().value(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
