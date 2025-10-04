use chrono::NaiveDateTime;
use diesel::prelude::*;
use domain::Category;
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable, Insertable, AsChangeset, PartialEq, Debug)]
#[diesel(table_name = crate::schema::categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(crate) struct CategoryEntity {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<Category> for CategoryEntity {
    fn from(ticket: Category) -> Self {
        CategoryEntity {
            id: ticket.id,
            title: ticket.title,
            description: ticket.description,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

impl Into<Category> for CategoryEntity {
    fn into(self) -> Category {
        Category {
            id: self.id,
            title: self.title,
            description: self.description,
        }
    }
}
