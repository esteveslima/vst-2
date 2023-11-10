use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Deserialize, Serialize)]
pub struct SomeQueryStruct {
    query_param: i32,
}

#[derive(Deserialize, Serialize)]
pub struct SomeBodyPropStruct {
    x: i32,
    y: bool,
    z: String,
}

#[derive(Deserialize, Serialize)]
pub struct SomeBodyStruct {
    c: String,
    d: SomeBodyPropStruct,
}

//

#[derive(Serialize, Deserialize)]
pub struct SomeResultStruct {
    path: String,
    query: SomeQueryStruct, //String,
    body: SomeBodyStruct,//String,
}

//

pub async fn foo_controller_handler(
    path1: String,
    path2: usize,
    query: SomeQueryStruct,
    body: SomeBodyStruct,
) -> Result<impl warp::Reply, warp::Rejection> {
    tokio::time::sleep(Duration::from_secs(5)).await;

    let obj = SomeResultStruct { 
        path: format!("{path1}, {path2}"),
        query,//serde_json::json!(query).to_string(),
        body,//serde_json::json!(body).to_string()
     };

    let result = serde_json::json!(obj);

    return Ok(warp::reply::json(&result));
}
