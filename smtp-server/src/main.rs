mod tests;

#[derive(Debug, PartialEq)]
enum Message {
    HELO(String),
}

struct ServerObj {
    name: String,
}

trait Server {
    type Msg;
    fn send(&self, msg: Self::Msg) -> Self::Msg;
}

impl ServerObj {
    fn new(name: String) -> Self {
        ServerObj { name }
    }
}

impl Server for ServerObj {
    type Msg = Message;

    fn send(&self, msg: Self::Msg) -> Self::Msg {
        match msg {
            Message::HELO(str) => Message::HELO(self.name.clone()),
        }
    }
}

fn main() {}
