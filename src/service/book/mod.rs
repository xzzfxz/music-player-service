use anyhow::Result;
use serde_json::Value;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{Read, Write},
};

use crate::model::book::SourceItem;

/**
 * @description: 书源去重
 * @param {*} Result
 * @return {*}
 */
pub fn delete_repeat() -> Result<()> {
    let folder = fs::read_dir("assets/oldSource")?;
    let mut pre_len = 0;
    let mut book_map: HashMap<String, Value> = HashMap::new();

    for entry in folder {
        if let Ok(entry) = entry {
            let mut file = File::open(entry.path())?;
            // 将文件内容转换成字符串
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            // 转换为json格式的数据
            let raw_list: Vec<Value> = serde_json::from_str(&contents)?;
            pre_len = pre_len + raw_list.len();

            // 遍历生成map去重
            for item in raw_list.iter() {
                let source_item: SourceItem = serde_json::from_value(item.clone())?;
                let ori_url = source_item.book_source_url;
                let arr: Vec<&str> = ori_url.split("#").collect();
                let mut url = arr[0];
                if url.chars().last() == Some('/') {
                    url = &url[..url.len() - 1];
                }

                // 判断更新时间
                if book_map.contains_key(url) {
                    let pre_value = book_map.get(url).unwrap();
                    let pre_item: SourceItem = serde_json::from_value(pre_value.clone())?;
                    // 使用最近更新的
                    if pre_item.last_update_time < source_item.last_update_time {
                        book_map.insert(url.to_string(), item.clone());
                    }
                } else {
                    book_map.insert(url.to_string(), item.clone());
                }
            }
        }
    }

    let last_list: Vec<Value> = book_map.into_values().collect();
    // 生成字符串，准备写入文件
    let list_str = serde_json::to_string(&last_list)?;
    let mut file = File::create("assets/lastSource.json")?;
    file.write_all(list_str.as_bytes())?;
    println!("去重前长度: {}, 去重后长度: {}", pre_len, last_list.len());
    Ok(())
}
