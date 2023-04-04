use std::env::{current_dir, temp_dir};
use std::path::{Path, PathBuf};

use anyhow::anyhow;
use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, Command};
use hyper::Uri;
use uuid::Uuid;
use md5;

use crate::Result;

pub struct Config {
    pub size: usize,
    pub uri: Uri,
    pub file_path: String,
    pub temp_file_dir: PathBuf,
}

impl Config {
    pub fn get() -> Result<Self> {
        let matches = Command::new(crate_name!())
            .version(crate_version!())
            .author(crate_authors!())
            .about(crate_description!())
            .args(&[
                Arg::new("size").help("并发任务数量").required(true),
                Arg::new("uri").help("资源 URI").required(true),
                Arg::new("file-path").help("保存文件路径").required(false),
            ])
            .get_matches();

        let size = matches.value_of_t("size")?;
        let uri: Uri = matches.value_of_t("uri")?;
        let file_path = matches.value_of("file-path");
      
        println!("The uri passed is: {:?}", uri.path());
        println!("The file_path passed is: {:?}", file_path.unwrap_or("null"));

        let file_path = match file_path {
            Some(val) => val.to_string(),
            None => {
                // 当前目录
                let current_dir = current_dir()?;
                // 文件名
                let file_name = format!("rs_download_{:x}", md5::compute(uri.to_string()));
                current_dir.as_path().join(file_name).to_str().unwrap().to_string()
            },
        };

        println!("final file_path={file_path}");

        // 检查文件是否已存在
        if Path::new(&file_path).exists() {
            return Err(anyhow!("文件 `{}` 已存在", file_path));
        }

        let temp_file_dir = temp_dir().join(Uuid::new_v4().to_string());
        println!("temp_file_dir={:?}", temp_file_dir);
       

        Ok(Self {
            size,
            uri,
            file_path,
            temp_file_dir,
        })
    }
}