#[cfg(test)]
mod tests {
    use crate::entities::{Category, Product};
    use shared::domain::value_objects::{CategoryId, Money, ProductId};

    #[test]
    fn test_new_category() {
        let category_id = CategoryId::new();
        let category = Category::new(
            category_id,
            "Category title".into(),
            "Category description".into(),
        );

        assert_eq!(category.id(), category_id);
        assert_eq!(category.title(), "Category title");
        assert_eq!(category.description(), "Category description");
    }

    #[test]
    fn test_new_product() {
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

        assert_eq!(product.id(), product_id);
        assert_eq!(product.title(), "Product title");
        assert_eq!(product.description(), "Product description");
        assert_eq!(product.quantity(), 5);
        assert_eq!(product.price(), &Money::from_f64(15.5).unwrap());

        assert_eq!(product.category().id(), category_id);
        assert_eq!(product.category().title(), "Category title");
        assert_eq!(product.category().description(), "Category description");
    }
}
