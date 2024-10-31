use std::sync::Arc;

use axum::{body::Body, http::Request};
use rstest::rstest;
use tower::ServiceExt;

use crate::{
    deserialize_json,
    helper::{fixture, make_router, v1, TestRequestExt},
};
use api::model::book::PaginatedBookResponse;
use kernel::{
    model::{
        book::Book,
        id::{BookId, UserId},
        list::PaginatedList,
        user::BookOwner,
    },
    repository::book::MockBookRepository,
};

#[rstest]
#[case::normal("/books", 20, 0, true)]
#[case::normal("/books?limit=50", 50, 0, true)]
#[case::normal("/books?limit=50&offset=20", 50, 20, true)]
#[case::normal("/books?offset=20", 20, 20, true)]
#[case::invalid_limit("/books?limit=-1", -1, 0, false)]
#[case::invalid_offset("/books?offset=aaa", 20, 0, false)]
#[tokio::test]
async fn show_book_list_with_query_200(
    mut fixture: registry::MockAppRegistryExt,
    #[case] path: &str,
    #[case] expected_limit: i64,
    #[case] expected_offset: i64,
    #[case] is_valid: bool,
) -> anyhow::Result<()> {
    let book_id = BookId::new();

    fixture.expect_book_repository().returning(move || {
        let mut mock = MockBookRepository::new();
        mock.expect_find_all().returning(move |opt| {
            let items = vec![Book {
                id: book_id,
                title: "Test Title".into(),
                author: "Test Author".into(),
                isbn: "Test ISBN".into(),
                description: "Test Description".into(),
                owner: BookOwner {
                    id: UserId::new(),
                    name: "Test Owner".into(),
                },
                checkout: None,
            }];
            Ok(PaginatedList {
                total: 1,
                limit: opt.limit,
                offset: opt.offset,
                items,
            })
        });
        Arc::new(mock)
    });

    let app: axum::Router = make_router(fixture);

    let req = Request::get(&v1(path)).bearer().body(Body::empty())?;
    let resp = app.oneshot(req).await?;
    if is_valid {
        assert_eq!(
            resp.status(),
            axum::http::StatusCode::OK,
            "ステータスコード{}を返しました．",
            resp.status()
        );
        let result = deserialize_json!(resp, PaginatedBookResponse);
        assert_eq!(result.limit, expected_limit);
        assert_eq!(result.offset, expected_offset);
    } else {
        assert_ne!(resp.status(), axum::http::StatusCode::OK);
    }
    Ok(())
}
