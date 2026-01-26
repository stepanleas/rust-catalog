#[cfg(test)]
mod tests {
    use crate::entities::{Category, Product};
    use shared::domain::value_objects::{CategoryId, Money, ProductId};

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

    #[test]
    fn test_new_product() -> anyhow::Result<()> {
        let category_id = CategoryId::new();
        let category = Category::new(
            category_id,
            "Category title".into(),
            "Category description".into(),
        );

        let product_id = ProductId::new();
        let product = Product::new(
            product_id,
            "Product title".into(),
            "Product description".into(),
            5,
            Money::from_f64(15.5).unwrap(),
            category,
        );

        assert_eq!(product_id, product.id());
        assert_eq!("Product title", product.title());
        assert_eq!("Product description", product.description());
        assert_eq!(5, product.quantity());
        assert_eq!("15.5", product.price().to_string());

        assert_eq!(category_id, product.category().id());
        assert_eq!("Category title", product.category().title());
        assert_eq!("Category description", product.category().description());

        Ok(())
    }
}
