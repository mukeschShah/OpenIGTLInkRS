use message_base::*;
use bytebuffer::*;
#[allow(dead_code)]

/// Status codes -- see igtl_status.h

const STATUS_INVALID: u16 = 0;
const STATUS_OK: u16 = 1;
const STATUS_UNKNOWN_ERROR: u16 = 2;
const STATUS_PANICK_MODE: u16 = 3; /* emergency */
const STATUS_NOT_FOUND: u16 = 4; /* file, configuration, device etc */
const STATUS_ACCESS_DENIED: u16 = 5;
const STATUS_BUSY: u16 = 6;
const STATUS_TIME_OUT: u16 = 7; /* Time out / Connection lost */
const STATUS_OVERFLOW: u16 = 8; /* Overflow / Can't be reached */
const STATUS_CHECKSUM_ERROR: u16 = 9; /* Checksum error */
const STATUS_CONFIG_ERROR: u16 = 10; /* Configuration error */
const STATUS_RESOURCE_ERROR: u16 = 11; /* Not enough resource (memory, storage etc) */
const STATUS_UNKNOWN_INSTRUCTION: u16 = 12; /* Illegal/Unknown instruction */
const STATUS_NOT_READY: u16 = 13; /* Device not ready (starting up)*/
const STATUS_MANUAL_MODE: u16 = 14; /* Manual mode (device does not accept commands) */
const STATUS_DISABLED: u16 = 15; /* Device disabled */
const STATUS_NOT_PRESENT: u16 = 16; /* Device not present */
const STATUS_UNKNOWN_VERSION: u16 = 17; /* Device version not known */
const STATUS_HARDWARE_FAILURE: u16 = 18; /* Hardware failure */
const STATUS_SHUT_DOWN: u16 = 19; /* Exiting / shut down in progress */
const STATUS_NUM_TYPES: u16 = 20;


pub struct StatusMessage {
    /// The error code.
    base: MessageBase,

    code: u16,

    /// The sub code.
    sub_code: i64,

    /// The error name.
    error_name: String,

    /// The status message string.
    status_message_string: String,
}

impl StatusMessage {
    pub fn new() -> StatusMessage {
        StatusMessage {
            // Header
            base: MessageBase {
                message_size: 58,
                header_version: 2,
                device_name: "STATUS".to_string(),
                message_type: "Message Type".to_string(),
                time_stamp_sec: 0,
                time_stamp_sec_fraction: 0,
                body_size: 0,
                is_header_unpacked: false,
                is_body_unpacked: false,
                is_body_packed: false,
            },
            // Body
            code: STATUS_OK,
            sub_code: 0,
            error_name: "Hello".to_string(),
            status_message_string: "Hello".to_string(),
        }
    }

    /// Sets the status code.
    pub fn set_code(&mut self, code: u16) {
        self.code = code;
    }
    /// Gets the status code.
    pub fn get_code(&self) -> u16 {
        self.code
    }
    /// Sets the sub code.
    pub fn set_sub_code(&mut self, subcode: i64) {
        self.sub_code = subcode;
    }

    /// Gets the sub code.
    pub fn get_sub_code(&self) -> i64 {
        self.sub_code
    }

    /// Sets the error name. The error name can be defined by a developer.
    pub fn set_error_name(&mut self, name: String) {
        self.error_name = name;
    }

    /// Gets the error name.
    pub fn get_error_name(&self) -> String {
        self.error_name.clone()
    }

    /// Sets the status string.
    pub fn set_status_string(&mut self, status_string: String) {
        self.status_message_string = status_string;
    }

    /// Gets the status string.
    pub fn get_status_string(&self) -> String {
        self.status_message_string.clone()
    }

    fn calculate_content_buffer_size(&self) -> u64 {
        unimplemented!();
    }


    fn calculate_body_size(&mut self) -> usize {
        let body_size: usize = (2 + 8 + 20) + self.status_message_string.len();
        self.base.body_size = body_size as u64;
        body_size
    }
    /// mutable because crc and body_size have to calculated and set
    pub fn to_bytebuffer(&mut self) -> ByteBuffer {

        // bodybuffer
        let mut bb: ByteBuffer = ByteBuffer::new();
        bb.clear();
        bb.write_u16(self.code);
        bb.write_i64(self.sub_code);

        // convert the string to 8 array;
        let mut array: [u8; 20] = [0; 20];
        let mut s = self.error_name.clone();
        s.truncate(20);
        for (i, c) in s.chars().enumerate() {
            array[i] = c as u8;
        }
        bb.write_bytes(&array);

        // convert the string to 8 array;

        //let mut s = self.strin.clone();
        for (i, c) in self.status_message_string.chars().enumerate() {
            array[i] = c as u8;
            bb.write_u8(c as u8);
        }

        // now the header part

        self.calculate_body_size(); // and write int into buffer;
        let mut hb: ByteBuffer = self.base.to_bytebuffer();

        hb.write_bytes(&bb.to_bytes());

        return hb;

    }

    // virtual int  PackContent();
    // virtual int  UnpackContent();
}


#[test]
fn status_message() {

    let mut test_object = StatusMessage::new();
    test_object.base.set_device_name("newStatus".to_string());
    // test setter and getter
    test_object.set_code(5);
    assert_eq!(test_object.get_code(), 5);

    test_object.set_sub_code(-10);
    assert_eq!(test_object.get_sub_code(), -10);

    test_object.set_error_name("my Error".to_string());
    assert_eq!(test_object.get_error_name(), "my Error");

    test_object.set_status_string("Die ist mein Status".to_string());
    assert_eq!(test_object.get_status_string(), "Die ist mein Status");

    assert_eq!(test_object.calculate_body_size(), 19 + 2 + (64 / 8) + 20);

    // test to_bytebuffer
    assert_eq!(test_object.to_bytebuffer().len(),
               test_object.calculate_body_size() + 58);
}