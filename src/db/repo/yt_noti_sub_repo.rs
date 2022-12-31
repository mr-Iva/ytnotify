use crate::db::dto::yt_noti_sub::NewYtNotiSub;

pub struct YtNotiSubRepo {}

impl YtNotiSubRepo {
    pub fn create(
        new_yt_noti_sub: &NewYtNotiSub,
        pg_connection: &mut PgConnection,
    ) -> Result<QueryYtNotificationSubscription, RepoError> {
        let insert_yt_notification_subscription = InsertYtNotificationSubscription {
            yt_channel_name: new_yt_noti_sub.channel_name.clone(),
            yt_channel_id: new_yt_noti_sub.channel_id.clone(),
            profile_id: new_yt_noti_sub.profile_id,
        };
        let subscription = diesel::insert_into(yt_notification_subscription::table)
            .values(&insert_yt_notification_subscription)
            .on_conflict(yt_notification_subscription::yt_channel_id)
            .do_update()
            .set(&insert_yt_notification_subscription)
            .get_result::<QueryYtNotificationSubscription>(pg_connection)?;
        Ok(subscription)
    }
}
