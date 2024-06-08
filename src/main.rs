use axum::{routing::get, Json, Router};
use serde::Serialize;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    // アプリケーションハンドラーの作成
    // ルートの設定
    let app = Router::new().route("/todos", get(todos_index));

    // 127.0.0.1:3000でTCPリスナー（ソケット）をバインド（＝接続の受け取り先を設定）
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    let addr = listener.local_addr().unwrap();

    println!("🤙 listening on {}", addr);

    // axumサーバーを起動
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Clone, Serialize)]
struct Todo {
    id: Uuid,
    title: String,
    is_completed: bool,
}

// todoの一覧を取得（read）
async fn todos_index() -> Json<Vec<Todo>> {
    // ハードコードされたTodoリスト
    // TODO: HashMapのような簡易的なDBを作成する
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

    Json(todos)
}
