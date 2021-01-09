table! {
    users (id) {
        id -> Nullable<Int4>,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        created_at -> Timestamp,
    }
}
