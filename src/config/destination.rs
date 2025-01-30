use serenity::all::{ChannelId, UserId};

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Destination {
    pub id: u64,
    ch_type: ChannelType,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub enum ChannelType {
    #[serde(rename = "dm")]
    Dm,
    #[serde(rename = "server")]
    Server,
}

impl Destination {
    pub async fn id(&self, ctx: crate::Ctx) -> ChannelId {
        match self.ch_type {
            ChannelType::Dm => {
                UserId::new(self.id)
                    .create_dm_channel(ctx.dis_ctx.clone())
                    .await
                    .unwrap()
                    .id
            }
            ChannelType::Server => ChannelId::new(self.id),
        }
    }
}
