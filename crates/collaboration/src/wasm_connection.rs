#![cfg(target_arch = "wasm32")]

use parking_lot::RwLock;
use std::sync::Arc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use web_sys::{MessageEvent, WebSocket};
use y_sync::awareness::Awareness;
use y_sync::sync::{DefaultProtocol, Error, Message, MessageReader, Protocol, SyncMessage};
use yrs::encoding::read::Cursor;
use yrs::updates::decoder::{Decode, DecoderV1};
use yrs::updates::encoder::{Encode, Encoder, EncoderV1};
use yrs::Update;
use yrs::UpdateSubscription;

#[derive(Debug)]
pub struct Connection {
    awareness: Arc<RwLock<Awareness>>,
    ws: WebSocket,
}

impl Connection {
    #[allow(dead_code)]
    pub async fn send(&self, msg: Vec<u8>) -> Result<(), Error> {
        let _ = self.ws.send_with_u8_array(&msg);
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn cslose(self) -> Result<(), Error> {
        let _ = self.ws.close();
        Ok(())
    }
}

impl Connection {
    pub fn new(awareness: Arc<RwLock<Awareness>>, ws: WebSocket) -> Result<Self, ()> {
        Self::with_protocol(awareness, DefaultProtocol, &ws).map_err(|_| ())
    }

    #[allow(dead_code)]
    pub fn awareness(&self) -> &Arc<RwLock<Awareness>> {
        &self.awareness
    }

    pub fn with_protocol<P>(
        awareness: Arc<RwLock<Awareness>>,
        protocol: P,
        ws: &WebSocket,
    ) -> Result<Self, Error>
    where
        P: Protocol + Send + Sync + 'static,
    {
        let loop_awareness = Arc::downgrade(&awareness);
        let payload = {
            let awareness = loop_awareness.upgrade().unwrap();
            let mut encoder = EncoderV1::new();
            let awareness = awareness.read();
            protocol.start(&awareness, &mut encoder)?;
            encoder.to_vec()
        };

        if !payload.is_empty() {
            if let Err(e) = ws.send_with_u8_array(&payload) {
                log::error!("connection failed to send back the reply {:?}", e);
                return Err(Error::Unsupported(1));
            } else {
                // console_log!("connection send back the reply");
                // return Err(Error::Unsupported(2)); // parent ConnHandler has been dropped
            }
        }

        let ws_clone = ws.clone();
        let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
            if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                let array = js_sys::Uint8Array::new(&abuf);
                // let len = array.byte_length() as usize;
                let data = array.to_vec();

                if let Some(awareness) = loop_awareness.upgrade() {
                    match Self::process(&protocol, &awareness, &ws_clone, data) {
                        Ok(()) => { /* continue */ }
                        Err(e) => {
                            log::error!("connection failed to process {:?}", e);
                        }
                    }
                } else {
                    // return Ok(()); // parent ConnHandler has been dropped
                }
            } else if let Ok(blob) = e.data().dyn_into::<web_sys::Blob>() {
                // better alternative to juggling with FileReader is to use https://crates.io/crates/gloo-file
                let fr = web_sys::FileReader::new().unwrap();
                let fr_c = fr.clone();
                // create onLoadEnd callback
                let cl = loop_awareness.clone();
                let ws_clone = ws_clone.clone();
                let onloadend_cb =
                    Closure::<dyn FnMut(_)>::new(move |_e: web_sys::ProgressEvent| {
                        let array = js_sys::Uint8Array::new(&fr_c.result().unwrap());
                        // let len = array.byte_length() as usize;

                        let data = array.to_vec();
                        if let Some(awareness) = cl.upgrade() {
                            let protocol = DefaultProtocol;
                            match Self::process(&protocol, &awareness, &ws_clone, data) {
                                Ok(()) => { /* continue */ }
                                Err(e) => {
                                    log::error!("connection failed to process {:?}", e);
                                }
                            }
                        } else {
                            // return Ok(()); // parent ConnHandler has been dropped
                        }
                        // here you can for example use the received image/png data
                    });
                fr.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
                fr.read_as_array_buffer(&blob).expect("blob not readable");
                onloadend_cb.forget();
            }
        });

        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();

        Ok(Connection {
            awareness,
            ws: ws.clone(),
        })
    }

    fn process<P: Protocol>(
        protocol: &P,
        awareness: &Arc<RwLock<Awareness>>,
        ws: &WebSocket,
        input: Vec<u8>,
    ) -> Result<(), Error> {
        let mut decoder = DecoderV1::new(Cursor::new(&input));
        let reader = MessageReader::new(&mut decoder);
        for r in reader {
            let msg = r?;
            if let Some(reply) = handle_msg(protocol, &awareness, msg)? {
                if let Err(e) = ws.send_with_u8_array(&reply.encode_v1()) {
                    log::error!("connection failed to send back the reply {:?}", e);
                    return Err(Error::Unsupported(0));
                } else {
                    log::error!("connection send back the reply");
                }
            }
        }
        Ok(())
    }
}

