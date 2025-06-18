pub struct NetServer {}

pub enum SendOptions {
    Unreliable,
    ReliableUnordered,
    Sequenced,
    ReliableOrdered,
}
