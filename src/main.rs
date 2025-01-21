//启动流程
//若bettergi已经打开，关闭
//判断是否需要更新，若需要，进行更新流程
//启动betterGI

//更新流程
//获取最新的tag版本，比较本地的tag版本
//若需要更新，获取最新版本的下载链接

mod http_util;
mod config;
mod error;

use std::env::set_var;
use crate::error::{Error, Result};
use crate::config::{read_config, write_to_config, Config};
use crate::http_util::{download_file, get_and_to_string};
use directories::ProjectDirs;
use serde_json::Value;
use std::fs;
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use std::process::{Command};

use log::info;

fn get_project_dirs() -> Result<ProjectDirs> {
    ProjectDirs::from("", "", "BetterGI-Starter").ok_or(
        Error::Message(
            *Box::new("获取project dirs失败".to_string().into_boxed_str())
        )
    )

}

fn kill_process(process_name: &str) -> Result<()> {
    Command::new("taskkill")
        .arg("/t")
        .arg("/im")
        .arg(process_name)
        .spawn()?;
    Ok(())
}

fn latest_version(owner: &str, repo_name: &str) -> Result<f64> {
    let url = format!("https://api.github.com/repos/{}/{}/tags?per_page=1", owner, repo_name);
    let s = get_and_to_string(&url)?;
    let json_value:Value = serde_json::from_str(&s)?;
    let tag = json_value[0]["name"].to_string().replace("\"", "");
    info!("最新版本：{}", &tag);
    Ok(tag.parse::<f64>()?)
}

fn update_version(owner: &str, repo_name: &str, project_dirs: &ProjectDirs) -> Result<PathBuf>{
    info!("开始更新版本");
    //获取最新版本的下载链接
    let url = format!("https://api.github.com/repos/{}/{}/releases?per_page=1", owner, repo_name);
    let req_result =  get_and_to_string(&url)?;
    let json_value:Value = serde_json::from_str(&req_result)?;
    let assets = &json_value[0]["assets"];
    let mut download_url_7z = None;
    match assets.as_array() {
        None => {
            return Err(Error::Message("获取数组失败".to_string().into_boxed_str()));
        }
        Some(asses) => {
            for ass in asses {
                let download_url = match ass["browser_download_url"].as_str() {
                    None => {
                        return Err(Error::Message("as_str fail".to_string().into_boxed_str()));
                    }
                    Some(v) => {v}
                };
                if download_url.ends_with(".7z") {
                    download_url_7z = Some(download_url);
                    break;
                }
            }
        }
    }
    match download_url_7z {
        None => {
            Err(Error::Message("未找到.7z的下载地址".to_string().into_boxed_str()))
        }
        Some(v) => {
            info!("下载地址：{}", v);
            let save_path = project_dirs.data_dir();
            info!("开始下载，保存路径：{}", save_path.to_str().unwrap());
            download(v, save_path)
        }
    }
}

fn download(url: &str, save_path: &Path) -> Result<PathBuf> {
    //创建父目录
    fs::create_dir_all(&save_path)?;

    //获取文件名字
    let index = match url.rfind("/") {
        None => {
            return Err(Error::Message(format!("未能找到url中的斜杠\n{}", &url).to_string().into_boxed_str()))
        }
        Some(v) => {v}
    };
    let filename = &url[index+1..];
    let file_path = save_path.join(filename);
    //创建文件
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&file_path)?;
    //使用国内镜像加速
    let proxy_url = format!("https://gh.llkk.cc/{}", url);
    match download_file(&proxy_url, file) {
        Ok(_) => {
            Ok(file_path)
        }
        Err(e) => {
            Err(e)
        }
    }
}


fn open_bettergi(path_buf: PathBuf) -> Result<()> {
    let path_str = path_buf.to_str().ok_or(
        Error::Message("path_buf转换字符串失败".to_string().into_boxed_str())
    )?;
    info!("打开程序：{}", &path_str);
    Command::new(&path_str).spawn()?;
    Ok(())
}

fn init(project_dirs: &ProjectDirs)-> Result<()>{
    fs::create_dir_all(project_dirs.config_dir())?;
    fs::create_dir_all(project_dirs.data_dir())?;
    //创建config文件
    let config_path = project_dirs.config_dir().join("config.txt");
    if !config_path.exists() {
        //不存在，创建
        let config = Config::new();
        File::create(config_path)?;
        write_to_config(project_dirs, &config)?;
    }
    Ok(())
}

fn main() -> Result<()> {

    set_var("RUST_LOG", "INFO");

    env_logger::init();


    let proj_dirs = get_project_dirs()?;

    init(&proj_dirs)?;

    let mut config = read_config(&proj_dirs)?;

    let _result = kill_process("BetterGI.exe");

    let latest_ver = latest_version("babalae", "better-genshin-impact")?;

    if latest_ver > config.version {
        let p = update_version("babalae", "better-genshin-impact", &proj_dirs)?;
        let path_str = p.to_str().ok_or(
            Error::Message("p转换字符串失败".to_string().into_boxed_str())
        )?;
        info!("正在解压：{}", &path_str);
        sevenz_rust::decompress_file(&p, proj_dirs.data_dir())?;
        info!("删除文件：{}", &path_str);
        fs::remove_file(&p)?;
        //写入配置文件
        config.version = latest_ver;
        write_to_config(&proj_dirs, &config)?;
    }

    let opgengi_exe_path = proj_dirs.data_dir().join("BetterGI").join("BetterGI.exe");
    open_bettergi(opgengi_exe_path)?;

    Ok(())
}
