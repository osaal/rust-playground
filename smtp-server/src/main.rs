mod tests;

#[derive(Debug, PartialEq)]
enum Message {
    HELO,
}

struct ServerObj {}

trait Server {
    type Msg;
    fn send(&self, msg: Self::Msg) -> Self::Msg;
}

impl ServerObj {
    fn new() -> Self {
        ServerObj {}
    }
}

impl Server for ServerObj {
    type Msg = Message;

    fn send(&self, msg: Self::Msg) -> Self::Msg {
        match msg {
            Message::HELO => Message::HELO,
        }
    }
}

fn main() {}
