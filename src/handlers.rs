use std::convert::Infallible;

use chrono::Utc;
use warp::http::StatusCode;
use warp::reply::json;

use log::debug;

use crate::db::{save_db, Db};
use crate::models::Comment;

pub async fn get_next_available_id(db: Db) -> Option<u16> {
    debug!("get_next_available_id: getting db lock...");
    let cs = db.lock().await;
    debug!("get_next_available_id: got the lock");
    for i in 0..u16::MAX {
        if !cs.iter().any(|c| c.uid == Some(i)) {
            debug!("get_next_available_id: found free uid: {}", i);
            return Some(i);
        }
    }
    None
}

pub async fn get_comments(slug: String, db: Db) -> Result<Box<dyn warp::Reply>, Infallible> {
    debug!("get_comments: called with {}. Getting db lock", slug);
    let comments = db.lock().await;
    debug!("get_comments: got db lock");
    let filter: Vec<&Comment> = comments.iter().filter(|c| c.on == slug).collect();
    debug!("get_comments: found {} comments: {:?}", filter.len(), &filter);
    Ok(Box::new(json(&filter)))
}

pub async fn new_comment(mut comment: Comment, db: Db) -> Result<impl warp::Reply, Infallible> {
    debug!("new_comment: called with {:?}", &comment);
    if comment.comment.is_empty() {
        return Ok(StatusCode::BAD_REQUEST);
    }
    if comment.created.is_none() {
        comment.created = Some(Utc::now());
    }
    debug!("new_comment: will get db lock");
    let uid = get_next_available_id(db.clone())
        .await
        .expect("Could not find a free uid in db");
    {
        // do db ops while we have the lock in its own sub-block
        let mut comments = db.lock().await;
        debug!("new_comment: got db lock");
        if comment.uid.is_none() {
            comment.uid = Some(uid);
        } else if comments.iter().any(|c| c.uid == comment.uid) {
            return Ok(StatusCode::BAD_REQUEST);
        }
        comments.push(comment);
    }
    //here, we get the lock back
    #[cfg(not(test))]
    save_db(db.clone()).await.expect("Could not save db");
    Ok(StatusCode::CREATED)
}
