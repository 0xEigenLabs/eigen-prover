use std::env;
use std::fs::File;
use std::io::BufReader;

use crate::make_prove_opt_for_update;
use crate::make_prove_opt_for_withdraw;
use crate::ProveOpt;
use crate::{make_prove_opt, VerifyOpt};
use actix_multipart::Multipart;
use actix_web::post;
use actix_web::HttpRequest;
use actix_web::{get, web, HttpResponse, Responder};

use futures::{StreamExt, TryStreamExt};
use plonky::api::calculate_witness;
use plonky::api::prove;
use plonky::api::verify;

use plonky::bellman_ce::bn256::Bn256;
use plonky::bellman_ce::{
    kate_commitment::{Crs, CrsForLagrangeForm, CrsForMonomialForm},
    pairing::Engine,
    plonk::{
        better_cs::cs::PlonkCsWidth4WithNextStepParams,
        better_cs::keys::{Proof, VerificationKey},
    },
    Field, PrimeField, PrimeFieldRepr, ScalarEngine,
};
use serde_json::Value;

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

#[post("/verify")]
async fn http_verify(value: web::Json<Value>, req: HttpRequest) -> impl Responder {
    //
    let req = value.0;

    let opt: VerifyOpt = serde_json::from_value(req).unwrap();
    let res = verify(&opt.vk_file, &opt.proof_bin, &opt.transcript);

    match res {
        Ok(res) => {
            let body = serde_json::to_string(&res).unwrap();

            HttpResponse::Ok().body(body)
        }
        Err(res) => HttpResponse::InternalServerError().body(res.to_string()),
    }
}

#[post("/prove")]
async fn http_prove(value: web::Json<Value>) -> impl Responder {
    let req = value.0;
    let opt: ProveOpt = make_prove_opt(req);
    println!("opt: {:?}", opt);
    std::fs::write(std::path::Path::new(&opt.input_json_file), opt.input_json).unwrap();
    calculate_witness(&opt.wasm_file, &opt.input_json_file, &opt.witness).unwrap();
    prove(
        &opt.circuit_file,
        &opt.witness,
        &opt.srs_monomial_form,
        None,
        &opt.transcript,
        &opt.proof_bin,
        &opt.proof_json,
        &opt.public_json,
    )
    .unwrap();
    // write back proof and public input
    let proof_json = std::fs::read_to_string(&opt.proof_json).unwrap();
    let proof_value: Value = serde_json::from_str(&proof_json).unwrap();

    let public_json = std::fs::read_to_string(&opt.public_json).unwrap();
    let public_value: Value = serde_json::from_str(&public_json).unwrap();
    let json = Value::Array(vec![proof_value, public_value]);
    let body = serde_json::to_string(&json).unwrap();
    HttpResponse::Ok().body(body)
}

#[test]
fn verifys() {
    let current_dir = env::current_dir().unwrap();
    println!("Current directory: {:?}", current_dir);

    let f = File::open("./test/vk.bin").expect("read vk err");
    println!("{:?}", f);
    let mut reader = BufReader::with_capacity(1 << 24, f);

    VerificationKey::<Bn256, PlonkCsWidth4WithNextStepParams>::read(&mut reader)
        .expect("read vk err");

    Proof::<Bn256, PlonkCsWidth4WithNextStepParams>::read(
        File::open("./test/proof.bin").expect("read proof file err"),
    )
    .unwrap();
}
