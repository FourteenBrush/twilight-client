use super::prelude::*;
use dawn_model::{
    channel::{permission_overwrite::PermissionOverwrite, ChannelType, GuildChannel},
    id::{ChannelId, GuildId},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct CreateGuildChannel<'a> {
    bitrate: Option<u64>,
    #[serde(rename = "type")]
    kind: Option<ChannelType>,
    nsfw: Option<bool>,
    parent_id: Option<ChannelId>,
    permission_overwrites: Option<Vec<PermissionOverwrite>>,
    position: Option<u64>,
    rate_limit_per_user: Option<u64>,
    topic: Option<String>,
    user_limit: Option<u64>,
    #[serde(skip)]
    fut: Option<Pending<'a, GuildChannel>>,
    #[serde(skip)]
    guild_id: GuildId,
    #[serde(skip)]
    http: &'a Client,
    name: String,
}

impl<'a> CreateGuildChannel<'a> {
    pub(crate) fn new(
        http: &'a Client,
        guild_id: impl Into<GuildId>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            bitrate: None,
            fut: None,
            guild_id: guild_id.into(),
            http,
            kind: None,
            name: name.into(),
            nsfw: None,
            parent_id: None,
            permission_overwrites: None,
            position: None,
            rate_limit_per_user: None,
            topic: None,
            user_limit: None,
        }
    }

    pub fn bitrate(mut self, bitrate: u64) -> Self {
        self.bitrate.replace(bitrate);

        self
    }

    pub fn kind(mut self, kind: ChannelType) -> Self {
        self.kind.replace(kind);

        self
    }

    pub fn nsfw(mut self, nsfw: bool) -> Self {
        self.nsfw.replace(nsfw);

        self
    }

    pub fn parent_id(mut self, parent_id: impl Into<ChannelId>) -> Self {
        self.parent_id.replace(parent_id.into());

        self
    }

    pub fn permission_overwrites(
        mut self,
        permission_overwrites: Vec<PermissionOverwrite>,
    ) -> Self {
        self.permission_overwrites.replace(permission_overwrites);

        self
    }

    pub fn position(mut self, position: u64) -> Self {
        self.position.replace(position);

        self
    }

    pub fn rate_limit_per_user(mut self, rate_limit_per_user: u64) -> Self {
        self.rate_limit_per_user.replace(rate_limit_per_user);

        self
    }

    pub fn topic(mut self, topic: impl Into<String>) -> Self {
        self.topic.replace(topic.into());

        self
    }

    pub fn user_limit(mut self, user_limit: u64) -> Self {
        self.user_limit.replace(user_limit);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            serde_json::to_vec(self)?,
            Route::CreateChannel {
                guild_id: self.guild_id.0,
            },
        )))));

        Ok(())
    }
}

poll_req!(CreateGuildChannel<'_>, GuildChannel);