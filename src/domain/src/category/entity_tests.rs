#[cfg(test)]
mod tests {
    use crate::category::entity::Category;
    use shared::domain::value_objects::CategoryId;

    #[test]
    fn test_new_category() -> anyhow::Result<()> {
        let category_id = CategoryId::new();
        let category = Category::new(
            category_id,
            "Category title".into(),
            "Category description".into(),
        );

        assert_eq!(category_id, category.id());
        assert_eq!("Category title", category.title());
        assert_eq!("Category description", category.description());

        Ok(())
    }
}
