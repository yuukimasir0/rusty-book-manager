use super::user::BookOwner;
use derive_new::new;
use garde::Validate;
use kernel::model::{
    book::{
        event::{CreateBook, UpdateBook},
        Book, BookListOptions,
    },
    id::{BookId, UserId},
    list::PaginatedList,
};
use serde::{Deserialize, Serialize};
#[cfg(debug_assertions)]
use utoipa::ToSchema;

use super::user::CheckoutUser;
use chrono::{DateTime, Utc};
use kernel::model::book::Checkout;
use kernel::model::id::CheckoutId;

#[derive(Debug, Deserialize, Validate)]
#[cfg_attr(debug_assertions, derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct CreateBookRequest {
    #[garde(length(min = 1))]
    pub title: String,
    #[garde(length(min = 1))]
    pub author: String,
    #[garde(length(min = 1))]
    pub isbn: String,
    #[garde(skip)]
    pub description: String,
}

impl From<CreateBookRequest> for CreateBook {
    fn from(value: CreateBookRequest) -> Self {
        let CreateBookRequest {
            title,
            author,
            isbn,
            description,
        } = value;
        CreateBook {
            title,
            author,
            isbn,
            description,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
#[cfg_attr(debug_assertions, derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct UpdateBookRequest {
    #[garde(length(min = 1))]
    pub title: String,
    #[garde(length(min = 1))]
    pub author: String,
    #[garde(length(min = 1))]
    pub isbn: String,
    #[garde(skip)]
    pub description: String,
}

#[derive(new)]
pub struct UpdateBookRequestWithIds(BookId, UserId, UpdateBookRequest);
impl From<UpdateBookRequestWithIds> for UpdateBook {
    fn from(value: UpdateBookRequestWithIds) -> Self {
        let UpdateBookRequestWithIds(
            book_id,
            user_id,
            UpdateBookRequest {
                title,
                author,
                isbn,
                description,
            },
        ) = value;
        UpdateBook {
            book_id,
            title,
            author,
            isbn,
            description,
            requested_user: user_id,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct BookListQuery {
    #[garde(range(min = 0))]
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[garde(range(min = 0))]
    #[serde(default)]
    pub offset: i64,
}

const DEFAULT_LIMIT: i64 = 20;
const fn default_limit() -> i64 {
    DEFAULT_LIMIT
}

impl From<BookListQuery> for BookListOptions {
    fn from(value: BookListQuery) -> Self {
        let BookListQuery { limit, offset } = value;
        Self { limit, offset }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct BookResponse {
    pub id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
    pub owner: BookOwner,
    pub checkout: Option<BookCheckoutResponse>,
}

impl From<Book> for BookResponse {
    fn from(value: Book) -> Self {
        let Book {
            id,
            title,
            author,
            isbn,
            description,
            owner,
            checkout,
        } = value;
        Self {
            id,
            title,
            author,
            isbn,
            description,
            owner: owner.into(),
            checkout: checkout.map(BookCheckoutResponse::from),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct PaginatedBookResponse {
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub items: Vec<BookResponse>,
}

impl From<PaginatedList<Book>> for PaginatedBookResponse {
    fn from(value: PaginatedList<Book>) -> Self {
        let PaginatedList {
            total,
            limit,
            offset,
            items,
        } = value;
        Self {
            total,
            limit,
            offset,
            items: items.into_iter().map(BookResponse::from).collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct BookCheckoutResponse {
    pub id: CheckoutId,
    pub checked_out_by: CheckoutUser,
    pub checked_out_at: DateTime<Utc>,
}

impl From<Checkout> for BookCheckoutResponse {
    fn from(value: Checkout) -> Self {
        let Checkout {
            checkout_id,
            checked_out_by,
            checked_out_at,
        } = value;
        Self {
            id: checkout_id,
            checked_out_by: checked_out_by.into(),

            checked_out_at,
        }
    }
}
