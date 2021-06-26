use crate::error_handler::CustomError;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
// 初回アクセス時に一回だけ初期化処理が実行されるグローバル変数を作成することができる
use lazy_static::lazy_static;
// データベースと正常につながっているかをチェックする関数？
use r2d2;
use std::env;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

embed_migrations!();

lazy_static! {
    // 文法上の特徴: "static" ではなく "static ref" を使う
    // POOLというグローバル変数を作成
    static ref POOL: Pool = {
        let db_url = env::var("DATABASE_URL").expect("Database url not set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("Failed to create db pool")
    };
}

pub fn init() {
  // グローバル変数の初期化？
  lazy_static::initialize(&POOL);
  // connectionがうまく行ったかの返り値を取得
  let conn = connection().expect("Failed to get db connection");
  embedded_migrations::run(&conn).unwrap();
}

pub fn connection() -> Result<DbConnection, CustomError> {
  POOL.get()
    // 失敗した時に中の値を加工したい場合に使用
    .map_err(|e| CustomError::new(500, format!("Failed getting db connection: {}", e)))
}