// @generated automatically by Diesel CLI.

diesel::table! {
    chat_messages (id) {
        id -> Int8,
        chat_id -> Int8,
        user_id -> Int8,
        content -> Text,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    chat_users (chat_id, user_id) {
        chat_id -> Int8,
        user_id -> Int8,
    }
}

diesel::table! {
    chats (id) {
        id -> Int8,
        name -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        username -> Text,
        name -> Text,
        hashed_password -> Text,
        status -> Nullable<Text>,
        bio -> Nullable<Text>,
        is_banned -> Nullable<Bool>,
        is_active -> Nullable<Bool>,
    }
}

diesel::joinable!(chat_messages -> chats (chat_id));
diesel::joinable!(chat_messages -> users (user_id));
diesel::joinable!(chat_users -> chats (chat_id));
diesel::joinable!(chat_users -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    chat_messages,
    chat_users,
    chats,
    users,
);
