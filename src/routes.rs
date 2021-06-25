use std::convert::Infallible;

use warp::http::Response;
use warp::Filter;

use crate::db::Db;
use crate::handlers;
use crate::models::Comment;

#[cfg(test)]
mod tests {
    use log::debug;
    use log::info;
    use serde::de;
    use tokio;
    use warp::http::Response;
    use warp::hyper::body::Bytes;
    use warp::test::request;

    use crate::db::init_db;
    use crate::models::{AuthorInfo, Comment};
    use crate::routes::{add_comment, get_comments, routes};

    const SLUG: &str = "rust-comment-system";

    fn deserialize<T>(req: Response<Bytes>) -> serde_json::Result<Response<T>>
    where
        for<'de> T: de::Deserialize<'de>,
    {
        let (parts, body) = req.into_parts();
        let body = serde_json::from_slice(&body)?;
        Ok(Response::from_parts(parts, body))
    }

    #[tokio::test]
    async fn test_post_anon_comment() {
        pretty_env_logger::try_init();
        let db = init_db();
        let comment = Comment {
            uid: None,
            on: SLUG.to_string(),
            comment: "I love testing".to_string(),
            created: None,
            updated: None,
            author: AuthorInfo {
                name: None,
                website: None,
                email: None,
            },
        };
        let post = add_comment(db.clone());
        let res = request()
            .path("/comments/")
            .method("POST")
            .header("accept", "application/json")
            .header("content-type", "application/json")
            .json(&comment)
            .reply(&post)
            .await;
        assert_eq!(res.status(), 201);
    }

    #[tokio::test]
    async fn test_post_anon_then_get() {
        pretty_env_logger::try_init();
        let db = init_db();
        let comment = Comment {
            uid: None,
            on: SLUG.to_string(),
            comment: "I love testing".to_string(),
            created: None,
            updated: None,
            author: AuthorInfo {
                name: None,
                website: None,
                email: None,
            },
        };
        let post = add_comment(db.clone());
        let get = get_comments(db.clone());
        info!("tests: sending comment creation...");
        let res = request()
            .path("/comments/{}")
            .method("POST")
            .header("accept", "application/json")
            .header("content-type", "application/json")
            .json(&comment)
            .reply(&post)
            .await;
        assert_eq!(res.status(), 201);
        info!("tests: sending GET comments on /comments/{}", SLUG);
        let res2 = request()
            .path(&format!("/comments/{}", SLUG))
            .header("accept", "application/json")
            .header("content-type", "application/json")
            .reply(&get)
            .await;
        assert_eq!(res2.status(), 200);
        let rd: Response<Vec<Comment>> = deserialize(res2).expect("could not deserialize response");
        assert_eq!(rd.body().len(), 1);
        let c = rd.body().first().unwrap();
        assert!(c.uid.is_some());
        assert_eq!(c.uid.unwrap(), 0);
        assert_eq!(c.comment, "I love testing");
        assert_eq!(c.on, SLUG);
        assert_eq!(c.updated, None);
        assert!(c.created.is_some());
    }

    #[tokio::test]
    async fn test_static_files() {
        let db = init_db();
        for path in &["dezrevellou.min.js", "dezrevellou.min.js.map", "dezrevellou.min.css", "dezrevellou.min.css.map"] {
            let r = request().path(&format!("/static/{}", path)).reply(&routes(db.clone())).await;
            assert_eq!(r.status(), 200);
        }
    }
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn json_body() -> impl Filter<Extract = (Comment,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn routes(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let static_content = warp::path!("static" / "dezrevellou.min.js")
        .and(warp::get())
        .map(|| {
            Response::builder()
                .header("Content-Type", "text/javascript")
                .body(include_str!("../dist/dezrevellou.min.js"))
        }).or(warp::path!("static" / "dezrevellou.js")
        .and(warp::get())
        .map(|| {
            Response::builder()
                .header("Content-Type", "text/javascript")
                .body(include_str!("../dist/dezrevellou.js"))
        })).or(warp::path!("static" / "dezrevellou.min.js.map")
        .and(warp::get())
        .map(|| {
            Response::builder()
                .header("Content-Type", "text/javascript")
                .body(include_str!("../dist/dezrevellou.min.js.map"))
        })).or(warp::path!("static" / "dezrevellou.min.css")
        .and(warp::get())
        .map(|| {
            Response::builder()
                .header("Content-Type", "text/css")
                .body(include_str!("../dist/dezrevellou.min.css"))
        })).or(warp::path!("static" / "dezrevellou.min.css.map")
        .and(warp::get())
        .map(|| {
            Response::builder()
                .header("Content-Type", "text/css")
                .body(include_str!("../dist/dezrevellou.min.css.map"))
        })).or(warp::path!("static" / "dezrevellou.css")
        .and(warp::get())
        .map(|| {
            Response::builder()
                .header("Content-Type", "text/css")
                .body(include_str!("../dist/dezrevellou.css"))
        }));
    let options = warp::path("comments")
        .and(warp::options())
        .map(|| warp::reply::with_header(warp::reply::reply(), "Allow", "OPTIONS, POST"));
    get_comments(db.clone())
        .or(add_comment(db))
        .or(static_content)
        .or(options)
}

/// GET /comments/:slug
pub fn get_comments(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("comments")
        .and(warp::path::param())
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::get_comments)
}

/// POST /comments/
pub fn add_comment(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("comments")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::new_comment)
}
