//启动流程
//若bettergi已经打开，关闭
//判断是否需要更新，若需要，进行更新流程
//启动betterGI

//更新流程
//获取最新的tag版本，比较本地的tag版本
//若需要更新，获取最新版本的下载链接

mod http_util;
mod config;

use crate::http_util::{download_file, get_and_to_string};
use directories::ProjectDirs;
use serde_json::Value;
use std::fs;
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use std::process::{exit, Command};
use crate::config::{read_config, write_to_config, Config};

fn get_project_dirs() -> Result<ProjectDirs, String> {
    Ok(
        ProjectDirs::from("", "", "BetterGI-Starter").ok_or("获取project dirs失败")?
    )
}

fn kill_process(process_name: &str) {
    Command::new("taskkill")
        .arg("/t")
        .arg("/im")
        .arg(process_name)
        .spawn().unwrap();
}

fn latest_version(owner: &str, repo_name: &str) -> Result<f64, String> {
    let url = format!("https://api.github.com/repos/{}/{}/tags?per_page=1", owner, repo_name);
    match get_and_to_string(&url) {
        Ok(s) => {
            let json_value:Value = match serde_json::from_str(&s) {
                Ok(json) => {
                    json
                }
                Err(e) => {
                    return Err(e.to_string());
                }
            };
            let tag = json_value[0]["name"].to_string().replace("\"", "");
            println!("最新版本：{}", &tag);
            match tag.parse::<f64>() {
                Ok(v) => {
                    Ok(v)
                }
                Err(e) => {
                    Err(e.to_string())
                }
            }
        }
        Err(e) => {
            Err(e.to_string())
        }
    }
}

fn update_version(owner: &str, repo_name: &str, project_dirs: &ProjectDirs) -> Result<PathBuf, String>{
    println!("开始更新版本");
    //获取最新版本的下载链接
    let url = format!("https://api.github.com/repos/{}/{}/releases?per_page=1", owner, repo_name);
    let req_result = match get_and_to_string(&url) {
        Ok(v) => {v}
        Err(e) => {
            return Err(e)
        }
    };
    let json_value:Value = match serde_json::from_str(&req_result) {
        Ok(v) => {v}
        Err(e) => {
            return Err(e.to_string())
        }
    };
    let assets = &json_value[0]["assets"];
    let mut download_url_7z = None;
    match assets.as_array() {
        None => {
            return Err(String::from("获取数组失败"));
        }
        Some(asses) => {
            for ass in asses {
                let download_url = match ass["browser_download_url"].as_str() {
                    None => {
                        return Err(String::from("as_str fail"));
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
            Err(String::from("未找到.7z的下载地址"))
        }
        Some(v) => {
            println!("下载地址：{}", v);
            let save_path = project_dirs.data_dir();
            println!("开始下载，保存路径：{}", save_path.to_str().unwrap());
            download(v, save_path)
        }
    }
}

fn download(url: &str, save_path: &Path) -> Result<PathBuf, String> {
    //创建父目录
    match fs::create_dir_all(&save_path) {
        Ok(_) => {}
        Err(e) => {
            return Err(format!("创建目录失败:{}", e.to_string()));
        }
    };

    //获取文件名字
    let index = match url.rfind("/") {
        None => {
            return Err(format!("未能找到url中的斜杠\n{}", &url))
        }
        Some(v) => {v}
    };
    let filename = &url[index+1..];
    let file_path = save_path.join(filename);
    //创建文件
    let file = match OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&file_path) {
        Ok(f) => {f}
        Err(e) => {
            return Err(format!("创建文件失败：{}", e.to_string()));
        }
    };
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


fn open_bettergi(path_buf: PathBuf){
    println!("打开程序：{}", path_buf.to_str().unwrap());
    Command::new(path_buf.to_str().unwrap())
        .spawn().unwrap();
}

fn init(project_dirs: &ProjectDirs)-> Result<(),String>{
    match fs::create_dir_all(project_dirs.config_dir()) {
        Ok(_) => {}
        Err(e) => {
            return Err(format!("创建config目录失败：{}", e.to_string()))
        }
    }
    match fs::create_dir_all(project_dirs.data_dir()) {
        Ok(_) => {}
        Err(e) => {
            return Err(format!("创建data目录失败：{}", e.to_string()))
        }
    }
    //创建config文件
    let config_path = project_dirs.config_dir().join("config.txt");
    if !config_path.exists() {
        //不存在，创建
        let config = Config::new();
        match File::create(config_path) {
            Ok(_) => {}
            Err(e) => {
                return Err(format!("创建配置文件失败{}", e.to_string()))
            }
        }
        match write_to_config(project_dirs, &config) {
            Ok(_) => {}
            Err(e) => {
                return Err(e)
            }
        }
    }
    Ok(())
}

fn main() {

    let proj_dirs = get_project_dirs().unwrap_or_else(|s| {
        println!("{}", s);
        exit(1)
    });

    init(&proj_dirs).unwrap_or_else(|s| {
        println!("{}", s);
        exit(1)
    });

    let mut config = read_config(&proj_dirs).unwrap_or_else(|s| {
        println!("{}", s);
        exit(1)
    });

    kill_process("BetterGI.exe");

    let latest_ver = latest_version("babalae", "better-genshin-impact").unwrap_or_else(|s| {
        println!("{}", s);
        exit(1)
    });

    if latest_ver > config.version {
        let p = update_version("babalae", "better-genshin-impact", &proj_dirs);
        match p {
            Ok(pb) => {
                println!("正在解压：{}", pb.to_str().unwrap());
                match sevenz_rust::decompress_file(&pb, proj_dirs.data_dir()) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("解压失败：{}", e.to_string());
                        exit(1);
                    }
                };
                println!("删除文件：{}", pb.to_str().unwrap());
                match fs::remove_file(&pb) {
                    Ok(_) => {
                        println!("删除成功")
                    }
                    Err(e) => {
                        println!("删除失败：{}", e.to_string())
                    }
                }
                //写入配置文件
                config.version = latest_ver;
                match write_to_config(&proj_dirs, &config) {
                    Ok(_) => {
                        println!("配置文件更新成功")
                    }
                    Err(e) => {
                        println!("配置文件更新失败{}", e);
                    }
                }
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    let opgengi_exe_path = proj_dirs.data_dir().join("BetterGI").join("BetterGI.exe");
    open_bettergi(opgengi_exe_path);


}
