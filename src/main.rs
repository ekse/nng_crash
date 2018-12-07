use std::thread;
use std::time::Duration;

use nng::{Aio, Context, Message, Protocol, Socket};

const URL_CLIENT: &str = "ipc://nng_client";
const URL_SERVER: &str =  "ipc://nng_server";

fn client() -> Result<(), nng::Error> {
    let url_rep = URL_CLIENT;
    let url_req = URL_SERVER;

    let mut rep = Socket::new(Protocol::Rep0)?;
    rep.listen(url_rep)?;

    let mut req = Socket::new(Protocol::Req0)?;
    req.dial(url_req)?;
    let _ = req.send(Message::new()?);
    println!("message sent");

    println!("waiting for connect back");
    let _msg = rep.recv()?;

    loop {
        let _ = req.send(Message::new()?);
        println!("message sent");
        thread::sleep(Duration::from_millis(100));
    }
}

fn server() -> Result<(), nng::Error> {
    let url_rep = URL_SERVER;
    let url_req = URL_CLIENT;

    let mut rep = Socket::new(Protocol::Rep0)?;
    rep.listen(url_rep)?;
    let msg = rep.recv()?;

    println!("connection on {}, connecting back to client", url_rep);
    
    let mut req = Socket::new(Protocol::Req0)?;
    req.dial(url_req)?;
    let _ = req.send(Message::new()?);
    println!("message sent");
    let context = Context::new(&rep)?;

    let aio = Aio::with_callback(move |aio: &Aio| {
        if let Some(msg) = aio.get_msg() {
            println!("msg {:?}", msg.body());
        }
    })?;

    loop {
        context.recv(&aio)?;
        //thread::sleep(Duration::from_millis(100));
    }
}

fn main() -> Result<(), nng::Error> {
    let _server = thread::spawn(server);
    thread::sleep(Duration::from_millis(100));
    client()?;
    Ok(())
}
