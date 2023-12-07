use mysql_async::Pool;




pub async fn get_connect(db_type: &str) -> Pool {
    // 设置连接选项
    let db = format!("mysql://test:test@192.168.2.189:3306/{}",db_type);
    let opts = db.as_str();
    let pool = Pool::new(opts);
    pool
}


