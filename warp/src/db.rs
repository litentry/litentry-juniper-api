use crate::model::Product;

pub struct Db {
    db_name: String,
}

impl Db {
    pub fn new<S>(db_name: S) -> Db
        where
            S: ToString,
    {
        let db_name = db_name.to_string();

        Db { db_name }
    }

    pub fn list_products(&self) -> Vec<Product> {
        let v: Vec<Product> = vec![];
        v
    }

    pub fn get_product(&self, id: i32) -> Option<Product> {
        Some(Product {
            id: 0,
            name: "a".to_owned(),
            slug: "a".to_owned(),
            tp: 0,
            qty: 0,
            price: 0,
            width: 0,
            height: 0,
            depth: 0,
            weight: 0,
            description: "a".to_owned(),
        })
    }
}
