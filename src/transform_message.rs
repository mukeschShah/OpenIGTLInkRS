extern crate time;

use message_base::*;
use bytebuffer::*;


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
    /// Sets normal vectors (or a rotation matrix) in the RAS coordinate system.
    pub fn set_normals(&mut self, o: [f32; 9]) {
        self.matrix[0] = o[0];
        self.matrix[1] = o[1];
        self.matrix[2] = o[2];

        self.matrix[4] = o[3];
        self.matrix[5] = o[4];
        self.matrix[6] = o[5];

        self.matrix[8] = o[6];
        self.matrix[9] = o[7];
        self.matrix[10] = o[8];
    }

    /// Gets normal vectors (or a rotation matrix) in the RAS coordinate system.
    //void GetNormals(float o[3][3]);
    pub fn get_normals(&self) -> [f32; 9] {
        let mut o: [f32; 9] = [0f32; 9];
        o[0] = self.matrix[0];
        o[1] = self.matrix[1];
        o[2] = self.matrix[2];

        o[3] = self.matrix[4];
        o[4] = self.matrix[5];
        o[5] = self.matrix[6];

        o[6] = self.matrix[8];
        o[7] = self.matrix[9];
        o[8] = self.matrix[10];

        // return
        o
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

    test_array = [1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 2.0, 3.0, 3.0, 3.0, 3.0, 4.0, 4.0, 4.0, 4.0];
    let mut test_normals: [f32; 9] = [1.2, 1.3, 1.4, 1.0, 2.0, 2.0, 2.0, 2.0, 3.0];
    t_o.set_normals(test_normals);
    assert_eq!(t_o.get_normals(), test_normals);

}
