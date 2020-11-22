table! {
    profiles (id) {
        id -> Int4,
        username -> Varchar,
        bio -> Nullable<Text>,
        profile_picture -> Nullable<Varchar>,
        user_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password -> Varchar,
    }
}

joinable!(profiles -> users (user_id));

allow_tables_to_appear_in_same_query!(profiles, users,);
