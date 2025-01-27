//! Module defining the `IndoAddressPortPair` struct, useful to format the output report file and
//! to keep track of statistics about the sniffed traffic.

use std::fmt;

use crate::{AppProtocol, TransProtocol};
use crate::enums::traffic_type::TrafficType;
use crate::utility::get_formatted_strings::get_formatted_bytes_string;

/// Struct useful to format the output report file and to keep track of statistics about the sniffed traffic.
///
/// Each InfoAddressPortPair struct is associated to a single address:port pair.
pub struct InfoAddressPortPair {
    /// Amount of bytes transmitted between the pair.
    pub transmitted_bytes: u128,
    /// Amount of packets transmitted between the pair.
    pub transmitted_packets: u128,
    /// First occurrence of information exchange featuring the associate address:port pair as a source or destination.
    pub initial_timestamp: String,
    /// Last occurrence of information exchange featuring the associate address:port pair as a source or destination.
    pub final_timestamp: String,
    ///  Transport layer protocol carried through the associate address:port pair (TCP or UPD).
    pub trans_protocol: TransProtocol,
    /// Set of application layer protocols carried through the associate address:port pair.
    pub app_protocol: AppProtocol,
    /// Check if source or destination is an IPv6 address longer than 25 bytes (used for Display
    pub very_long_address: bool,
    /// Flag to determine which of the address is that of the sniffed adapter or remote
    pub traffic_type: TrafficType,
}


impl InfoAddressPortPair {
    pub fn print_gui(&self) -> String {
        self.to_string().get(0..46).unwrap().to_string().replace('|', "")
    }
}


impl fmt::Display for InfoAddressPortPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes_string = get_formatted_bytes_string(self.transmitted_bytes);

        let app_string = match self.app_protocol {
            AppProtocol::Other => { "Other".to_string() }
            _ => { self.app_protocol.to_string() }
        };

        if self.very_long_address {
            write!(f, "   {}   |{:^9}|{:>10}  |{:>10}  | {} | {} |",
                   self.trans_protocol, app_string,
                   self.transmitted_packets, bytes_string,
                   self.initial_timestamp, self.final_timestamp)
        } else {
            write!(f, "   {}   |{:^9}|{:>10}  |{:>10}  | {} | {} |{}",
                   self.trans_protocol, app_string,
                   self.transmitted_packets, bytes_string,
                   self.initial_timestamp, self.final_timestamp, " ".repeat(40))
        }
    }
}