use crate::category::entity::CategoryEntity;
use chrono::NaiveDateTime;
use diesel::internal::derives::multiconnection::bigdecimal::BigDecimal;
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable, Selectable};
use domain::entities::Product;
use rusty_money::{Money, iso};
use shared::domain::value_objects::ProductId;
use std::str::FromStr;
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
        let price = Money::from_str(self.price.to_string().as_str(), iso::USD)
            .unwrap_or(Money::from_minor(0, iso::USD));

        Product::builder()
            .id(ProductId::from_uuid(self.id))
            .title(&self.title)
            .description(&self.description)
            .quantity(self.quantity)
            .price(price)
            .category(category_entity.into())
            .build()
    }

    pub fn category_id(&self) -> Uuid {
        self.category_id
    }
}

impl From<Product> for ProductEntity {
    fn from(product: Product) -> Self {
        let price = BigDecimal::from_str(product.price().amount().to_string().as_str())
            .unwrap_or(BigDecimal::from(0));

        ProductEntity {
            id: product.id().into(),
            category_id: product.category().id().into(),
            title: product.title().into(),
            description: product.description().into(),
            quantity: product.quantity(),
            price,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
