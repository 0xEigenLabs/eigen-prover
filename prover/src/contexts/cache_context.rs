use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Clone, Copy)]
pub enum CacheStage {
    Agg(StarkFileType),
    Final(StarkFileType),
    Snark(Curve),
}

#[derive(Clone, Copy, Default)]
pub enum StarkFileType {
    #[default]
    R1cs,
    Pil,
    PilJson,
    Const,
    Exec,
    Wasm,
}

#[derive(Clone, Copy)]
pub enum Curve {
    BN128(SnarkFileType),
    BLS12381(SnarkFileType),
}

#[derive(Clone, Copy, Default)]
pub enum SnarkFileType {
    #[default]
    R1cs,
    PK,
    VK,
    Wasm,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ProveDataCache {
    pub task_name: String,
    pub base_dir: String,
    pub cache_dir: String,
    pub agg_cache: AggData,
    pub final_cache: FinalData,
    pub snark_cache: SnarkData,
}
// task_name: String, base_dir: String, cach_dir: String, stage: CacheStage
impl ProveDataCache {
    pub fn new(task_name: String, base_dir: String, cache_dir: String) -> Self {
        let already_cached = std::env::var("CACHE").unwrap_or("no".to_string());
        let already_cached = already_cached.contains("yes");
        log::debug!("Cache used: {already_cached}");
        ProveDataCache {
            task_name,
            base_dir,
            cache_dir,
            agg_cache: AggData {already_cached, ..Default::default()},
            final_cache: FinalData {already_cached, ..Default::default()},
            snark_cache: SnarkData {
               bn128_data: SnarkFile{already_cached, ..Default::default()}, 
               bls12381_data: SnarkFile{already_cached, ..Default::default()}, 
            },
        }
    }

    pub fn add(&mut self, src_full_path: String, stage: CacheStage) -> Result<()> {
        let src_path = Path::new(&src_full_path);

        let src_file_name = src_path
            .file_name()
            .ok_or_else(|| anyhow!("Infalid file"))?;
        let src_file_name_str = src_file_name
            .to_str()
            .ok_or_else(|| anyhow!("Infalid file"))?;

        let stage_dir = stage.construct_stage_dir(
            self.task_name.clone(),
            self.base_dir.clone(),
            self.cache_dir.clone(),
        );

        log::info!("save_checkpoint, mkdir: {:?}", stage_dir);
        fs::create_dir_all(stage_dir.clone())?;

        let mut path_buf = PathBuf::from(stage_dir);
        path_buf.push(src_file_name_str);
        let cache_path = path_buf.to_string_lossy().to_string();

        fs::copy(src_path, cache_path.clone())?;

        match stage {
            CacheStage::Agg(file_type) => self.agg_cache.add(cache_path.clone(), file_type),
            CacheStage::Final(file_type) => self.final_cache.add(cache_path.clone(), file_type),
            CacheStage::Snark(curve) => match curve {
                Curve::BN128(file_type) => self
                    .snark_cache
                    .bn128_data
                    .add(cache_path.clone(), file_type),
                Curve::BLS12381(file_type) => self
                    .snark_cache
                    .bls12381_data
                    .add(cache_path.clone(), file_type),
            },
        }
        Ok(())
    }

    pub fn update_cache_flag(&mut self, stage: CacheStage) {
        match stage {
            CacheStage::Agg(_) => self.agg_cache.already_cached = true,
            CacheStage::Final(_) => self.final_cache.already_cached = true,
            CacheStage::Snark(curve) => match curve {
                Curve::BN128(_) => self.snark_cache.bn128_data.already_cached = true,
                Curve::BLS12381(_) => self.snark_cache.bls12381_data.already_cached = true,
            },
        }
    }
}

type AggData = StarkFile;
type FinalData = StarkFile;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct StarkFile {
    pub already_cached: bool,
    pub r1cs_file: String,
    pub pil_file: String,
    pub piljson_file: String,
    pub const_file: String,
    pub exec_file: String,
    pub wasm_file: String,
}

impl StarkFile {
    pub fn add(&mut self, cache_path: String, file_type: StarkFileType) {
        match file_type {
            StarkFileType::R1cs => self.r1cs_file = cache_path,
            StarkFileType::Pil => self.pil_file = cache_path,
            StarkFileType::PilJson => self.piljson_file = cache_path,
            StarkFileType::Const => self.const_file = cache_path,
            StarkFileType::Exec => self.exec_file = cache_path,
            StarkFileType::Wasm => self.wasm_file = cache_path,
        }
    }

    pub fn update_pil_json(&mut self) {
        self.add(
            format!("{}.json", self.pil_file.clone()),
            StarkFileType::PilJson,
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SnarkData {
    pub bn128_data: SnarkFile,
    pub bls12381_data: SnarkFile,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SnarkFile {
    pub already_cached: bool,
    pub curve_type: String,
    pub r1cs_file: String,
    pub pk_file: String,
    pub vk_file: String,
    pub wasm_file: String,
}

impl SnarkFile {
    pub fn add(&mut self, cache_path: String, file_type: SnarkFileType) {
        match file_type {
            SnarkFileType::PK => self.pk_file = cache_path,
            SnarkFileType::VK => self.vk_file = cache_path,
            SnarkFileType::R1cs => self.r1cs_file = cache_path,
            SnarkFileType::Wasm => self.wasm_file = cache_path,
        }
    }
}

impl From<CacheStage> for String {
    fn from(cache_stage: CacheStage) -> Self {
        match cache_stage {
            CacheStage::Agg(_) => String::from("agg"),
            CacheStage::Final(_) => String::from("final"),
            CacheStage::Snark(curve) => match curve {
                Curve::BN128(_) => String::from("snark/bn128"),
                Curve::BLS12381(_) => String::from("snark/bls12381"),
            },
        }
    }
}

impl CacheStage {
    pub fn construct_stage_dir(
        self,
        task_name: String,
        base_dir: String,
        cache_dir: String,
    ) -> String {
        let mut path_buf = PathBuf::from(base_dir);
        path_buf.push(cache_dir);
        path_buf.push(task_name);
        path_buf.push(self.cache_stage());
        path_buf.to_string_lossy().to_string()
    }

    pub fn cache_stage(self) -> String {
        self.into()
    }
}
