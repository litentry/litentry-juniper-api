#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    #[serde(rename = "_id")]
    pub id: i32,

    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub slug: String,

    #[serde(default)]
    pub tp: i32,

    #[serde(default)]
    pub qty: i32,

    #[serde(default)]
    pub price: i32,

    #[serde(default)]
    pub width: i32,

    #[serde(default)]
    pub height: i32,

    #[serde(default)]
    pub depth: i32,

    #[serde(default)]
    pub weight: i32,

    #[serde(default)]
    pub description: String,
}

//graphql_object!(Product: Context |&self| {
//    field id() -> String { if let Some(ref id) = self.id { id.to_hex() } else { "".into() } }
//    field name() -> &str { self.name.as_str() }
//    field slug() -> &str { self.slug.as_str() }
//    field tp() -> i32 { self.tp }
//    field qty() -> i32 { self.qty }
//    field price() -> i32 { self.price }
//    field width() -> i32 { self.width }
//    field height() -> i32 { self.height }
//    field depth() -> i32 { self.depth }
//    field weight() -> i32 { self.weight }
//    field description() -> &str { self.description.as_str() }
//});

//graphql_object!(Mutations: Context |&self| {
//    field saveProduct(&executor,
//        id: Option<String>,
//        name: String,
//        slug: Option<String>,
//        tp: i32,
//        qty: i32,
//        price: i32,
//        width: Option<i32>,
//        height: Option<i32>,
//        depth: Option<i32>,
//        weight: Option<i32>,
//        description: Option<String>,
//    ) -> FieldResult<Option<Product>> {
//        let context = executor.context();
//        let id = id.map(|id| ObjectId::with_string(&id)).map_or(Ok(None), |v| v.map(Some))?;
//
//        let product = Product {
//            id: id,
//            name: name,
//            slug: slug.unwrap_or_else(|| "".into()),
//            tp: tp,
//            qty: qty,
//            price: price,
//            width: width.unwrap_or(0),
//            height: height.unwrap_or(0),
//            depth: depth.unwrap_or(0),
//            weight: weight.unwrap_or(0),
//            description: description.unwrap_or_else( || "".into()),
//        };
//
//        Ok(context.db.save_product(product)?)
//    }
// });



