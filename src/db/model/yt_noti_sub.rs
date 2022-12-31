use crate::db::model::profile::Profile;
use crate::db::schema::yt_notification_subscription;
use diesel_derives::{AsChangeset, Identifiable, Insertable, Queryable};

pub struct QueryYtNotificationSubscription {
    pub id: i32,
    pub yt_channel_name: String,
    pub yt_channel_id: String,
    pub profile_id: i32,
}
