use crate::schema::categories::dsl::categories;
use crate::schema::products::dsl::products;

use crate::DbPool;
use crate::category::entity::CategoryEntity;
use crate::product::entity::ProductEntity;
use application::ProductRepository;
use diesel::{ExpressionMethods, SelectableHelper};
use diesel::{OptionalExtension, QueryDsl, RunQueryDsl};
use domain::{DomainError, Product};
use uuid::Uuid;

pub struct PostgresProductRepository {
    pool: DbPool,
}

impl PostgresProductRepository {
    pub fn new(pool: &DbPool) -> Self {
        Self { pool: pool.clone() }
    }
}

impl ProductRepository for PostgresProductRepository {
    fn list_all(&self) -> anyhow::Result<Vec<Product>> {
        let mut connection = self.pool.get()?;

        let results: Vec<(ProductEntity, CategoryEntity)> = products
            .inner_join(categories)
            .select((ProductEntity::as_select(), CategoryEntity::as_select()))
            .load::<(ProductEntity, CategoryEntity)>(&mut connection)?;

        Ok(results
            .into_iter()
            .map(|(product_entity, category_entity)| product_entity.into_domain(category_entity))
            .collect())
    }

    fn find_by_id(&self, entity_id: Uuid) -> anyhow::Result<Product> {
        let mut connection = self.pool.get()?;

        let (product_entity, category_entity) = products
            .inner_join(categories)
            .filter(crate::schema::products::id.eq(entity_id))
            .select((ProductEntity::as_select(), CategoryEntity::as_select()))
            .first::<(ProductEntity, CategoryEntity)>(&mut connection)
            .optional()?
            .ok_or(DomainError::NotFound { id: entity_id })?;

        Ok(product_entity.into_domain(category_entity))
    }

    fn save(&self, entity: Product) -> anyhow::Result<Product> {
        let mut connection = self.pool.get()?;

        let persistent_entity = ProductEntity::from(entity);
        let product_entity = diesel::insert_into(products)
            .values(&persistent_entity)
            .on_conflict(crate::schema::products::id)
            .do_update()
            .set(&persistent_entity)
            .returning(ProductEntity::as_returning())
            .get_result::<ProductEntity>(&mut connection)?;

        let category_entity = categories
            .find(product_entity.category_id())
            .first::<CategoryEntity>(&mut connection)?;

        Ok(product_entity.into_domain(category_entity))
    }

    fn delete(&self, entity_id: Uuid) -> anyhow::Result<()> {
        let mut connection = self.pool.get()?;
        diesel::delete(products.filter(crate::schema::products::id.eq(entity_id)))
            .execute(&mut connection)?;

        Ok(())
    }
}
