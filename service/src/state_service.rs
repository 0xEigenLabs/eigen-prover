use log::{debug, error};
use plonky::field_gl::Fr;
use proto::state::{
    result_code::Code, state_service_server::StateService, Fea, FlushResponse, GetProgramRequest,
    GetProgramResponse, GetRequest, GetResponse, LoadDbRequest, LoadProgramDbRequest, ResultCode,
    SetProgramRequest, SetProgramResponse, SetRequest, SetResponse, SiblingList,
};
use state::{
    database::Database,
    smt::{SmtGetResult, SmtSetResult, SMT},
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::{Request, Response, Status};
use utils::scalar::{h4_to_string, str_to_biguint};

#[derive(Debug)]
pub struct StateServiceImpl {
    db: Arc<Database>,
    smt: Arc<RwLock<SMT>>,
}

impl StateServiceImpl {
    pub fn new(db: &Arc<Database>) -> Self {
        StateServiceImpl {
            db: Arc::clone(db),
            smt: Arc::new(RwLock::new(SMT::new(db))),
        }
    }
}

#[tonic::async_trait]
impl StateService for StateServiceImpl {
    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        debug!("Got a request: {:?}", request);

        let msg = request.get_ref();
        let root = fea_to_fr(msg.root.as_ref().unwrap());
        let key = fea_to_fr(msg.key.as_ref().unwrap());

        let mut si = self.smt.write().await;
        let r = si.get(&root, &key).await.unwrap();
        let reply = smt_get_result_to_proto(&r);

        Ok(Response::new(reply))
    }

    async fn set(&self, request: Request<SetRequest>) -> Result<Response<SetResponse>, Status> {
        debug!("Got a request: {:?}", request);

        let msg = request.get_ref();
        let old_root = fea_to_fr(msg.old_root.as_ref().unwrap());
        let key = fea_to_fr(msg.key.as_ref().unwrap());
        let new_value = str_to_biguint(&msg.value);

        let mut si = self.smt.write().await;
        let r = si
            .set(&old_root, &key, new_value, msg.persistent)
            .await
            .unwrap();
        let reply = smt_set_result_to_proto(&r);

        Ok(Response::new(reply))
    }

    async fn set_program(
        &self,
        request: Request<SetProgramRequest>,
    ) -> Result<Response<SetProgramResponse>, Status> {
        debug!("Got a request: {:?}", request);

        let msg = request.get_ref();
        let key = fea_to_fr(msg.key.as_ref().unwrap());
        let key = h4_to_string(&key);

        let r = self.db.set_program(&key, &msg.data, true).await;
        let reply = SetProgramResponse {
            result: Some(ResultCode {
                code: match r {
                    Ok(_) => Code::Success.into(),
                    _ => Code::SmtInvalidDataSize.into(),
                },
            }),
        };

        Ok(Response::new(reply))
    }

    async fn get_program(
        &self,
        request: Request<GetProgramRequest>,
    ) -> Result<Response<GetProgramResponse>, Status> {
        debug!("Got a request: {:?}", request);

        let msg = request.get_ref();
        let key = fea_to_fr(msg.key.as_ref().unwrap());
        let key = h4_to_string(&key);

        let r = self.db.get_program(&key).await;
        let reply = match r {
            Ok(data) => GetProgramResponse {
                data,
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

        Ok(Response::new(reply))
    }

    async fn load_db(&self, request: Request<LoadDbRequest>) -> Result<Response<()>, Status> {
        debug!("Got a request: {:?}", request);

        let msg = request.get_ref();
        for (k, v) in msg.input_db.iter() {
            let fe_list = v.fe.iter().map(|e| Fr::from(*e)).collect();
            self.db.write(k, &fe_list, true).await.unwrap();
        }

        Ok(Response::new(()))
    }

    async fn load_program_db(
        &self,
        request: Request<LoadProgramDbRequest>,
    ) -> Result<Response<()>, Status> {
        debug!("Got a request: {:?}", request);

        let msg = request.get_ref();
        for (k, v) in msg.input_program_db.iter() {
            self.db.write_program(k, v, true).await.unwrap();
        }

        Ok(Response::new(()))
    }

    async fn flush(&self, request: Request<()>) -> Result<Response<FlushResponse>, Status> {
        debug!("Got a request: {:?}", request);

        let reply = FlushResponse {
            result: Some(ResultCode {
                code: Code::Success.into(),
            }),
        };

        Ok(Response::new(reply))
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
        value: r.value.to_str_radix(16),
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
