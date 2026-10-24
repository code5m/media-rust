use axum::{extract::{Path,State},http::{HeaderMap,StatusCode},routing::{get,post},Json,Router};
use serde::{Deserialize,Serialize};
use serde_json::{json,Value};
use std::{collections::HashMap,sync::{Arc,Mutex}};
#[derive(Clone,Serialize)]struct Platform {id:&'static str,name:&'static str,mode:&'static str,verification:&'static str}
#[derive(Clone,Deserialize)]#[serde(rename_all="camelCase")]struct Target {platform:String,account_id:String}
#[derive(Deserialize)]#[serde(rename_all="camelCase")]struct PublishRequest {content_id:String,targets:Vec<Target>,#[serde(default)]dry_run:bool}
#[derive(Clone,Serialize)]#[serde(rename_all="camelCase")]struct Job {platform:String,account_id:String,mode:String,status:&'static str,reason:&'static str}
#[derive(Clone,Serialize)]#[serde(rename_all="camelCase")]struct Batch {id:String,status:&'static str,dry_run:bool,jobs:Vec<Job>}
#[derive(Default)]struct Data {batches:Mutex<HashMap<String,Batch>>,keys:Mutex<HashMap<String,String>>}
type Shared=Arc<Data>;
async fn health()->Json<Value>{Json(json!({"status":"UP","runtime":"rust","mode":"scaffold"}))}
async fn platforms()->Json<Vec<Platform>>{Json(vec![
Platform{id:"wechat",name:"微信公众号",mode:"DIRECT_API",verification:"DOCUMENTED"},
Platform{id:"xiaohongshu",name:"小红书",mode:"HANDOFF",verification:"DOCUMENTED"},
Platform{id:"douyin",name:"抖音",mode:"DIRECT_API",verification:"DOCUMENTED"},
Platform{id:"bilibili",name:"B站",mode:"HANDOFF",verification:"DOCUMENTED"},
Platform{id:"weibo",name:"微博",mode:"HANDOFF",verification:"DOCUMENTED"},
Platform{id:"zhihu",name:"知乎",mode:"HANDOFF",verification:"DOCUMENTED"},
Platform{id:"wordpress",name:"WordPress",mode:"DIRECT_API",verification:"DOCUMENTED"}])}
async fn publish(State(state):State<Shared>,headers:HeaderMap,Json(req):Json<PublishRequest>)->(StatusCode,Json<Value>){
 if req.content_id.trim().is_empty()||req.targets.is_empty(){return (StatusCode::BAD_REQUEST,Json(json!({"code":"INVALID_REQUEST","message":"contentId and targets are required"})))}
 if !req.dry_run{return (StatusCode::NOT_IMPLEMENTED,Json(json!({"code":"PUBLISH_NOT_IMPLEMENTED","message":"Real publishing and authorization have not been implemented"})))}
 let key=headers.get("Idempotency-Key").and_then(|v|v.to_str().ok()).filter(|s|!s.is_empty()).map(str::to_owned);
 let id=if let Some(key)=key {let mut keys=state.keys.lock().unwrap();keys.entry(key).or_insert_with(||uuid::Uuid::new_v4().to_string()).clone()} else {uuid::Uuid::new_v4().to_string()};
 let mut batches=state.batches.lock().unwrap();
 let batch=batches.entry(id.clone()).or_insert_with(||Batch{id,status:"VALIDATED",dry_run:true,jobs:req.targets.into_iter().map(|t|Job{mode:if t.platform=="xiaohongshu"{"HANDOFF"}else{"UNVERIFIED"}.into(),platform:t.platform,account_id:t.account_id,status:"BLOCKED_AUTH",reason:"No real platform authorization or media upload has been configured"}).collect()}).clone();
 (StatusCode::ACCEPTED,Json(serde_json::to_value(batch).unwrap()))
}
async fn get_batch(State(state):State<Shared>,Path(id):Path<String>)->(StatusCode,Json<Value>){match state.batches.lock().unwrap().get(&id){Some(batch)=>(StatusCode::OK,Json(serde_json::to_value(batch).unwrap())),None=>(StatusCode::NOT_FOUND,Json(json!({"code":"NOT_FOUND","message":"Batch not found"})))}}
#[tokio::main]async fn main(){let state=Arc::new(Data::default());let app=Router::new().route("/api/v1/health",get(health)).route("/api/v1/platforms",get(platforms)).route("/api/v1/publish-batches",post(publish)).route("/api/v1/publish-batches/:id",get(get_batch)).with_state(state);let listener=tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();println!("Media Studio Rust listening on http://127.0.0.1:8080");axum::serve(listener,app).await.unwrap();}
