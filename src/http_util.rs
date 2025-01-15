use curl::easy::{Easy, List};
use std::fs::File;
use std::io::Write;

fn get_easy() -> Result<Easy, String> {
    let mut easy = Easy::new();
    let mut list = List::new();
    list.append("user-agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0").unwrap();
    match easy.http_headers(list) {
        Ok(_) => {}
        Err(e) => {
            return Err(e.to_string());
        }
    }
    Ok(easy)
}

pub fn download_file(url: &str, mut file: File) -> Result<(), String>{
    let easy_result = get_easy();
    let mut easy = match easy_result {
        Ok(e) => {e}
        Err(err) => {
            return Err(err);
        }
    };
    match easy.url(&url) {
        Ok(_) => {}
        Err(e) => {
            return Err(e.to_string());
        }
    };
    match easy.get(true) {
        Ok(_) => {}
        Err(e) => {
            return Err(e.to_string());
        }
    };
    match easy.progress(true) {
        Ok(_) => {}
        Err(e) => {
            return Err(e.to_string());
        }
    };
    match easy.progress_function(|total_download_size, current_download_size, _total_upload_size, _current_upload_size|{
        if total_download_size > 0.0 {
            println!("已下载：{:.2}%", current_download_size / total_download_size * 100.0);
        }else {
            println!("已下载：0.00%");
        }
        true
    }) {
        Ok(_) => {}
        Err(e) => {
            return Err(e.to_string());
        }
    };
    match easy.write_function(move |data|{
        match file.write_all(data) {
            Ok(_) => {}
            Err(e) => {
                println!("写入错误：{}", e.to_string());
            }
        }
        Ok(data.len())
    }) {
        Ok(_) => {}
        Err(e) => {
            return Err(e.to_string());
        }
    }
    match easy.perform() {
        Ok(_) => {}
        Err(e) => {
            return Err(e.to_string());
        }
    };
    Ok(())
}

pub fn get_and_to_string(url: &str) -> Result<String, String> {
    let easy_result = get_easy();
    let mut easy = match easy_result {
        Ok(e) => {e}
        Err(err) => {
            return Err(err);
        }
    };

    match easy.url(&url) {
        Ok(_) => {}
        Err(e) => {
            return Err(e.to_string());
        }
    };
    match easy.get(true) {
        Ok(_) => {}
        Err(e) => {
            return Err(e.to_string());
        }
    };

    let mut chars = Vec::new();

    {
        let mut transfer = easy.transfer();
        match transfer.write_function(|new_data| {
            chars.extend_from_slice(new_data);
            Ok(new_data.len())
        }) {
            Ok(_) => {}
            Err(e) => {
                return Err(e.to_string());
            }
        }
        match transfer.perform() {
            Ok(_) => {}
            Err(e) => {
                return Err(e.to_string());
            }
        };
    }

    Ok( String::from_utf8(chars).unwrap())
}