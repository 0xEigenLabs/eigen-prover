#![allow(clippy::all)]
#![allow(unknown_lints)]
use log::{debug, error};
use plonky::field_gl::Fr;
use statedb::{
    database::Database,
    smt::{SmtGetResult, SmtSetResult, SMT},
};
use statedb_service::state_db_service_server::StateDbService;
use statedb_service::{
    result_code::Code, Fea, FlushResponse, GetProgramRequest, GetProgramResponse, GetRequest,
    GetResponse, LoadDbRequest, LoadProgramDbRequest, ResultCode, SetProgramRequest,
    SetProgramResponse, SetRequest, SetResponse, SiblingList,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Status};
use utils::scalar::{h4_to_string, str_to_biguint};

pub mod statedb_service {
    tonic::include_proto!("statedb.v1"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct StateDBServiceSVC {
    smt: Arc<Mutex<SMT>>,
}

impl StateDBServiceSVC {
    pub fn new(url: String) -> Self {
        let db = Database::new(Some(url));
        StateDBServiceSVC {
            smt: Arc::new(Mutex::new(SMT::new(db))),
        }
    }
}

#[inline(always)]
fn fea_to_fr(fea: &Fea) -> [Fr; 4] {
    [
        Fr::from(fea.fe0),
        Fr::from(fea.fe1),
        Fr::from(fea.fe2),
        Fr::from(fea.fe3),
    ]
}

#[inline(always)]
fn fr_to_fea(sca: &[Fr; 4]) -> Fea {
    Fea {
        fe0: sca[0].as_int(),
        fe1: sca[1].as_int(),
        fe2: sca[2].as_int(),
        fe3: sca[3].as_int(),
    }
}

#[inline(always)]
fn smt_get_result_to_proto(r: &SmtGetResult) -> GetResponse {
    let sib = r
        .siblings
        .iter()
        .map(|(k, v)| {
            let rk = *k as u64;
            let sl = v.iter().map(|e| e.as_int()).collect::<Vec<u64>>();
            (rk, SiblingList { sibling: sl })
        })
        .collect::<HashMap<u64, SiblingList>>();
    GetResponse {
        root: Some(fr_to_fea(&r.root)),
        key: Some(fr_to_fea(&r.key)),
        siblings: sib,
        ins_key: Some(fr_to_fea(&r.ins_key)),
        ins_value: r.ins_value.to_string(),
        is_old0: r.is_old0,
        value: r.value.to_string(),
        proof_hash_counter: r.proof_hash_counter,
        db_read_log: HashMap::new(),
        result: Some(ResultCode {
            code: Code::Success.into(),
        }),
    }
}

#[inline(always)]
fn smt_set_result_to_proto(r: &SmtSetResult) -> SetResponse {
    let sib = r
        .siblings
        .iter()
        .map(|(k, v)| {
            let rk = *k as u64;
            let sl = v.iter().map(|e| e.as_int()).collect::<Vec<u64>>();
            (rk, SiblingList { sibling: sl })
        })
        .collect::<HashMap<u64, SiblingList>>();
    SetResponse {
        old_root: Some(fr_to_fea(&r.old_root)),
        key: Some(fr_to_fea(&r.key)),
        siblings: sib,
        ins_key: Some(fr_to_fea(&r.ins_key)),
        ins_value: r.ins_value.to_string(),
        is_old0: r.is_old0,
        proof_hash_counter: r.proof_hash_counter,
        db_read_log: HashMap::new(),
        result: Some(ResultCode {
            code: Code::Success.into(),
        }),
        mode: r.mode.clone(),
        new_root: Some(fr_to_fea(&r.new_root)),
        old_value: r.old_value.to_string(),
        new_value: r.new_value.to_string(),
    }
}

#[tonic::async_trait]
impl StateDbService for StateDBServiceSVC {
    async fn get(
        &self,
        request: Request<GetRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<GetResponse>, Status> {
        // Return an instance of type HelloReply
        debug!("Got a request: {:?}", request);
        let msg = request.get_ref();
        let root = fea_to_fr(msg.root.as_ref().unwrap());
        let key = fea_to_fr(msg.key.as_ref().unwrap());
        let mut si = self.smt.lock().map_err(|e| {
            error!("Get smt instance error due to data race, {:?}", e);
            tonic::Status::resource_exhausted("SMT data race")
        })?;
        let r = si.get(&root, &key).unwrap();
        let reply = smt_get_result_to_proto(&r);
        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn set(
        &self,
        request: Request<SetRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<SetResponse>, Status> {
        // Return an instance of type HelloReply
        debug!("Got a request: {:?}", request);
        let msg = request.get_ref();

        let old_root = fea_to_fr(msg.old_root.as_ref().unwrap());
        let key = fea_to_fr(msg.key.as_ref().unwrap());
        let new_value = str_to_biguint(&msg.value);
        let mut si = self.smt.lock().map_err(|e| {
            error!("Get smt instance error due to data race, {:?}", e);
            tonic::Status::resource_exhausted("SMT data race")
        })?;
        let r = si.set(&old_root, &key, new_value, msg.persistent).unwrap();
        let reply = smt_set_result_to_proto(&r);
        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn set_program(
        &self,
        request: Request<SetProgramRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<SetProgramResponse>, Status> {
        // Return an instance of type HelloReply
        debug!("Got a request: {:?}", request);
        let msg = request.get_ref();

        let mut si = self.smt.lock().map_err(|e| {
            error!("Get smt instance error due to data race, {:?}", e);
            tonic::Status::resource_exhausted("SMT data race")
        })?;
        let key = fea_to_fr(msg.key.as_ref().unwrap());
        let key = h4_to_string(&key);

        let r = si.db_mut().set_program(&key, &msg.data, true);

        let reply = SetProgramResponse {
            result: Some(ResultCode {
                code: match r {
                    Ok(_) => Code::Success.into(),
                    _ => Code::SmtInvalidDataSize.into(),
                },
            }),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn get_program(
        &self,
        request: Request<GetProgramRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<GetProgramResponse>, Status> {
        // Return an instance of type HelloReply
        debug!("Got a request: {:?}", request);

        let msg = request.get_ref();

        let mut si = self.smt.lock().map_err(|e| {
            error!("Get smt instance error due to data race, {:?}", e);
            tonic::Status::resource_exhausted("SMT data race")
        })?;
        let key = fea_to_fr(msg.key.as_ref().unwrap());
        let key = h4_to_string(&key);

        let r = si.db_mut().get_program(&key);

        let reply = match r {
            Ok(data) => GetProgramResponse {
                data: data,
                result: Some(ResultCode {
                    code: Code::Success.into(),
                }),
            },
            Err(e) => {
                error!("error: {:?}", e);
                GetProgramResponse {
                    data: vec![],
                    result: Some(ResultCode {
                        code: Code::SmtInvalidDataSize.into(),
                    }),
                }
            }
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn load_db(
        &self,
        request: Request<LoadDbRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<()>, Status> {
        // Return an instance of type HelloReply
        debug!("Got a request: {:?}", request);

        Ok(Response::new(())) // Send back our formatted greeting
    }

    async fn load_program_db(
        &self,
        request: Request<LoadProgramDbRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<()>, Status> {
        // Return an instance of type HelloReply
        debug!("Got a request: {:?}", request);

        Ok(Response::new(())) // Send back our formatted greeting
    }

    async fn flush(
        &self,
        request: Request<()>, // Accept request of type HelloRequest
    ) -> Result<Response<FlushResponse>, Status> {
        // Return an instance of type HelloReply
        debug!("Got a request: {:?}", request);

        let reply = FlushResponse::default();

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}