pub fn handle_msg<P: Protocol>(
    protocol: &P,
    a: &Arc<RwLock<Awareness>>,
    msg: Message,
) -> Result<Option<Message>, Error> {
    match msg {
        Message::Sync(msg) => match msg {
            SyncMessage::SyncStep1(sv) => {
                let awareness = a.read();
                protocol.handle_sync_step1(&awareness, sv)
            }
            SyncMessage::SyncStep2(update) => {
                let mut awareness = a.write();
                protocol.handle_sync_step2(&mut awareness, Update::decode_v1(&update)?)
            }
            SyncMessage::Update(update) => {
                let mut awareness = a.write();

                protocol.handle_update(&mut awareness, Update::decode_v1(&update)?)
            }
        },
        Message::Auth(reason) => {
            let awareness = a.read();
            protocol.handle_auth(&awareness, reason)
        }
        Message::AwarenessQuery => {
            let awareness = a.read();
            protocol.handle_awareness_query(&awareness)
        }
        Message::Awareness(update) => {
            let mut awareness = a.write();
            protocol.handle_awareness_update(&mut awareness, update)
        }
        Message::Custom(tag, data) => {
            let mut awareness = a.write();
            protocol.missing_handle(&mut awareness, tag, data)
        }
    }
}

pub struct WasmConnection {
    awareness: Arc<RwLock<Awareness>>,
    connection: Option<Connection>,
    _sub: UpdateSubscription,
}

impl WasmConnection {
    pub fn new(awareness: Arc<RwLock<Awareness>>, url: &str) -> Result<Arc<RwLock<Self>>, ()> {
        if let Ok(ws) = WebSocket::new(url) {
            let sub = {
                let a = awareness.write();
                let doc = a.doc();
                let cloned_ws = ws.clone();
                doc.observe_update_v1(move |_txn, e| {
                    log::error!("sending update");
                    let update = e.update.to_owned();
                    let msg =
                        y_sync::sync::Message::Sync(y_sync::sync::SyncMessage::Update(update))
                            .encode_v1();
                    if let Err(e) = cloned_ws.send_with_u8_array(&msg) {
                        log::error!("connection failed to send back the reply {:?}", e);
                    } else {
                        // console_log!("connection send back the reply");
                        // return Err(Error::Unsupported(2)); // parent ConnHandler has been dropped
                    }
                })
                .unwrap()
            };

            let wasm_connection = Arc::new(RwLock::new(WasmConnection {
                awareness: awareness.clone(),
                connection: None,
                _sub: sub,
            }));

            let cloned_wrapper = wasm_connection.clone();
            let cloned_ws = ws.clone();
            let cloned_awareness = awareness.clone();
            let onopen_callback = Closure::<dyn FnOnce()>::once(move || {
                let mut wrapper = cloned_wrapper.write();
                match Connection::new(cloned_awareness, cloned_ws) {
                    Ok(conn) => wrapper.connection = Some(conn),
                    Err(_) => return,
                }
            });

            ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
            onopen_callback.forget();

            Ok(wasm_connection)
        } else {
            return Err(());
        }
    }

    pub fn awareness(&self) -> Arc<RwLock<Awareness>> {
        self.awareness.clone()
    }
}
