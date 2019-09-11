pub enum LitentryEvents {
    // block_hash -> Text,
    //        address -> Text,
    //        identity_hash -> Text,
    NewIdentity(String, String, String),
    //    block_hash -> Text,
    //owner_address -> Text,
    //identity_hash -> Text,
    //token_hash -> Text,
    NewToken(String, String, String, String),
}
