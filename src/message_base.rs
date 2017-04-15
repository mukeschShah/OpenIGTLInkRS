
/// byteBuffer not work..because String handle have meta information
// extern crate bincode;
use bytebuffer::*;
#[allow(dead_code)]



pub struct MessageBase {
    pub message_size: i16,
    /// An unsigned short for the message format version
    pub header_version: u16,

    /// A character string for the device name (message name).
    pub device_name: String,
    /// a character string for message type name
    pub message_type: String,

    /// A time stamp (second) for message creation. It consists of fields for seconds
    /// (m_TimeStampSec)and fractions of a second (m_TimeStampSecFraction).
    pub time_stamp_sec: u32,

    /// A time stamp (second) for message creation. It consists of fields for seconds
    /// (m_TimeStampSec)and fractions of a second (m_TimeStampSecFraction).
    pub time_stamp_sec_fraction: u32,

    // not in original c++ implementation...but needed
    pub body_size: u64,

    /// Unpacking (deserialization) status for the header
    pub is_header_unpacked: bool,

    /// Unpacking (deserialization) status for the body
    pub is_body_unpacked: bool,

    /// Packing (serialization) status for the body
    pub is_body_packed: bool,
}


impl MessageBase {
    pub fn new() -> MessageBase {
        MessageBase {
            message_size: 58,
            header_version: 2,
            device_name: String::with_capacity(12),
            message_type: "Message Type".to_string(),
            time_stamp_sec: 0,
            time_stamp_sec_fraction: 0,
            body_size: 0,
            is_header_unpacked: false,
            is_body_unpacked: false,
            is_body_packed: false,
        }
    }

    pub fn set_device_name(&mut self, name: String) {
        self.device_name = name;
    }
    /// Gets the device name.
    pub fn get_device_name(&self) -> String {
        // We know these bytes are valid, so just use `unwrap()`.
        let mut s = self.device_name.clone();
        s.truncate(12);
        s
    }


    pub fn get_body_size(&self) -> u64 {
        self.body_size
    }


    pub fn get_crc64(&self) -> u64 {
        0
    }

    pub fn set_header_version(&mut self, version: u16) {
        self.header_version = version;

    }

    pub fn get_header_version(&self) -> u16 {
        self.header_version
    }

    pub fn to_bytebuffer(&self) -> ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.clear();
        buffer.write_u16(self.header_version);

        // convert the string to 8 array;
        let mut array: [u8; 12] = [0; 12];
        let mut s = self.device_name.clone();
        s.truncate(12);
        for (i, c) in s.chars().enumerate() {
            array[i] = c as u8;
        }
        buffer.write_bytes(&array);

        // convert the string to 8 array;
        let mut array: [u8; 20] = [0; 20];
        let mut s = self.message_type.clone();
        s.truncate(20);
        for (i, c) in s.chars().enumerate() {
            array[i] = c as u8;
        }
        buffer.write_bytes(&array);

        buffer.write_u32(self.time_stamp_sec);
        buffer.write_u32(self.time_stamp_sec_fraction);
        buffer.write_u64(self.get_body_size());
        buffer.write_u64(self.get_crc64());

        return buffer;
    }
}


#[test]
fn message_base_test() {

    let mut test_object = MessageBase::new();
    test_object.set_device_name("hello".to_string());
    assert_eq!("hello".to_string(), test_object.get_device_name());

    test_object.set_header_version(2);
    assert_eq!(2, test_object.get_header_version());

    // println!("ByteBuffer string: {}",
    //         test_object.to_bytebuffer().to_string());
    // is buffer.len() = 58;
    assert_eq!(58, test_object.to_bytebuffer().len());

}
