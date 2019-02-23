table! {
    item_groups (layer) {
        layer -> Integer,
        name -> Text,
        description -> Nullable<Text>,
    }
}

table! {
    items (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        layer -> Integer,
    }
}

joinable!(items -> item_groups (layer));

allow_tables_to_appear_in_same_query!(
    item_groups,
    items,
);
