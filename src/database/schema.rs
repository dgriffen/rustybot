table! {
    factoids (id) {
        id -> Integer,
        label -> Text,
        intent -> Text,
        description -> Text,
        nickname -> Text,
        timestamp -> Timestamp,
        locked -> Bool,
    }
}

table! {
    qotd (id) {
        id -> Integer,
        quote -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    factoids,
    qotd,
);
