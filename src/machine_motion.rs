use byteserde_derive::{ByteDeserializeSlice, ByteSerializeHeap};

use crate::types::{CompressedVec3, PosQuatPair};
#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct MachineMotion {
    last_sent_seconds_a: f32,
    last_sent_seconds_b: f64,
    timestamp: f32,
    player_id: u8,
    target_point: CompressedVec3<768>,
    rb_state: RigidBodyState,
}
#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct RigidBodyState {
    rb_pos_rot: PosQuatPair,
    angular_velocity: CompressedVec3<768>,
    center_of_mass: CompressedVec3<32>,
}
