use constants::{ICON_CLIENT_STATE_TYPE_URL, ICON_CONSENSUS_STATE_TYPE_URL, ICON_SIGNED_HEADER_TYPE_URL};
use icon::icon::{lightclient::v1::{ClientState, ConsensusState}, types::v1::SignedHeader};
use traits::AnyTypes;

pub mod btp_header;
pub mod constants;
pub mod icon;
pub mod rlp;
pub mod types;
pub mod utils;
pub mod traits;


impl AnyTypes for ClientState {
    fn get_type_url() -> String {
        ICON_CLIENT_STATE_TYPE_URL.to_string()
    }
}

impl AnyTypes for ConsensusState {
    fn get_type_url() -> String {
        ICON_CONSENSUS_STATE_TYPE_URL.to_string()
    }
}

impl AnyTypes for SignedHeader {
    fn get_type_url() -> String {
        ICON_SIGNED_HEADER_TYPE_URL.to_string()
    }
}
