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
