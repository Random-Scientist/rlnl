use byteserde_derive::{ByteDeserializeSlice, ByteSerializeHeap};

use crate::types::{CompressedVec3, PosQuatPair};
#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct MachineMotion {
    pub last_sent_seconds_a: f32,
    pub last_sent_seconds_b: f64,
    pub timestamp: f32,
    pub player_id: u8,
    pub target_point: CompressedVec3<768>,
    pub rb_state: RigidBodyState,
}
#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct RigidBodyState {
    pub rb_pos_rot: PosQuatPair,
    pub angular_velocity: CompressedVec3<768>,
    pub center_of_mass: CompressedVec3<32>,
}
