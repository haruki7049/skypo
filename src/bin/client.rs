use clap::Parser;
use futures_util::{StreamExt, SinkExt, stream::{SplitSink, SplitStream}};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_tungstenite::{connect_async, tungstenite::Message, WebSocketStream};

// 通信のはじめ
// write: 送信用の関数
// msg: メッセージ
async fn on_open(
    write: &mut SplitSink<WebSocketStream<impl AsyncRead + AsyncWrite + Unpin>, Message>,
    msg: &str
) {
    let request_msg = Message::Text(format!("{}", msg).into());
    write.send(request_msg).await.expect("Failed to send registration message");
}

// メッセージの送受信
// 接続したwebsocketのやり取りを行う
// read: 受信用の関数
// write: 送信用の関数
async fn on_message(
    mut read: SplitStream<WebSocketStream<impl AsyncRead + AsyncWrite + Unpin>>,
    mut write: SplitSink<WebSocketStream<impl AsyncRead + AsyncWrite + Unpin>, Message>
) {
    while let Some(message) = read.next().await {
        match message {
            Ok(msg) => {
                recive_send(&mut write, &msg).await;
            },
            Err(e) => {
                eprintln!("Error reading message: {:?}", e);
            }
        }
    }
}
// メッセージを受けたあとの処理
// ここでは、メッセージを受け取ったら、適当なメッセージを返すようにしている
async fn recive_send(
    write: &mut SplitSink<WebSocketStream<impl AsyncRead + AsyncWrite + Unpin>, Message>,
    msg: &Message,
){
    write.send(Message::Text(format!("{}", "hogehoge").into())).await.expect("Failed to send message");
    println!("Received a message: {}", msg);
}

pub async fn client(url: &str) {
    // URLのセッティング
    println!("Connecting to - {}", url);
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("Connected to Agent Network");

    // 受信と送信用関数を切り離し
    let (mut write, read) = ws_stream.split();

    // wss://echo.websocket.eventsには必要ないが、オブジェクトの描き方がてら、適当な文字を入れておく。
    let start_msg = r#"{
        "context": "message",
        "object_tmp": {
            "hoge": "hoge"
        }
    }"#;
    // websocketの接続
    // register the weboscoket
    on_open(&mut write, start_msg).await;

    // メッセージの受信用にスレッドを開始
    // Handle incoming messages in a separate task
    let on_message = tokio::spawn(async move {
        on_message(read, write).await;
    });

    // tokioにスレッドをぶん投げ
    // Await both tasks (optional, depending on your use case)
    let _ = tokio::try_join!(on_message);
}

#[derive(Debug, Parser)]
struct CLIArgs {
    url: String,
}

#[tokio::main]
async fn main() {
    let args: CLIArgs = CLIArgs::parse();
    client(&args.url).await;
}
