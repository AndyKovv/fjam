table! {
    user_profiles (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        name -> Varchar,
        second_name -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password -> Text,
    }
}

joinable!(user_profiles -> users (user_id));

allow_tables_to_appear_in_same_query!(
    user_profiles,
    users,
);
