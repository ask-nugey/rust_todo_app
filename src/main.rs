use axum::Router;

#[tokio::main]
async fn main() {
    // アプリケーションハンドラーの作成
    let app = Router::new();

    // 127.0.0.1:3000でTCPリスナー（ソケット）をバインド（＝接続の受け取り先を設定）
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    let addr = listener.local_addr().unwrap();

    println!("🤙 listening on {}", addr);

    // axumサーバーを起動
    axum::serve(listener, app).await.unwrap();
}
