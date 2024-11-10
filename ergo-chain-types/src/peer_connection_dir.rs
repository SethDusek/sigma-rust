/// Direction of the connection to a peer
#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub enum ConnectionDirection {
    /// A peer is connecting to us
    Incoming,
    /// We are connecting to a peer
    Outgoing,
}
