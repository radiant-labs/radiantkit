#![cfg(not(target_arch = "wasm32"))]

use futures_util::stream::SplitStream;
use futures_util::SinkExt;
use futures_util::{ready, stream::SplitSink, Sink, Stream, StreamExt};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::task;
use tokio::{net::TcpStream, sync::RwLock};
use tokio_tungstenite::{tungstenite, MaybeTlsStream, WebSocketStream};
use tungstenite::Message;
use y_sync::awareness::Awareness;
use y_sync::net::Connection;
use y_sync::sync::Error;
use yrs::updates::encoder::Encode;
use yrs::UpdateSubscription;

struct TungsteniteSink(SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>);

impl Sink<Vec<u8>> for TungsteniteSink {
    type Error = Error;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let sink = unsafe { Pin::new_unchecked(&mut self.0) };
        let result = ready!(sink.poll_ready(cx));
        match result {
            Ok(_) => Poll::Ready(Ok(())),
            Err(e) => Poll::Ready(Err(Error::Other(Box::new(e)))),
        }
    }

    fn start_send(mut self: Pin<&mut Self>, item: Vec<u8>) -> Result<(), Self::Error> {
        let sink = unsafe { Pin::new_unchecked(&mut self.0) };
        let result = sink.start_send(Message::binary(item));
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::Other(Box::new(e))),
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let sink = unsafe { Pin::new_unchecked(&mut self.0) };
        let result = ready!(sink.poll_flush(cx));
        match result {
            Ok(_) => Poll::Ready(Ok(())),
            Err(e) => Poll::Ready(Err(Error::Other(Box::new(e)))),
        }
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let sink = unsafe { Pin::new_unchecked(&mut self.0) };
        let result = ready!(sink.poll_close(cx));
        match result {
            Ok(_) => Poll::Ready(Ok(())),
            Err(e) => Poll::Ready(Err(Error::Other(Box::new(e)))),
        }
    }
}

struct TungsteniteStream(SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>);
impl Stream for TungsteniteStream {
    type Item = Result<Vec<u8>, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let stream = unsafe { Pin::new_unchecked(&mut self.0) };
        let result = ready!(stream.poll_next(cx));
        match result {
            None => Poll::Ready(None),
            Some(Ok(msg)) => Poll::Ready(Some(Ok(msg.into_data()))),
            Some(Err(e)) => Poll::Ready(Some(Err(Error::Other(Box::new(e))))),
        }
    }
}

pub struct NativeConnection {
    _connection: Option<Connection<TungsteniteSink, TungsteniteStream>>,
    awareness: Arc<RwLock<Awareness>>,
    _sub: Option<UpdateSubscription>,
}

impl NativeConnection {
    pub async fn new(
        awareness: Arc<RwLock<Awareness>>,
        url: &str,
    ) -> Result<Arc<parking_lot::RwLock<Self>>, ()> {
        let Ok((ws_stream, _)) = tokio_tungstenite::connect_async(url).await else {
            return Err(());
        };
        let (sink, stream) = ws_stream.split();
        let connection = Connection::new(
            awareness.clone(),
            TungsteniteSink(sink),
            TungsteniteStream(stream),
        );

        let sub = {
            let sink = connection.sink();
            let a = connection.awareness().write().await;
            let doc = a.doc();
            doc.observe_update_v1(move |_txn, e| {
                let update = e.update.to_owned();
                if let Some(sink) = sink.upgrade() {
                    task::spawn(async move {
                        let msg =
                            y_sync::sync::Message::Sync(y_sync::sync::SyncMessage::Update(update))
                                .encode_v1();
                        let mut sink = sink.lock().await;
                        sink.send(msg).await.unwrap();
                    });
                }
            })
            .unwrap()
        };

        Ok(Arc::new(parking_lot::RwLock::new(Self {
            _connection: Some(connection),
            awareness,
            _sub: Some(sub),
        })))
    }

    pub fn awareness(&self) -> Arc<RwLock<Awareness>> {
        self.awareness.clone()
    }
}
