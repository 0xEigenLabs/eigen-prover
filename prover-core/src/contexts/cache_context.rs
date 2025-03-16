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
    Snark(SnarkFileType),
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
        let already_cached = !cache_dir.is_empty();
        log::debug!("Cache used: {already_cached}");
        let cache_dir = if cache_dir.is_empty() { "cache".to_string() } else { cache_dir };
        ProveDataCache {
            task_name,
            base_dir,
            cache_dir,
            agg_cache: AggData { already_cached, ..Default::default() },
            final_cache: FinalData { already_cached, ..Default::default() },
            snark_cache: SnarkData { already_cached, ..Default::default() },
        }
        .load()
    }

    /// Load all the cached file to memory, TODO
    pub fn load(mut self) -> Self {
        if self.agg_cache.already_cached {
            self.agg_cache.add(
                format!("{}/agg/{}.recursive1.const", self.cache_dir, self.task_name),
                StarkFileType::Const,
            );
            self.agg_cache.add(
                format!("{}/agg/{}.recursive1.exec", self.cache_dir, self.task_name),
                StarkFileType::Exec,
            );
            self.agg_cache.add(
                format!("{}/agg/{}.recursive1.pil", self.cache_dir, self.task_name),
                StarkFileType::Pil,
            );
            self.agg_cache.add(
                format!("{}/agg/{}.recursive1.pil.json", self.cache_dir, self.task_name),
                StarkFileType::PilJson,
            );
            self.agg_cache.add(
                format!("{}/agg/{}.recursive1.r1cs", self.cache_dir, self.task_name),
                StarkFileType::R1cs,
            );
            self.agg_cache.add(
                format!("{}/agg/{}.recursive1.wasm", self.cache_dir, self.task_name),
                StarkFileType::Wasm,
            );
        }

        if self.final_cache.already_cached {
            self.final_cache.add(
                format!("{}/final/{}.recursive2.const", self.cache_dir, self.task_name),
                StarkFileType::Const,
            );
            self.final_cache.add(
                format!("{}/final/{}.recursive2.exec", self.cache_dir, self.task_name),
                StarkFileType::Exec,
            );
            self.final_cache.add(
                format!("{}/final/{}.recursive2.pil", self.cache_dir, self.task_name),
                StarkFileType::Pil,
            );
            self.final_cache.add(
                format!("{}/final/{}.recursive2.pil.json", self.cache_dir, self.task_name),
                StarkFileType::PilJson,
            );
            self.final_cache.add(
                format!("{}/final/{}.recursive2.r1cs", self.cache_dir, self.task_name),
                StarkFileType::R1cs,
            );
            self.final_cache.add(
                format!("{}/final/{}.recursive2.wasm", self.cache_dir, self.task_name),
                StarkFileType::Wasm,
            );
        }

        if self.snark_cache.already_cached {
            self.snark_cache.add(
                format!("{}/snark/{}.final.wasm", self.cache_dir, self.task_name),
                SnarkFileType::Wasm,
            );
            self.snark_cache.add(
                format!("{}/snark/{}.final.r1cs", self.cache_dir, self.task_name),
                SnarkFileType::R1cs,
            );
            self.snark_cache.add(format!("{}/snark/g16.key", self.cache_dir), SnarkFileType::PK);
            self.snark_cache
                .add(format!("{}/snark/verification_key.json", self.cache_dir), SnarkFileType::VK);
        }

        log::debug!("Load cache done, {:?}", self);
        self
    }

    pub fn batch_add(&mut self, caches: Vec<(String, CacheStage)>) -> Result<()> {
        caches.iter().for_each(|f| self.add(f.0.clone(), f.1).unwrap());
        Ok(())
    }

    pub fn add(&mut self, src_full_path: String, stage: CacheStage) -> Result<()> {
        let src_path = Path::new(&src_full_path);

        let src_file_name = src_path.file_name().ok_or_else(|| anyhow!("Infalid file"))?;
        let src_file_name_str = src_file_name.to_str().ok_or_else(|| anyhow!("Infalid file"))?;

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
            CacheStage::Snark(file_type) => self.snark_cache.add(cache_path.clone(), file_type),
        }
        Ok(())
    }
}

type AggData = StarkFile;
type FinalData = StarkFile;
type SnarkData = SnarkFile;

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
            CacheStage::Snark(_) => String::from("snark"),
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
