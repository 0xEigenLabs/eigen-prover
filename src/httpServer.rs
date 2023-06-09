use actix_web::post;
use actix_web::{get, Responder, HttpResponse, web};
use serde_json::Value;
use crate::{make_prove_opt, VerifyOpt};
use crate::make_prove_opt_for_update;
use crate::make_prove_opt_for_withdraw;



#[post("/make_prove_opt")]
async fn http_make_prove_opt(value: web::Json<Value>) -> impl Responder {
    let value = value.0;
    let res = make_prove_opt(value);
    let body = serde_json::to_string(&res).unwrap();
    HttpResponse::Ok().body(body)

}

#[post("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong".to_string())

}



#[post("/make_prove_opt_for_update")]
async fn http_make_prove_opt_for_update(value: web::Json<Value>) ->  impl Responder {
    let value = value.0;
    let res = make_prove_opt_for_update(value);
    let body = serde_json::to_string(&res).unwrap();
    HttpResponse::Ok().body(body)
    
}
#[post("/make_prove_opt_for_withdraw")]
async fn http_make_prove_opt_for_withdraw(value: web::Json<Value>) ->  impl Responder {
    let value = value.0;
    let res = make_prove_opt_for_withdraw(value);
    let body = serde_json::to_string(&res).unwrap();
    HttpResponse::Ok().body(body)
 
}

#[post("/verify")]
async fn http_verify(value: web::Json<Value>) ->  impl Responder {

    // let opt: VerifyOpt = serde_json::from_value(req.body).unwrap();
    // verify(&opt.vk_file, &opt.proof_bin, &opt.transcript)?;
    // lines.send(true.to_string()).await?;
 
 
    // let body = serde_json::to_string(&res).unwrap();
    HttpResponse::Ok().body("NOT work")
 
}
