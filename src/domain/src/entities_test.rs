#[cfg(test)]
mod tests {
    use crate::{Category, Product};
    use shared::domain::value_objects::Money;
    use std::str::FromStr;
    use uuid::Uuid;

    #[test]
    fn test_new_category() {
        let category_id: Uuid =
            Uuid::from_str("65d95811-447c-4dcf-a79a-5e1845d48638".into()).unwrap();
        let category = Category::new(
            category_id,
            "Category title".into(),
            "Category description".into(),
        );

        assert_eq!(
            category.id(),
            Uuid::from_str("65d95811-447c-4dcf-a79a-5e1845d48638".into()).unwrap()
        );
        assert_eq!(category.title(), "Category title");
        assert_eq!(category.description(), "Category description");
    }

    #[test]
    fn test_new_product() {
        let category_id: Uuid =
            Uuid::from_str("19b0755d-6ca3-4e4f-809c-9b92695d2929".into()).unwrap();
        let category = Category::new(
            category_id,
            "Category title".into(),
            "Category description".into(),
        );

        let product_id: Uuid =
            Uuid::from_str("65d95811-447c-4dcf-a79a-5e1845d48638".into()).unwrap();
        let product = Product::new(
            product_id,
            "Product title".into(),
            "Product description".into(),
            5,
            Money::from_f64(15.5).unwrap(),
            category,
        );

        assert_eq!(
            product.id(),
            Uuid::from_str("65d95811-447c-4dcf-a79a-5e1845d48638".into()).unwrap()
        );
        assert_eq!(product.title(), "Product title");
        assert_eq!(product.description(), "Product description");
        assert_eq!(product.quantity(), 5);
        assert_eq!(product.price(), &Money::from_f64(15.5).unwrap());
        assert_eq!(
            product.category().id(),
            Uuid::from_str("19b0755d-6ca3-4e4f-809c-9b92695d2929".into()).unwrap()
        );
        assert_eq!(product.category().title(), "Category title");
        assert_eq!(product.category().description(), "Category description");
    }
}
