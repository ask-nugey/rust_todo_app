use axum::{extract::State, routing::get, Json, Router};
use serde::Serialize;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    // データベースの初期化
    let db = initialize_db();

    // アプリケーションハンドラーの作成
    // ルートの設定
    let app = Router::new()
        .route("/todos", get(todos_index)) // todos_indexは引数にdbを受け取る
        .with_state(db); // データベースの状態を共有

    // 127.0.0.1:3000でTCPリスナー（ソケット）をバインド（＝接続の受け取り先を設定）
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    let addr = listener.local_addr().unwrap();

    println!("🤙 listening on {}", addr);

    // axumサーバーを起動
    axum::serve(listener, app).await.unwrap();
}

// Arc（Atomic Reference Counting）: 複数のスレッド間で同一データを共有する仕組みを提供
// RwLock（Read-Write Lock）: 読み書きのアクセス制御を提供
type Db = Arc<RwLock<Vec<Todo>>>;

#[derive(Debug, Clone, Serialize)]
struct Todo {
    id: Uuid,
    title: String,
    is_completed: bool,
}

// 初期データの設定
fn initialize_db() -> Db {
    let todos = vec![
        Todo {
            id: Uuid::new_v4(),
            title: String::from("Sample Todo 1"),
            is_completed: false,
        },
        Todo {
            id: Uuid::new_v4(),
            title: String::from("Sample Todo 2"),
            is_completed: true,
        },
    ];

    // 共有データの初期化
    Arc::new(RwLock::new(todos))
}

// todoの一覧を取得（read）
async fn todos_index(State(db): State<Db>) -> Json<Vec<Todo>> {
    let todos = db.read().unwrap();
    Json(todos.clone()) // 各スレッドでデータを共有するためにcloneが必要
}
