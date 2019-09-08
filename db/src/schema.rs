table! {
    users (id) {
        id -> Integer,
        name -> Text,
        address -> Text,
        public_key -> Text,
        balance -> Text,
    }
}

table! {
    identities (id) {
        id -> Integer,
        owner_id -> Integer,
        identity_hash -> Text,
    }
}

table! {
    tokens (id) {
        id -> Integer,
        owner_id -> Integer,
        identity_id -> Integer,
        token_hash -> Text,
        cost -> Text,
        data -> Text,
        data_type -> Text,
        expired -> Text,
    }
}

table! {
    litentry_index (id) {
        id -> Integer,
        identity_index -> Integer,
        token_index -> Integer,
    }
}

table! {
    new_identity_event (id) {
        id -> Integer,
        block_hash -> Text,
        address -> Text,
        identity_hash -> Text,
    }
}

table! {
    new_token_event (id) {
        id -> Integer,
        block_hash -> Text,
        owner_address -> Text,
        identity_hash -> Text,
        token_hash -> Text,
    }
}

joinable!(tokens -> users (owner_id));
joinable!(tokens -> identities (identity_id));
joinable!(identities -> users (owner_id));
allow_tables_to_appear_in_same_query!(
    tokens,
    users,
    identities,
);

