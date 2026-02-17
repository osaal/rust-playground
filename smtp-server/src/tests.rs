mod tests {
    use crate::{Message, Server, ServerObj};

    #[test]
    fn receives_msg() {
        let input = Message::HELO;
        let output = Message::HELO;

        let server = ServerObj::new();
        assert_eq!(server.send(input), output);
    }
}
