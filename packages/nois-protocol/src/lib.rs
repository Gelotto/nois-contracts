mod checks;
mod data;
mod ibc_msg;

use cosmwasm_std::IbcOrder;

pub use checks::{check_order, check_version, ChannelError};
pub use data::Data;
pub use ibc_msg::{
    Beacon, DeliverBeaconPacket, DeliverBeaconPacketAck, RequestBeaconPacket,
    RequestBeaconPacketAck, StdAck,
};

pub const IBC_APP_VERSION: &str = "nois-v1";
pub const APP_ORDER: IbcOrder = IbcOrder::Unordered;
// we use this for tests to ensure it is rejected
pub const BAD_APP_ORDER: IbcOrder = IbcOrder::Ordered;