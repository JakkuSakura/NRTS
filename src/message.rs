use crate::geometry::PhyPos;

pub enum MessageOrigin {
    Unit(),

}
pub enum MessageType {

}
pub struct Message {
    pub origin_pos: PhyPos,
    pub origin: MessageOrigin,
    pub msg_type: MessageType
}