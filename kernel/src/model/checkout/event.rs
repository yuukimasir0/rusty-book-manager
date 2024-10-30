use derive_new::new;
use sqlx::types::chrono::{DateTime, Utc};

use crate::model::id::{BookId, CheckoutId, UserId};

#[derive(new)]
pub struct CreateCheckout {
    pub book_id: BookId,
    pub checked_out_by: UserId,
    pub checked_out_at: DateTime<Utc>,
}

#[derive(new)]
pub struct UpdateReturned {
    pub checkout_id: CheckoutId,
    pub book_id: BookId,
    pub returned_by: UserId,
    pub returned_at: DateTime<Utc>,
}
