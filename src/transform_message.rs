extern crate time;

use message_base::*;
use bytebuffer::*;
use crc::{crc64, Hasher64};

#[allow(dead_code, unused_variables)]


pub struct TransformMessage {
    header: MessageBase,
    /// The transformation matri
    matrix: [f32; 16],
}

impl TransformMessage {
    pub fn new() -> TransformMessage {
        TransformMessage {
            // Header
            header: MessageBase {
                message_size: 58,
                header_version: 2, // have to be 1 to test for ReceiverServer
                device_name: "Device".to_string(),
                message_type: "TRANSFORM".to_string(),
                time_stamp_sec: 0,
                time_stamp_sec_fraction: 0,
                body_size: 0,
                crc64: 0,
                is_header_unpacked: false,
                is_body_unpacked: false,
                is_body_packed: false,
            },
            // Body
            matrix: [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
                     1.0],
        }
    }

    /// Sets rotation matrix using igtl::Matrix4x4.
    pub fn set_matrix(&mut self, arg: [f32; 16]) {
        self.matrix = arg;
    }

    /// Sets rotation matrix using igtl::Matrix4x4.
    pub fn get_matrix(&self) -> [f32; 16] {
        self.matrix
    }

    /// Sets a position (or a translation vector) in the RAS coordinate system.
    pub fn set_postion(&mut self, pos: [f32; 3]) {
        self.matrix[12] = pos[0];
        self.matrix[13] = pos[1];
        self.matrix[14] = pos[2];
    }
    /// Gets a position (or a translation vector) in the RAS coordinate system.

    pub fn get_postion(&self) -> [f32; 3] {
        let mut pos: [f32; 3] = [0.0, 0.0, 0.0];
        pos[0] = self.matrix[12];
        pos[1] = self.matrix[13];
        pos[2] = self.matrix[14];
        pos
    }
}


#[test]
fn status_message() {
    let mut t_o = TransformMessage::new();
    let mut test_array: [f32; 16] = [1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 2.0, 3.0, 3.0, 3.0, 3.0,
                                     4.0, 4.0, 4.0, 4.0];
    t_o.set_matrix(test_array);
    assert_eq!(t_o.get_matrix(), test_array);
    t_o.set_postion([10.0, 11.0, 12.0]);
    test_array = [1f32, 1f32, 1f32, 1f32, 2f32, 2f32, 2f32, 2f32, 3f32, 3f32, 3f32, 3f32, 10f32,
                  11f32, 12f32, 4f32];
    assert_eq!(t_o.get_matrix(), test_array);
    let test_pos: [f32; 3] = [10.0, 11.0, 12.0];
    assert_eq!(t_o.get_postion(), test_pos);

}
