table! {
    profiles (profileid) {
        profileid -> Int4,
        profilename -> Text,
        created_at -> Timestamp,
        userr_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        created_at -> Timestamp,
    }
}

joinable!(profiles -> users (userr_id));

allow_tables_to_appear_in_same_query!(
    profiles,
    users,
);
