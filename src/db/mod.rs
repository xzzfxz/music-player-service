use diesel::{mysql::MysqlConnection, Connection};
use dotenv::dotenv;

/// 获取mysql连接
pub fn get_connection() -> MysqlConnection {
    dotenv().ok();
    let value = dotenv::var("MYSQL_URL").expect("获取mysql连接失败");
    MysqlConnection::establish(&value).expect("建立mysql连接失败")
}
