use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(example = json!({"bookSourceUrl": "http://www.77dushu.com#未月十八repair", "lastUpdateTime": "1670071887256"}))]
pub struct SourceItem {
    /// 书源标识
    #[serde(rename = "bookSourceUrl")]
    pub book_source_url: String,
    /// 上次更新时间
    #[serde(rename = "lastUpdateTime")]
    pub last_update_time: u64,
}
