/// Component Ethernet interface inventory (per-NIC IP/MAC records).
pub mod ethernet_interfaces;
/// Hardware inventory by location (populated components in the system).
pub mod hardware;
/// Hardware inventory by FRU (field-replaceable-unit identity).
pub mod hardware_by_fru;
/// Redfish endpoint inventory (BMC connection details).
pub mod redfish_endpoint;
/// Shared inventory wire types (hardware, node summary, etc.).
pub mod types;
