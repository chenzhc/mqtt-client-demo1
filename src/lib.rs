use log::info;
use sqlx::{sqlite::SqliteQueryResult, Sqlite, SqlitePool};
use sqlx::{migrate::MigrateDatabase};

// init log config
pub fn init() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("INFO");
    // let _ = env_logger::builder()
    //     .target(env_logger::Target::Stdout)
    //     .filter_level(log::LevelFilter::Trace)
    //     .is_test(true)
    //     .try_init();
}


pub async fn create_schema(db_url: &str) -> Result<SqliteQueryResult, sqlx::Error> {
    // 创建一个连接到db_url的连接池
    let pool = SqlitePool::connect(db_url).await?;
    info!("test");

      // 定义数据库表
    let qry = 
    "PRAGMA foreign_keys = ON;
    CREATE TABLE IF NOT EXISTS settings
    (
        settings_id     INTEGER PRIMARY KEY NOT NULL,
        description     TEXT                NOT NULL,
        created_on      DATETIME DEFAULT    (datetime('now', 'localtime')),
        updated_on      DATETIME DEFAULT    (datetime('now', 'localtime')),
        done            BOOLEAN             NOT NULL DEFAULT 0
    );
    CREATE TABLE IF NOT EXISTS project
    (
        project_id      INTEGER PRIMARY KEY AUTOINCREMENT,
        product_name    TEXT,
        created_on      DATETIME DEFAULT    (datetime('now', 'localtime')),
        updated_on      DATETIME DEFAULT    (datetime('now', 'localtime')),
        img_directory   TEXT     NOT NULL,
        out_directory   TEXT     NOT NULL,
        status          TEXT     NOT NULL,
        settings_id     INTEGER  NOT NULL DEFAULT 1,
        FOREIGN KEY (settings_id) REFERENCES settings (settings_id) ON UPDATE SET NULL ON DELETE SET NULL
    );";

     // 运行
    let result = sqlx::query(qry).execute(&pool).await;
    // 关闭连接池
    pool.close().await; 
    result
}

pub async fn test_sqlite_db() {
    log::trace!("Commencing yak shaving");
    info!("Thi***************.");
      // 在项目根目录下创建一个数据库名为'sqlite.db'文件
    let db_url = String::from("sqlite://sqlite.db");

    // 如果数据库不存在，则创建它。
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false){
        Sqlite::create_database(&db_url).await.unwrap();

        //如果存在，则调用create_schema
        match create_schema(&db_url).await {
            // 如果一切顺利，打印OK…否则panic
            Ok(_) => println!("database created succesfully"),
            Err(e) => panic!("{}", e)
        }
    }

    // 连接数据库
    let instances = SqlitePool::connect(&db_url).await.unwrap();

    // 在settings表的description字段插入"testing"
    let qry = "INSERT INTO settings (description) VALUES($1)";
    let result = sqlx::query(qry).bind("testing").execute(&instances).await;
    info!("result: {:?}", result);
    // 关闭数据库
    instances.close().await;
    
}



#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_test_db01() {
        crate::init();
        let _ = test_sqlite_db().await;
        info!("This is an info message from the test.");
    }
}