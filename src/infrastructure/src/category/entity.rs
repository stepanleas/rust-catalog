use chrono::NaiveDateTime;
use diesel::prelude::*;
use domain::Category;
use shared::domain::value_objects::CategoryId;
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable, Insertable, AsChangeset, PartialEq, Debug)]
#[diesel(table_name = crate::schema::categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(crate) struct CategoryEntity {
    id: Uuid,
    title: String,
    description: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl From<Category> for CategoryEntity {
    fn from(category: Category) -> Self {
        Self {
            id: category.id().into(),
            title: category.title().into(),
            description: category.description().into(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

impl From<CategoryEntity> for Category {
    fn from(entity: CategoryEntity) -> Self {
        Category::new(
            CategoryId::from_uuid(entity.id),
            entity.title,
            entity.description,
        )
    }
}
