use crate::DbPool;
use crate::category::entity::CategoryEntity;
use crate::schema::categories::dsl::categories;
use crate::schema::categories::id;
use application::CategoryRepository;
use diesel::ExpressionMethods;
use diesel::{OptionalExtension, QueryDsl, RunQueryDsl};
use domain::{Category, DomainError};
use shared::domain::value_objects::CategoryId;

pub struct PostgresCategoryRepository {
    pool: DbPool,
}

impl PostgresCategoryRepository {
    pub fn new(pool: &DbPool) -> Self {
        Self { pool: pool.clone() }
    }
}

impl CategoryRepository for PostgresCategoryRepository {
    fn list_all(&self) -> anyhow::Result<Vec<Category>> {
        let mut connection = self.pool.get()?;
        let items: Vec<CategoryEntity> = categories.load(&mut connection)?;

        Ok(items.into_iter().map(CategoryEntity::into).collect())
    }

    fn find_by_id(&self, entity_id: CategoryId) -> anyhow::Result<Category> {
        let mut connection = self.pool.get()?;

        let category_entity = categories
            .filter(id.eq(entity_id.as_uuid()))
            .first::<CategoryEntity>(&mut connection)
            .optional()?
            .ok_or(DomainError::NotFound {
                message: format!("Could not find category with id: {}", entity_id.as_uuid()),
            })?;

        Ok(category_entity.into())
    }

    fn save(&self, entity: Category) -> anyhow::Result<Category> {
        let mut connection = self.pool.get()?;

        let persistent_entity = CategoryEntity::from(entity);

        diesel::insert_into(categories)
            .values(&persistent_entity)
            .on_conflict(id)
            .do_update()
            .set(&persistent_entity)
            .execute(&mut connection)?;

        Ok(persistent_entity.into())
    }

    fn delete(&self, entity_id: CategoryId) -> anyhow::Result<()> {
        let mut connection = self.pool.get()?;
        diesel::delete(categories.filter(id.eq(entity_id.as_uuid()))).execute(&mut connection)?;

        Ok(())
    }
}
