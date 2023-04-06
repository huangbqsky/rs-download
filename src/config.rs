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
            .arg(Arg::with_name("size")
                // .short('f')
                // .long("size")
                // .takes_value(true)
                .help("并发任务数"))
            .arg(Arg::with_name("uri")
                // .short('u')
                // .long("uri")
                // .takes_value(true)
                .help("下载地址"))
            .arg(Arg::with_name("file-path")
                // .short('p')
                // .long("file-path")
                // .takes_value(true)
                .help("保存路径"))
            .get_matches();
            // .args(&[
            //     Arg::new("size").help("并发任务数量").required(true),
            //     Arg::new("uri").help("资源 URI").required(true),
            //     Arg::new("file-path").help("保存文件路径").required(false),
            // ])
            // .get_matches();

        let size = matches.value_of_t("size")?;
        let uri: Uri = matches.value_of_t("uri")?;
        let file_path = matches.value_of("file-path");
      
        println!("you input file_path is {}", file_path.unwrap_or("null"));

        let file_path = match file_path {
            Some(val) => val.to_string(),
            None => {
                // 当前目录
                let current_dir = current_dir()?;
                let path = Path::new(uri.path().trim());
                let file_name = path.file_name();
                // 文件名
                let file_name = if file_name.is_none() { // 解析等 file_name 为空，则 md5 url 作为文件名
                    format!("rs_download_{:x}", md5::compute(uri.to_string()))
                } else {
                    format!("rs_download_{}", file_name.unwrap().to_str().unwrap())  
                };
                // path = dir + file_name
                let defalut_path = current_dir.as_path().join(file_name).to_str().unwrap().to_string();
                println!("defalut file_path = {}", defalut_path);
                defalut_path
            },
        };

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