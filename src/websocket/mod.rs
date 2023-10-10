use crate::*;
use actix_web_actors::ws;
use actix::{Actor, StreamHandler};
use actix_web::Error;
use actix_web_actors::ws::{Message, ProtocolError};

struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<Message, ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<Message, ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(Message::Ping(msg)) => {ctx.pong(&msg)},
            Ok(Message::Text(text)) => ctx.text(text),
            Ok(Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

// 测试发现只能用http，需要关闭https；
#[get("/ws/")]
pub async fn ws_server(req:HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}
