use bytes::Bytes;
use mini_redis::{Command, Connection, Frame};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        let db = Arc::clone(&db);

        tokio::spawn(async {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    // The `Connection` lets us read/write redis **frames** instead of
    // byte streams. The `Connection` type is defined by mini-redis.
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        let response = match Command::from_frame(frame).unwrap() {
            Command::Set(cmd) => {
                db.lock()
                    .unwrap()
                    .insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Command::Get(cmd) => {
                if let Some(value) = db.lock().unwrap().get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => Frame::Error(format!("unimplemented {:?}", cmd)),
        };

        connection.write_frame(&response).await.unwrap();
    }
}
