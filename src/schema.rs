// @generated automatically by Diesel CLI.

diesel::table! {
    assets (id) {
        id -> Uuid,
        content_id -> Uuid,
        image_url -> Text,
    }
}

diesel::table! {
    content (id) {
        id -> Uuid,
        title -> Text,
        description -> Text,
        author -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Text,
        username -> Text,
        full_name -> Text,
        is_staff -> Bool,
        is_active -> Bool,
        date_joined -> Timestamp,
        last_login -> Nullable<Timestamp>,
        updated_at -> Timestamp,
        password -> Text,
    }
}

diesel::joinable!(assets -> content (content_id));
diesel::joinable!(content -> users (author));

diesel::allow_tables_to_appear_in_same_query!(
    assets,
    content,
    users,
);
