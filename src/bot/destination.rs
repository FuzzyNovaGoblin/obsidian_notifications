use serenity::all::{ChannelId, UserId};

use super::context::DisCtx;

#[derive(Clone, Copy)]
pub enum Destination {
    Fuzzy,
    Bean,
    MagicBeansObsidianCh,
    DebugObsidianCh,
}

include!("secret_ids.rs");

impl Destination {
    pub async fn id(&self, ctx: DisCtx) -> ChannelId {
        match self {
            Destination::Fuzzy => {
                UserId::new(FUZZY_ID)
                    .create_dm_channel(ctx)
                    .await
                    .unwrap()
                    .id
            }
            Destination::Bean => {
                UserId::new(BEAN_ID)
                    .create_dm_channel(ctx)
                    .await
                    .unwrap()
                    .id
            }
            Destination::MagicBeansObsidianCh => ChannelId::new(MAGIC_BEANS_OBSIDIAN_CH_ID),
            Destination::DebugObsidianCh => ChannelId::new(DEBUG_OBSIDIAN_CH_ID),
        }
    }
}
