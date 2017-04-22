
//! This is documentation for the `foo` crate.
//!
//! The foo crate is meant to be used for bar.

extern crate bytebuffer;
// to get the timestamp;
extern crate time;

// mod use message_base::MessageBase; // <-- Use one::one::bar
// #[path = "message_base.rs"]
pub mod message_base;
pub mod status_message;
pub mod transform_message;
pub mod util;
// fn main() {
//     let mut test_object = message_base::MessageBase::new();
//     test_object.set_device_name("helloasdfasfasfdasdfasdfasdfasfd".to_string());
// }


#[cfg(test)]
mod tests {
    // https://codereview.stackexchange.com/questions/110073/simple-tcp-client-in-rust
    use std::io::prelude::*;
    use std::net::TcpStream;
    use message_base::OpenIGTLinkMessage;
    use super::status_message::*;
    #[test]
    fn generating_status_message() {
        let mut t_o = StatusMessage::new();
        t_o.set_error_name("OK!".to_string());
        t_o.set_status_string("This is a test to send status message. MJS".to_string());
        t_o.set_sub_code(128);

    }
    #[test]
    fn sending_to_receiver_server() {
        const HOST: &'static str = "127.0.0.1:18944";
        let mut t_o = StatusMessage::new();
        t_o.set_error_name("OK!".to_string());
        t_o.set_status_string("This is a test to send status message. MJS".to_string());
        t_o.set_sub_code(128);


        let mut stream = TcpStream::connect(HOST).unwrap();

        for i in 1..10 {
            let response = stream.write(&t_o.to_bytebuffer().to_bytes()).unwrap();
        }
        let response = stream.write(&t_o.to_bytebuffer().to_bytes()).unwrap();
        //println!(" länge: {:?}", t_o.to_bytebuffer().to_bytes().len());
        print!("response  {:?}", response);

    }
}
