use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ComponentType {
    CDU,
    CabinetCDU,
    CabinetPDU,
    CabinetPDUOutlet,
    CabinetPDUPowerConnector,
    CabinetPDUController,
    r#Cabinet,
    Chassis,
    ChassisBMC,
    CMMRectifier,
    CMMFpga,
    CEC,
    ComputeModule,
    RouterModule,
    NodeBMC,
    NodeEnclosure,
    NodeEnclosurePowerSupply,
    HSNBoard,
    Node,
    Processor,
    Drive,
    StorageGroup,
    NodeNIC,
    Memory,
    NodeAccel,
    NodeAccelRiser,
    NodeFpga,
    HSNAsic,
    RouterFpga,
    RouterBMC,
    HSNLink,
    HSNConnector,
    INVALID,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInventoryByFRU {
    #[serde(rename(serialize = "FRUID"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fru_id: Option<String>,
    #[serde(rename(serialize = "Type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename(serialize = "FRUSubType"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fru_sub_type: Option<String>,
    #[serde(rename(serialize = "HWInventoryByFRUType"))]
    pub hw_inventory_by_fru_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishChassisLocationInfo {
    #[serde(rename(serialize = "Id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename(serialize = "Name"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename(serialize = "Description"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename(serialize = "Hostname"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocChassis {
    #[serde(rename(serialize = "ID"))]
    pub id: String,
    #[serde(rename(serialize = "Type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename(serialize = "Ordinal"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename(serialize = "Status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename(serialize = "HWInventoryByLocationType"))]
    pub hw_inventory_by_location_type: String,
    #[serde(rename(serialize = "PopulatedFRU"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename(serialize = "ChassisLocatinInfo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chassis_location_info: Option<RedfishChassisLocationInfo>,
    #[serde(rename(serialize = "ComputeModules"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_modules: Option<HWInvByLocComputeModule>,
    #[serde(rename(serialize = "RouterModules"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_modules: Option<HWInvByLocRouterModule>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocNodeEnclosure {
    #[serde(rename(serialize = "ID"))]
    pub id: String,
    #[serde(rename(serialize = "Type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename(serialize = "Ordinal"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename(serialize = "Status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename(serialize = "HWInventoryByLocationType"))]
    pub hw_inventory_by_location_type: String,
    #[serde(rename(serialize = "PopulatedFRU"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename(serialize = "NodeEnclosureLocationInfo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_enclosure_location_info: Option<RedfishChassisLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocComputeModule {
    #[serde(rename(serialize = "ID"))]
    pub id: String,
    #[serde(rename(serialize = "Type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename(serialize = "Ordinal"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename(serialize = "Status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename(serialize = "HWInventoryByLocationType"))]
    pub hw_inventory_by_location_type: String,
    #[serde(rename(serialize = "PopulatedFRU"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename(serialize = "ComputeModuleLocationInfo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_module_location_info: Option<RedfishChassisLocationInfo>,
    #[serde(rename(serialize = "NodeEnclosures"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_enclosures: Option<HWInvByLocNodeEnclosure>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocHSNBoard {
    #[serde(rename(serialize = "ID"))]
    pub id: String,
    #[serde(rename(serialize = "Type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename(serialize = "Ordinal"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename(serialize = "Status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename(serialize = "HWInventoryByLocationType"))]
    pub hw_inventory_by_location_type: String,
    #[serde(rename(serialize = "PopulatedFRU"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename(serialize = "HSNBoardLocationInfo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsn_board_location_info: Option<RedfishChassisLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocRouterModule {
    #[serde(rename(serialize = "ID"))]
    pub id: String,
    #[serde(rename(serialize = "Type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename(serialize = "Ordinal"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename(serialize = "Status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename(serialize = "HWInventoryByLocationType"))]
    pub hw_inventory_by_location_type: String,
    #[serde(rename(serialize = "PopulatedFRU"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename(serialize = "RouterModuleLocationInfo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_module_location_info: Option<RedfishChassisLocationInfo>,
    pub hsn_boards: Option<HWInvByLocHSNBoard>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocCabinet {
    #[serde(rename(serialize = "ID"))]
    pub id: String,
    #[serde(rename(serialize = "Type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename(serialize = "Ordinal"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename(serialize = "Status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename(serialize = "HWInventoryByLocationType"))]
    pub hw_inventory_by_location_type: String,
    #[serde(rename(serialize = "PopulatedFRU"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename(serialize = "CabinetLocationInfo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cabinet_location_info: Option<RedfishChassisLocationInfo>,
    #[serde(rename(serialize = "Chassis"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chassis: Option<HWInvByLocChassis>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocMgmtSwitch {
    #[serde(rename(serialize = "ID"))]
    pub id: String,
    #[serde(rename(serialize = "Type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename(serialize = "Ordinal"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename(serialize = "Status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename(serialize = "HWInventoryByLocationType"))]
    pub hw_inventory_by_location_type: String,
    #[serde(rename(serialize = "PopulatedFRU"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename(serialize = "MgmtSwitchLocationInfo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mgmt_switch_location_info: Option<RedfishChassisLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocMgmtHLSwitch {
    #[serde(rename(serialize = "ID"))]
    pub id: String,
    #[serde(rename(serialize = "Type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename(serialize = "Ordinal"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename(serialize = "Status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename(serialize = "HWInventoryByLocationType"))]
    pub hw_inventory_by_location_type: String,
    #[serde(rename(serialize = "PopulatedFRU"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename(serialize = "MgmtHLSwitchLocationInfo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mgmt_hl_switch_location_info: Option<RedfishChassisLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocCDUMgmtSwitch {
    #[serde(rename(serialize = "ID"))]
    pub id: String,
    #[serde(rename(serialize = "Type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename(serialize = "Ordinal"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename(serialize = "Status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename(serialize = "HWInventoryByLocationType"))]
    pub hw_inventory_by_location_type: String,
    #[serde(rename(serialize = "PopulatedFRU"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename(serialize = "CDUMgmtSwitchLocationInfo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdu_mgmt_switch_location_info: Option<RedfishChassisLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessorSummary {
    #[serde(rename(serialize = "Count"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<u32>,
    #[serde(rename(serialize = "Model"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemorySummary {
    #[serde(rename(serialize = "TotalSystemMemoryGiB"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    total_system_memory_gib: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishSystemLocationInfo {
    #[serde(rename(serialize = "Id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename(serialize = "Name"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename(serialize = "Description"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename(serialize = "Hostname"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename(serialize = "ProcessorSummary"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_summary: Option<ProcessorSummary>,
    #[serde(rename(serialize = "MemorySummary"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_summary: Option<MemorySummary>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishProcessorLocationInfo {
    #[serde(rename(serialize = "Id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename(serialize = "Name"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename(serialize = "Description"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename(serialize = "Socket"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub socket: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocProcessor {
    #[serde(rename(serialize = "ID"))]
    pub id: String,
    #[serde(rename(serialize = "Type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename(serialize = "Ordinal"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename(serialize = "Status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename(serialize = "HWInventoryByLocationType"))]
    pub hw_inventory_by_location_type: String,
    #[serde(rename(serialize = "PopulatedFRU"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename(serialize = "ProcessorLocationInfo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    processor_location_info: Option<RedfishProcessorLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocNodeAccel {
    #[serde(rename(serialize = "ID"))]
    pub id: String,
    #[serde(rename(serialize = "Type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename(serialize = "Ordinal"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename(serialize = "Status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename(serialize = "HWInventoryByLocationType"))]
    pub hw_inventory_by_location_type: String,
    #[serde(rename(serialize = "PopulatedFRU"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename(serialize = "NodeAccelLocationInfo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    node_accel_location_info: Option<RedfishProcessorLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishDriveLocationInfo {
    #[serde(rename(serialize = "Id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename(serialize = "Name"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename(serialize = "Description"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocDrive {
    #[serde(rename(serialize = "ID"))]
    pub id: String,
    #[serde(rename(serialize = "Type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename(serialize = "Ordinal"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename(serialize = "Status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename(serialize = "HWInventoryByLocationType"))]
    pub hw_inventory_by_location_type: String,
    #[serde(rename(serialize = "PopulatedFRU"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename(serialize = "DriveLocationInfo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    drive_location_info: Option<RedfishDriveLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemoryLocation {
    #[serde(rename(serialize = "Socket"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    socket: Option<u32>,
    #[serde(rename(serialize = "MemoryController"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_controller: Option<u32>,
    #[serde(rename(serialize = "Channel"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    channel: Option<u32>,
    #[serde(rename(serialize = "Slot"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    slot: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishMemoryLocationInfo {
    #[serde(rename(serialize = "Id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename(serialize = "Name"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename(serialize = "Description"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename(serialize = "MemoryLocation"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_location: Option<MemoryLocation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocMemory {
    #[serde(rename(serialize = "ID"))]
    pub id: String,
    #[serde(rename(serialize = "Type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename(serialize = "Ordinal"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename(serialize = "Status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename(serialize = "HWInventoryByLocationType"))]
    pub hw_inventory_by_location_type: String,
    #[serde(rename(serialize = "PopulatedFRU"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename(serialize = "DriveLocationInfo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    drive_location_info: Option<RedfishMemoryLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishNodeAccelRiserLocationInfo {
    #[serde(rename(serialize = "Name"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename(serialize = "Description"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocNodeAccelRiser {
    #[serde(rename(serialize = "ID"))]
    pub id: String,
    #[serde(rename(serialize = "Type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename(serialize = "Ordinal"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename(serialize = "Status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename(serialize = "HWInventoryByLocationType"))]
    pub hw_inventory_by_location_type: String,
    #[serde(rename(serialize = "PopulatedFRU"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename(serialize = "NodeAccelRiserLocationInfo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_accel_riser_location_info: Option<RedfishNodeAccelRiserLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HSNNICLocationInfo {
    #[serde(rename(serialize = "Id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename(serialize = "Name"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename(serialize = "Description"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocHSNNIC {
    #[serde(rename(serialize = "ID"))]
    pub id: String,
    #[serde(rename(serialize = "Type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename(serialize = "Ordinal"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename(serialize = "Status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename(serialize = "HWInventoryByLocationType"))]
    pub hw_inventory_by_location_type: String,
    #[serde(rename(serialize = "PopulatedFRU"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename(serialize = "HSNNICLocationInfo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsn_noc_location_info: Option<HSNNICLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocNode {
    #[serde(rename(serialize = "ID"))]
    pub id: String,
    #[serde(rename(serialize = "Type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename(serialize = "Ordinal"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename(serialize = "Status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename(serialize = "HWInventoryByLocationType"))]
    pub hw_inventory_by_location_type: String,
    #[serde(rename(serialize = "PopulatedFRU"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename(serialize = "NodeLocationInfo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_location_info: Option<RedfishSystemLocationInfo>,
    #[serde(rename(serialize = "Processors"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processors: Option<HWInvByLocProcessor>,
    #[serde(rename(serialize = "NodeAccels"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_accels: Option<HWInvByLocNodeAccel>,
    #[serde(rename(serialize = "Dives"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drives: Option<HWInvByLocDrive>,
    #[serde(rename(serialize = "Memory"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<HWInvByLocMemory>,
    #[serde(rename(serialize = "NodeAccelRisers"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_accel_risers: Option<HWInvByLocNodeAccelRiser>,
    #[serde(rename(serialize = "NodeHsnNICs"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_hsn_nics: Option<HWInvByLocHSNNIC>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishPDULocationInfo {
    #[serde(rename(serialize = "Id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename(serialize = "Name"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename(serialize = "Description"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename(serialize = "UUID"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishOutletLocationInfo {
    #[serde(rename(serialize = "Id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename(serialize = "Name"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename(serialize = "Description"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocOutlet {
    #[serde(rename(serialize = "ID"))]
    pub id: String,
    #[serde(rename(serialize = "Type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename(serialize = "Ordinal"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename(serialize = "Status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename(serialize = "HWInventoryByLocationType"))]
    pub hw_inventory_by_location_type: String,
    #[serde(rename(serialize = "PopulatedFRU"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename(serialize = "OutletLocationInfo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outlet_location_info: Option<RedfishOutletLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocPDU {
    #[serde(rename(serialize = "ID"))]
    pub id: String,
    #[serde(rename(serialize = "Type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename(serialize = "Ordinal"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename(serialize = "Status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename(serialize = "HWInventoryByLocationType"))]
    pub hw_inventory_by_location_type: String,
    #[serde(rename(serialize = "PopulatedFRU"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename(serialize = "PDULocationInfo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdu_location_info: Option<RedfishPDULocationInfo>,
    #[serde(rename(serialize = "CabinetPDUPowerConnectors"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cabinet_pdu_power_connectors: Option<HWInvByLocOutlet>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishCMMRectifierLocationInfo {
    #[serde(rename(serialize = "Name"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename(serialize = "FirmwareVersion"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocCMMRectifier {
    #[serde(rename(serialize = "ID"))]
    pub id: String,
    #[serde(rename(serialize = "Type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename(serialize = "Ordinal"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename(serialize = "Status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename(serialize = "HWInventoryByLocationType"))]
    pub hw_inventory_by_location_type: String,
    #[serde(rename(serialize = "PopulatedFRU"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename(serialize = "CMMRectifierLocationInfo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    cmm_rectifier_location_info: Option<RedfishCMMRectifierLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishNodeEnclosurePowerSupplyLocationInfo {
    #[serde(rename(serialize = "Name"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename(serialize = "FirmwareVersion"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocNodePowerSupply {
    #[serde(rename(serialize = "ID"))]
    pub id: String,
    #[serde(rename(serialize = "Type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename(serialize = "Ordinal"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename(serialize = "Status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename(serialize = "HWInventoryByLocationType"))]
    pub hw_inventory_by_location_type: String,
    #[serde(rename(serialize = "PopulatedFRU"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename(serialize = "NodeEnclosurePowerSupplyLocationInfo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_enclosure_power_supply_location_info:
        Option<RedfishNodeEnclosurePowerSupplyLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishManagerLocationInfo {
    #[serde(rename(serialize = "Id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename(serialize = "Name"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename(serialize = "Description"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename(serialize = "DateTime"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time: Option<String>,
    #[serde(rename(serialize = "DateTimeLocalOffset"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time_local_offset: Option<String>,
    #[serde(rename(serialize = "FirmwareVersion"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocNodeBMC {
    #[serde(rename(serialize = "ID"))]
    pub id: String,
    #[serde(rename(serialize = "Type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename(serialize = "Ordinal"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename(serialize = "Status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename(serialize = "HWInventoryByLocationType"))]
    pub hw_inventory_by_location_type: String,
    #[serde(rename(serialize = "PopulatedFRU"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename(serialize = "NodeBMCLocationInfo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_bmc_location_info: Option<RedfishManagerLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocRouterBMC {
    #[serde(rename(serialize = "ID"))]
    pub id: String,
    #[serde(rename(serialize = "Type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename(serialize = "Ordinal"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename(serialize = "Status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename(serialize = "HWInventoryByLocationType"))]
    pub hw_inventory_by_location_type: String,
    #[serde(rename(serialize = "PopulatedFRU"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename(serialize = "RouterBMCLocationInfo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_bmc_location_info: Option<RedfishManagerLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInventory {
    #[serde(rename(serialize = "XName"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xname: Option<String>,
    #[serde(rename(serialize = "Format"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename(serialize = "Cabinets"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cabinets: Option<HWInvByLocCabinet>,
    #[serde(rename(serialize = "Chassis"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chassis: Option<HWInvByLocChassis>,
    #[serde(rename(serialize = "ComputeModules"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_modules: Option<HWInvByLocComputeModule>,
    #[serde(rename(serialize = "RouterModules"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_modules: Option<HWInvByLocRouterModule>,
    #[serde(rename(serialize = "NodeEnclosures"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_enclosures: Option<HWInvByLocNodeEnclosure>,
    #[serde(rename(serialize = "HSNBoards"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsn_boards: Option<HWInvByLocHSNBoard>,
    #[serde(rename(serialize = "MgmtSwitches"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mgmt_switches: Option<HWInvByLocMgmtSwitch>,
    #[serde(rename(serialize = "MgmtHLSwitches"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mgmt_hl_switches: Option<HWInvByLocMgmtHLSwitch>,
    #[serde(rename(serialize = "CDUMgmtSwitches"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdu_mgmt_switches: Option<HWInvByLocCDUMgmtSwitch>,
    #[serde(rename(serialize = "Nodes"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<HWInvByLocNode>,
    #[serde(rename(serialize = "Processors"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processors: Option<HWInvByLocProcessor>,
    #[serde(rename(serialize = "NodeAccels"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_accels: Option<HWInvByLocNodeAccel>,
    #[serde(rename(serialize = "Drives"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drives: Option<HWInvByLocDrive>,
    #[serde(rename(serialize = "Memory"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<HWInvByLocMemory>,
    #[serde(rename(serialize = "CabinetPDUs"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cabinet_pdus: Option<HWInvByLocPDU>,
    #[serde(rename(serialize = "CabinetPDUPowerConnectors"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cabinet_pdu_power_connectors: Option<HWInvByLocOutlet>,
    #[serde(rename(serialize = "CMMRectifiers"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmm_rectifiers: Option<HWInvByLocCMMRectifier>,
    #[serde(rename(serialize = "NodeAccelRisers"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_accel_risers: Option<HWInvByLocNodeAccelRiser>,
    #[serde(rename(serialize = "NodeHsnNICs"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_hsn_nics: Option<HWInvByLocHSNNIC>,
    #[serde(rename(serialize = "NodeEnclosurePowerSupplies"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_enclosure_power_supplies: Option<HWInvByLocNodePowerSupply>,
    #[serde(rename(serialize = "NodeBMC"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_bmc: Option<HWInvByLocNodeBMC>,
    #[serde(rename(serialize = "RouterBMC"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_bmc: Option<HWInvByLocRouterBMC>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInventoryByLocation {
    #[serde(rename(serialize = "ID"))]
    pub id: String,
    #[serde(rename(serialize = "Type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename(serialize = "Ordinal"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename(serialize = "Status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename(serialize = "HWInventoryByLocationType"))]
    pub hw_inventory_by_location_type: String,
    #[serde(rename(serialize = "PopulatedFRU"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
}
