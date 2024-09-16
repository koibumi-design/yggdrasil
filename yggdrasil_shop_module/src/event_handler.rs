use sea_orm::sqlx::types::chrono;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartItem {
    pub production_id: u64,
    pub variant: String,
    pub amount: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cart {
    pub items: Vec<CartItem>,
    pub create_at: chrono::NaiveDateTime,
}

#[async_trait::async_trait]
pub trait ShopModuleEventHandler {
    async fn before_order_created(&self, cart: Cart);
    async fn after_order_fulfilled(&self, cart: Cart);
    async fn after_order_canceled(&self, cart: Cart);
}