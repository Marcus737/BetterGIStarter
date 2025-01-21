use crate::error::Result;
use curl::easy::{Easy, List, SslOpt};
use std::fs::File;
use std::io::Write;

fn get_easy() -> Result<Easy> {
    let mut easy = Easy::new();
    let mut list = List::new();
    list.append("user-agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0")?;
    easy.http_headers(list)?;
    let mut opt = SslOpt::new();
    //跳过证书检查
    let opt = opt.no_revoke(true);
    easy.ssl_options(opt)?;
    Ok(easy)
}

pub fn download_file(url: &str, mut file: File) -> Result<()>{
    let mut easy = get_easy()?;
    easy.url(&url)?;
    easy.get(true)?;
    easy.progress(true)?;



    easy.progress_function(move |total_download_size, current_download_size, _total_upload_size, _current_upload_size|{
        if total_download_size > 0.0 {
            // info!("已下载：{:.2}%\r", current_download_size / total_download_size * 100.0);
            print!("已下载：{:.2}%\r", current_download_size / total_download_size * 100.0);
        }else {
            // info!("已下载：0.00%\r");
            print!("已下载：0.00%\r");
        }

        true
    })?;
    easy.write_function(move |data|{
        match file.write_all(data) {
            Ok(_) => {}
            Err(e) => {
                println!("写入错误：{}", e.to_string());
            }
        }
        Ok(data.len())
    })?;
    easy.perform()?;

    Ok(())
}

pub fn get_and_to_string(url: &str) -> Result<String> {
    let mut easy = get_easy()?;

    easy.url(&url)?;
    easy.get(true)?;
    let mut chars = Vec::new();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|new_data| {
            chars.extend_from_slice(new_data);
            Ok(new_data.len())
        })?;
        transfer.perform()?;
    }
    Ok(String::from_utf8(chars)?)
}