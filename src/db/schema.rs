// @generated automatically by Diesel CLI.

diesel::table! {
    profile (id) {
        id -> Int4,
        telegram_name -> Varchar,
        telegram_id -> Text,
    }
}

diesel::table! {
    yt_notification_subscription (id) {
        id -> Int4,
        yt_channel_name -> Varchar,
        yt_channel_id -> Varchar,
        profile_id -> Int4,
    }
}

diesel::joinable!(yt_notification_subscription -> profile (profile_id));

diesel::allow_tables_to_appear_in_same_query!(
    profile,
    yt_notification_subscription,
);
