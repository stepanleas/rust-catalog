#[cfg(test)]
mod tests {
    use crate::category::queries::FindCategoryQuery;
    use uuid::Uuid;

    #[test]
    fn test_find_category_query() {
        let category_id = Option::from(Uuid::new_v4());
        let query = FindCategoryQuery::new(category_id);

        assert_eq!(query.id(), category_id);
    }
}
