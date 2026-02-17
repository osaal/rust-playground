mod tests {
    use crate::{Message, Server, ServerObj};

    #[test]
    fn receives_msg() {
        let input = Message::HELO("my-domain.local".into());

        let t_servername = String::from("server.local");
        let output = Message::HELO(t_servername.clone());
        let server = ServerObj::new(t_servername.clone());

        assert_eq!(server.send(input), output);
    }
}
