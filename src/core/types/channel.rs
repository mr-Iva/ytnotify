pub struct Channel {
    pub name: String,
    pub kind: ChannelKind,
}

pub enum ChannelKind {
    Youtube(YtChannel),
    Twitch(TwitchChannel),
}

pub struct YtChannel {
    pub id: String,
}

pub struct TwitchChannel {
    pub id: String,
}
