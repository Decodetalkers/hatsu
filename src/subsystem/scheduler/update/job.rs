use activitypub_federation::config::{Data, FederationConfig};
use sea_orm::*;
use url::Url;

use crate::{
    AppData,
    AppError,
    entities::{
        prelude::*,
        impls::JsonUserFeed,
        user::{self, Model as DbUser},
        user_feed_item::Model as DbUserFeedItem,
    },
    utilities::Feed,
};

use super::check_feed_item;

// pub async fn fast_update(data: &Data<AppData>) -> Result<(), AppError> {
pub async fn fast_update(config: &FederationConfig<AppData>) -> Result<(), AppError> {
    let data = config.to_request_data();

    for user in User::find()
        .filter(user::Column::Local.eq(true))
        .order_by_asc(user::Column::Id)
        .all(&data.conn)
        .await? {
            fast_update_per_user(&data, user).await?;
        }

    Ok(())
}

pub async fn fast_update_per_user(data: &Data<AppData>, user: DbUser) -> Result<(), AppError> {
    let feed: Feed = serde_json::from_str(&user.feed.clone().unwrap())?;

    // Tests for JSON Feed only
    let feed: JsonUserFeed = reqwest::get(Url::parse(&feed.json.clone().unwrap())?)
        .await?
        .json::<JsonUserFeed>()
        .await?;

    for item in feed.items {
        check_feed_item(data, &user, DbUserFeedItem::from_json(item, Url::parse(&user.id)?.into(), None).await?).await?;
    }

    Ok(())
}