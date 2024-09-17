use crate::repository::{AffiliateGraphData, AffiliateGraphDataBeforeCreate, AffiliateStatisticsData};
use sea_orm::{ConnectionTrait, DbErr, DbErr::RecordNotFound, TransactionError, TransactionTrait};
use serde::{Deserialize, Serialize};
use tracing::warn;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffiliateEvent {
    pub from: Uuid,
    pub to: Uuid,
    pub raw_value: f32,
}

pub async fn write_event_into_database(
    db: &(impl ConnectionTrait + TransactionTrait),
    event: &AffiliateEvent,
) -> Result<(), DbErr> {
    let to_user = AffiliateStatisticsData::find_by_id(db, event.from).await?;
    if to_user.is_none() {
        warn!("Yggdrasil Affiliate Module Error: A user ({}) doesn't have a statistics record but invites another user ({}).
         This usually happen because of data inconsistency or bug.", event.to, event.from);
        return Err(RecordNotFound("User does not exist".to_owned()));
    }
    let to_user = to_user.unwrap();
    let rate = to_user.rate;
    let raw_value = event.raw_value;
    let reward = rate * raw_value;
    let graph_edge = AffiliateGraphDataBeforeCreate {
        from: event.from.clone(),
        to: event.to.clone(),
        reward,
        rate,
    };
    let tr_result = db.transaction::<_, (), DbErr>(|tx| {
        Box::pin(async move {
            AffiliateGraphData::create(tx, &graph_edge).await?;
            AffiliateStatisticsData::on_invite(tx, &to_user, raw_value).await?;
            Ok(())
        })
    }).await;

    fn flatten_error(transaction_error: TransactionError<DbErr>) -> DbErr {
        match transaction_error {
            TransactionError::Connection(err) => err,
            TransactionError::Transaction(err) => err,
        }
    }

    tr_result.map_err(flatten_error)
}