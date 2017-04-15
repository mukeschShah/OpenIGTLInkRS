
#![cfg(test)]

extern crate bytebuffer;
// mod use message_base::MessageBase; // <-- Use one::one::bar
// #[path = "message_base.rs"]
pub mod message_base;
pub mod status_message;

// fn main() {
//     let mut test_object = message_base::MessageBase::new();
//     test_object.set_device_name("helloasdfasfasfdasdfasdfasdfasfd".to_string());
// }


#[cfg(test)]
mod tests {
    //use super::message_base::MessageBase;
    #[test]
    fn it_works() {
        //let mut test_object = MessageBase::new();
    }
}
