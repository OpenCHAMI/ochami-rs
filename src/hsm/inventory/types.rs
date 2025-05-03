use manta_backend_dispatcher::types::{
    ArtifactSummary as FrontEndArtifactSummary, ArtifactType as FrontEndArtifactType,
    HSNNICFRUInfo as FrontEndHSNNICFRUInfo, HSNNICLocationInfo as FrontEndHSNNICLocationInfo,
    HWInvByFRUHSNNIC as FrontEndHWInvByFRUHSNNIC, HWInvByFRUMemory as FrontEndHWInvByFRUMemory,
    HWInvByFRUNode as FrontEndHWInvByFRUNode, HWInvByFRUNodeAccel as FrontEndHWInvByFRUNodeAccel,
    HWInvByFRUProcessor as FrontEndHWInvByFRUProcessor,
    HWInvByLocHSNNIC as FrontEndHWInvByLocHSNNIC, HWInvByLocMemory as FrontEndHWInvByLocMemory,
    HWInvByLocNode as FrontEndHWInvByLocNode, HWInvByLocNodeAccel as FrontEndHWInvByLocNodeAccel,
    HWInvByLocProcessor as FrontEndHWInvByLocProcessor, HWInventory as FrontEndHWInventory,
    HWInventoryByLocation as FrontEndHWInventoryByLocation,
    HWInventoryByLocationList as FrontEndHWInventoryByLocationList,
    MemoryLocation as FrontEndMemoryLocation, MemorySummary as FrontEndMemorySummary,
    NodeLocationInfo as FrontEndNodeLocationInfo, NodeSummary as FrontEndNodeSummary,
    ProcessorSummary as FrontEndProcessorSummary,
    RedfishMemoryFRUInfo as FrontEndRedfishMemoryFRUInfo,
    RedfishMemoryLocationInfo as FrontEndRedfishMemoryLocationInfo,
    RedfishProcessorFRUInfo as FrontEndRedfishProcessorFRUInfo,
    RedfishProcessorLocationInfo as FrontEndRedfishProcessorLocationInfo,
    RedfishSystemFRUInfo as FrontEndRedfishSystemFRUInfo,
    RedfishSystemLocationInfo as FrontEndRedfishSystemLocationInfo,
};

use serde::{Deserialize, Serialize};

use serde_json::Value;
use std::str::FromStr;
use std::string::ToString;
use strum_macros::{AsRefStr, Display, EnumIter, EnumString, IntoStaticStr};

///////////////////////////////////////////////////////////////////////////////
// MESA - These are nonr official structs created from 'curl' response payload
#[derive(
    Debug, EnumIter, EnumString, IntoStaticStr, AsRefStr, Display, Serialize, Deserialize, Clone,
)]
pub enum ArtifactType {
    Memory,
    Processor,
    NodeAccel,
    NodeHsnNic,
    Drive,
    CabinetPDU,
    CabinetPDUPowerConnector,
    CMMRectifier,
    NodeAccelRiser,
    NodeEnclosurePowerSupplie,
    NodeBMC,
    RouterBMC,
}

impl From<FrontEndArtifactType> for ArtifactType {
    fn from(value: FrontEndArtifactType) -> Self {
        match value {
            FrontEndArtifactType::Memory => ArtifactType::Memory,
            FrontEndArtifactType::Processor => ArtifactType::Processor,
            FrontEndArtifactType::NodeAccel => ArtifactType::NodeAccel,
            FrontEndArtifactType::NodeHsnNic => ArtifactType::NodeHsnNic,
            FrontEndArtifactType::Drive => ArtifactType::Drive,
            FrontEndArtifactType::CabinetPDU => ArtifactType::CabinetPDU,
            FrontEndArtifactType::CabinetPDUPowerConnector => {
                ArtifactType::CabinetPDUPowerConnector
            }
            FrontEndArtifactType::CMMRectifier => ArtifactType::CMMRectifier,
            FrontEndArtifactType::NodeAccelRiser => ArtifactType::NodeAccelRiser,
            FrontEndArtifactType::NodeEnclosurePowerSupplie => {
                ArtifactType::NodeEnclosurePowerSupplie
            }
            FrontEndArtifactType::NodeBMC => ArtifactType::NodeBMC,
            FrontEndArtifactType::RouterBMC => ArtifactType::RouterBMC,
        }
    }
}

impl Into<FrontEndArtifactType> for ArtifactType {
    fn into(self) -> FrontEndArtifactType {
        match self {
            ArtifactType::Memory => FrontEndArtifactType::Memory,
            ArtifactType::Processor => FrontEndArtifactType::Processor,
            ArtifactType::NodeAccel => FrontEndArtifactType::NodeAccel,
            ArtifactType::NodeHsnNic => FrontEndArtifactType::NodeHsnNic,
            ArtifactType::Drive => FrontEndArtifactType::Drive,
            ArtifactType::CabinetPDU => FrontEndArtifactType::CabinetPDU,
            ArtifactType::CabinetPDUPowerConnector => {
                FrontEndArtifactType::CabinetPDUPowerConnector
            }
            ArtifactType::CMMRectifier => FrontEndArtifactType::CMMRectifier,
            ArtifactType::NodeAccelRiser => FrontEndArtifactType::NodeAccelRiser,
            ArtifactType::NodeEnclosurePowerSupplie => {
                FrontEndArtifactType::NodeEnclosurePowerSupplie
            }
            ArtifactType::NodeBMC => FrontEndArtifactType::NodeBMC,
            ArtifactType::RouterBMC => FrontEndArtifactType::RouterBMC,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeSummary {
    pub xname: String,
    pub r#type: String,
    pub processors: Vec<ArtifactSummary>,
    pub memory: Vec<ArtifactSummary>,
    pub node_accels: Vec<ArtifactSummary>,
    pub node_hsn_nics: Vec<ArtifactSummary>,
}

/*
impl From<HWInventoryByLocation> for NodeSummary {
    fn from(value: HWInventoryByLocation) -> Self {
        NodeSummary {
            xname: value.id,
            r#type: value.r#type.unwrap_or_default(),
            processors: value
                .processors
                .unwrap_or_default()
                .into_iter()
                .map(ArtifactSummary::from)
                .collect(),
            // FIXME: implement FROM trait from HWInvByLlocProcessor to ArtifactSummary
            memory: value
                .memory
                .unwrap_or_default()
                .into_iter()
                .map(ArtifactSummary::from)
                .collect(),
            // FIXME: implement FROM trait from HWInvByLlocMemory to ArtifactSummary
            node_accels: value
                .node_accels
                .unwrap_or_default()
                .into_iter()
                .map(ArtifactSummary::from)
                .collect(),
            // FIXME: implement FROM trait from HWInvByLlocNodeAccel to ArtifactSummary
            node_hsn_nics: value
                .node_hsn_nics
                .unwrap_or_default()
                .into_iter()
                .map(ArtifactSummary::from)
                .collect(),
            // FIXME: implement FROM trait from HWInvByLlocHSNNIC to ArtifactSummary
        }
    }
}

impl Into<HWInventoryByLocation> for NodeSummary {
    fn into(self) -> HWInventoryByLocation {
        let redfish_system_fru_info = RedfishSystemFRUInfo {
            asset_tag: None,
            bios_version: None,
            model: None,
            manufacturer: None,
            part_number: None,
            serial_number: None,
            sku: None,
            system_type: None,
            uuid: None,
        };

        let hw_inv_by_fr_node = HWInvByFRUNode {
            fru_id: None,
            r#type: None,
            fru_sub_type: None,
            hw_inventory_by_fru_type: self.r#type.clone(),
            node_fru_info: redfish_system_fru_info,
        };

        let processor_summary = ProcessorSummary {
            count: self.processors.len().try_into().ok(),
            model: self.processors.first().unwrap().info.clone(),
        };

        let memory_capacity_vec: Vec<usize> = self
            .memory
            .iter()
            .map(|memory| {
                let binding = "".to_string();
                let mem_capacity = memory.info.as_ref().unwrap_or(&binding);
                /* .strip_suffix(" MiB")
                .unwrap_or_default(); */

                usize::from_str(mem_capacity.strip_suffix(" MiB").unwrap_or_default())
                    .unwrap_or_default()
            })
            .collect();

        let memory_summary = MemorySummary {
            total_system_memory_gib: Some(memory_capacity_vec.iter().sum::<usize>() as u32),
        };

        let node_location_info = NodeLocationInfo {
            id: self.xname.clone(),
            name: None,
            description: None,
            hostname: None,
            processor_summary: Some(processor_summary),
            memory_summary: Some(memory_summary),
        };

        HWInventoryByLocation {
            id: self.xname,
            r#type: Some(self.r#type.clone()),
            ordinal: None,
            status: None,
            node_location_info,
            hw_inventory_by_location_type: self.r#type,
            populated_fru: Some(hw_inv_by_fr_node),
            cabinets: None,
            chassis: None,
            compute_modules: None,
            router_modules: None,
            node_enclosures: None,
            hsn_boards: None,        // FIXME: implement!
            mgmt_switches: None,     // FIXME: implement!
            mgmt_hl_switches: None,  // FIXME: implement!
            cdu_mgmt_switches: None, // FIXME: implement!
            nodes: None,             // FIXME: implement!
            processors: Some(
                self.processors
                    .into_iter()
                    .map(|processor| processor.into())
                    .collect(),
            ), // FIXME: implement!
            node_accels: Some(
                self.node_accels
                    .into_iter()
                    .map(|node_accel| node_accel.into())
                    .collect(),
            ), // FIXME: implement!
            drives: None,            // FIXME: implement!
            memory: Some(
                self.memory
                    .into_iter()
                    .map(|memory| memory.into())
                    .collect(),
            ), // FIXME: implement!
            cabinet_pdus: None,      // FIXME: implement!
            cabinet_pdu_power_connectors: None, // FIXME: implement!
            cmm_rectifiers: None,    // FIXME: implement!
            node_accel_risers: None, // FIXME: implement!
            node_hsn_nics: Some(
                self.node_hsn_nics
                    .into_iter()
                    .map(|node_hsn_nic| node_hsn_nic.into())
                    .collect(),
            ), // FIXME: implement!
            node_enclosure_power_supplies: None, // FIXME: implement!
            node_bmc: None,
            router_bmc: None, // FIXME: implement!
        }
    }
} */

impl From<HWInvByLocNode> for NodeSummary {
    fn from(value: HWInvByLocNode) -> Self {
        NodeSummary {
            xname: value.id,
            r#type: value.r#type.unwrap_or_default(),
            processors: value
                .processors
                .unwrap_or_default()
                .into_iter()
                .map(ArtifactSummary::from)
                .collect(),
            // FIXME: implement FROM trait from HWInvByLlocProcessor to ArtifactSummary
            memory: value
                .memory
                .unwrap_or_default()
                .into_iter()
                .map(ArtifactSummary::from)
                .collect(),
            // FIXME: implement FROM trait from HWInvByLlocMemory to ArtifactSummary
            node_accels: value
                .node_accels
                .unwrap_or_default()
                .into_iter()
                .map(ArtifactSummary::from)
                .collect(),
            // FIXME: implement FROM trait from HWInvByLlocNodeAccel to ArtifactSummary
            node_hsn_nics: value
                .node_hsn_nics
                .unwrap_or_default()
                .into_iter()
                .map(ArtifactSummary::from)
                .collect(),
            // FIXME: implement FROM trait from HWInvByLlocHSNNIC to ArtifactSummary
        }
    }
}

impl Into<HWInvByLocNode> for NodeSummary {
    fn into(self) -> HWInvByLocNode {
        let redfish_system_fru_info = RedfishSystemFRUInfo {
            asset_tag: None,
            bios_version: None,
            model: None,
            manufacturer: None,
            part_number: None,
            serial_number: None,
            sku: None,
            system_type: None,
            uuid: None,
        };

        let hw_inv_by_fr_node = HWInvByFRUNode {
            fru_id: None,
            r#type: None,
            fru_sub_type: None,
            hw_inventory_by_fru_type: self.r#type.clone(),
            node_fru_info: redfish_system_fru_info,
        };

        HWInvByLocNode {
            id: self.xname,
            r#type: Some(self.r#type.clone()),
            ordinal: None,
            status: None,
            hw_inventory_by_location_type: self.r#type,
            populated_fru: Some(hw_inv_by_fr_node),
            node_location_info: None,
            processors: Some(
                self.processors
                    .into_iter()
                    .map(|processor| processor.into())
                    .collect(),
            ),
            node_accels: Some(
                self.node_accels
                    .into_iter()
                    .map(|node_accel| node_accel.into())
                    .collect(),
            ),
            drives: None,
            memory: Some(
                self.memory
                    .into_iter()
                    .map(|memory| memory.into())
                    .collect(),
            ),
            node_accel_risers: None, // FIXME: implement From/Into traits!
            node_hsn_nics: Some(
                self.node_hsn_nics
                    .into_iter()
                    .map(|node_hsn_nic| node_hsn_nic.into())
                    .collect(),
            ),
        }
    }
}

impl From<FrontEndNodeSummary> for NodeSummary {
    fn from(value: FrontEndNodeSummary) -> Self {
        NodeSummary {
            xname: value.xname,
            r#type: value.r#type,
            processors: value
                .processors
                .into_iter()
                .map(ArtifactSummary::from)
                .collect(),
            memory: value
                .memory
                .into_iter()
                .map(ArtifactSummary::from)
                .collect(),
            node_accels: value
                .node_accels
                .into_iter()
                .map(ArtifactSummary::from)
                .collect(),
            node_hsn_nics: value
                .node_hsn_nics
                .into_iter()
                .map(ArtifactSummary::from)
                .collect(),
        }
    }
}

impl Into<FrontEndNodeSummary> for NodeSummary {
    fn into(self) -> FrontEndNodeSummary {
        FrontEndNodeSummary {
            xname: self.xname,
            r#type: self.r#type,
            processors: self
                .processors
                .into_iter()
                .map(ArtifactSummary::into)
                .collect(),
            memory: self.memory.into_iter().map(ArtifactSummary::into).collect(),
            node_accels: self
                .node_accels
                .into_iter()
                .map(ArtifactSummary::into)
                .collect(),
            node_hsn_nics: self
                .node_hsn_nics
                .into_iter()
                .map(ArtifactSummary::into)
                .collect(),
        }
    }
}

impl NodeSummary {
    pub fn from_csm_value(hw_artifact_value: Value) -> Self {
        let processors = hw_artifact_value["Processors"]
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .map(|processor_value| ArtifactSummary::from_processor_value(processor_value.clone()))
            .collect();

        let memory = hw_artifact_value["Memory"]
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .map(|memory_value| ArtifactSummary::from_memory_value(memory_value.clone()))
            .collect();

        let node_accels = hw_artifact_value["NodeAccels"]
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .map(|nodeaccel_value| ArtifactSummary::from_nodeaccel_value(nodeaccel_value.clone()))
            .collect();

        let node_hsn_nics = hw_artifact_value["NodeHsnNics"]
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .map(|nodehsnnic_value| {
                ArtifactSummary::from_nodehsnnics_value(nodehsnnic_value.clone())
            })
            .collect();

        Self {
            xname: hw_artifact_value["ID"].as_str().unwrap().to_string(),
            r#type: hw_artifact_value["Type"].as_str().unwrap().to_string(),
            processors,
            memory,
            node_accels,
            node_hsn_nics,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArtifactSummary {
    pub xname: String,
    pub r#type: ArtifactType,
    pub info: Option<String>,
}

impl From<HWInvByLocProcessor> for ArtifactSummary {
    fn from(value: HWInvByLocProcessor) -> Self {
        ArtifactSummary {
            xname: value.id,
            r#type: ArtifactType::from_str(value.r#type.unwrap().as_str()).unwrap(),
            info: value.populated_fru.and_then(|hw_inv_by_fru_processor| {
                hw_inv_by_fru_processor.processor_fru_info.model
            }),
        }
    }
}

impl Into<HWInvByLocProcessor> for ArtifactSummary {
    fn into(self) -> HWInvByLocProcessor {
        let redfish_process_fru_info = RedfishProcessorFRUInfo {
            instruction_set: None,
            manufacturer: None,
            max_speed_mhz: None,
            model: self.info,
            processor_architecture: None,
            processor_id: None,
            processor_type: None,
            total_cores: None,
            total_threads: None,
        };

        let hw_inv_by_fru_processor = HWInvByFRUProcessor {
            fru_id: None,
            r#type: Some(self.r#type.to_string()),
            fru_sub_type: None,
            hw_inventory_by_fru_type: self.r#type.to_string(),
            processor_fru_info: redfish_process_fru_info,
        };

        let redfish_processor_location_info = RedfishProcessorLocationInfo {
            id: None,
            name: None,
            description: None,
            socket: None,
        };

        HWInvByLocProcessor {
            id: self.xname,
            r#type: Some(self.r#type.to_string()),
            ordinal: None,
            status: None,
            hw_inventory_by_location_type: self.r#type.to_string(),
            populated_fru: Some(hw_inv_by_fru_processor),
            processor_location_info: redfish_processor_location_info,
        }
    }
}

impl From<HWInvByLocMemory> for ArtifactSummary {
    fn from(value: HWInvByLocMemory) -> Self {
        ArtifactSummary {
            xname: value.id,
            r#type: ArtifactType::from_str(value.r#type.unwrap().as_str()).unwrap(),
            info: value.populated_fru.and_then(|hw_inv_by_fru_memory| {
                hw_inv_by_fru_memory
                    .memory_fru_info
                    .capacity_mib
                    .map(|capacity_mib| capacity_mib.to_string())
            }),
        }
    }
}

impl Into<HWInvByLocMemory> for ArtifactSummary {
    fn into(self) -> HWInvByLocMemory {
        let redfish_memory_fru_info = RedfishMemoryFRUInfo {
            base_module_type: None,
            bus_width_bits: None,
            capacity_mib: self.info.map(|info| usize::from_str(&info).unwrap_or(0)), // FIXME:
            // settings memory capacity to 0 if any issue... this does not look ok...
            data_width_bits: None,
            error_correction: None,
            manufacturer: None,
            memory_type: None,
            memory_device_type: None,
            operating_speed_mhz: None,
            part_number: None,
            rank_count: None,
            serial_number: None,
        };

        let hw_inv_by_fru_memory = HWInvByFRUMemory {
            fru_id: None,
            r#type: Some(self.r#type.to_string()),
            fru_sub_type: None,
            hw_inventory_by_fru_type: self.r#type.to_string(),
            memory_fru_info: redfish_memory_fru_info,
        };

        let memory_location = MemoryLocation {
            socket: None,
            memory_controller: None,
            channel: None,
            slot: None,
        };

        let redfish_memory_location_info = RedfishMemoryLocationInfo {
            id: None,
            name: None,
            description: None,
            memory_location: Some(memory_location),
        };

        HWInvByLocMemory {
            id: self.xname,
            r#type: Some(self.r#type.to_string()),
            ordinal: None,
            status: None,
            hw_inventory_by_location_type: self.r#type.to_string(),
            populated_fru: Some(hw_inv_by_fru_memory),
            memory_location_info: redfish_memory_location_info,
        }
    }
}

impl From<HWInvByLocNodeAccel> for ArtifactSummary {
    fn from(value: HWInvByLocNodeAccel) -> Self {
        ArtifactSummary {
            xname: value.id,
            r#type: ArtifactType::from_str(value.r#type.unwrap().as_str()).unwrap(),
            info: value.populated_fru.and_then(|hw_inv_by_fru_node_accel| {
                hw_inv_by_fru_node_accel.node_accel_fru_info.model
            }),
        }
    }
}

impl Into<HWInvByLocNodeAccel> for ArtifactSummary {
    fn into(self) -> HWInvByLocNodeAccel {
        // NOTE: yes, sounds weird FRU for node accelerator uses FRU for processor... but that is
        // what the API docs says...
        let redfish_processor_fru_info = RedfishProcessorFRUInfo {
            instruction_set: None,
            manufacturer: None,
            max_speed_mhz: None,
            model: self.info,
            processor_architecture: None,
            processor_id: None,
            processor_type: None,
            total_cores: None,
            total_threads: None,
        };

        let hw_inv_by_fru_node_accel = HWInvByFRUNodeAccel {
            fru_id: None,
            r#type: Some(self.r#type.to_string()),
            fru_sub_type: None,
            hw_inventory_by_fru_type: self.r#type.to_string(),
            node_accel_fru_info: redfish_processor_fru_info,
        };

        HWInvByLocNodeAccel {
            id: self.xname,
            r#type: Some(self.r#type.to_string()),
            ordinal: None,
            status: None,
            hw_inventory_by_location_type: self.r#type.to_string(),
            populated_fru: Some(hw_inv_by_fru_node_accel),
            node_accel_location_info: None,
        }
    }
}

impl From<HWInvByLocHSNNIC> for ArtifactSummary {
    fn from(value: HWInvByLocHSNNIC) -> Self {
        ArtifactSummary {
            xname: value.id,
            r#type: ArtifactType::from_str(value.r#type.unwrap().as_str()).unwrap(),
            info: value.populated_fru.and_then(|hw_inv_by_fru_hsn_nic| {
                hw_inv_by_fru_hsn_nic.node_hsn_nic_fru_info.model
            }),
        }
    }
}

impl Into<HWInvByLocHSNNIC> for ArtifactSummary {
    fn into(self) -> HWInvByLocHSNNIC {
        // NOTE: yes, sounds weird FRU for node accelerator uses FRU for processor... but that is
        // what the API docs says...
        let hsn_nic_fru_info = NodeHsnNicFRUInfo {
            manufacturer: None,
            model: self.info,
            part_number: None,
            sku: None,
            serial_number: None,
        };

        let hw_inv_by_fru_hsn_nic = HWInvByFRUHSNNIC {
            fru_id: None,
            r#type: Some(self.r#type.to_string()),
            fru_sub_type: None,
            hw_inventory_by_fru_type: self.r#type.to_string(),
            node_hsn_nic_fru_info: hsn_nic_fru_info,
        };

        let hsn_nic_location_info = HSNNICLocationInfo {
            id: None,
            name: None,
            description: None,
        };

        HWInvByLocHSNNIC {
            id: self.xname,
            r#type: Some(self.r#type.to_string()),
            ordinal: None,
            status: None,
            hw_inventory_by_location_type: self.r#type.to_string(),
            populated_fru: Some(hw_inv_by_fru_hsn_nic),
            hsn_nic_location_info,
        }
    }
}

impl From<FrontEndArtifactSummary> for ArtifactSummary {
    fn from(value: FrontEndArtifactSummary) -> Self {
        ArtifactSummary {
            xname: value.xname,
            r#type: value.r#type.into(),
            info: value.info.map(|v| v),
        }
    }
}

impl Into<FrontEndArtifactSummary> for ArtifactSummary {
    fn into(self) -> FrontEndArtifactSummary {
        FrontEndArtifactSummary {
            xname: self.xname,
            r#type: self.r#type.into(),
            info: self.info.map(|v| v),
        }
    }
}

impl ArtifactSummary {
    fn from_processor_value(processor_value: Value) -> Self {
        Self {
            xname: processor_value["ID"].as_str().unwrap().to_string(),
            r#type: ArtifactType::from_str(processor_value["Type"].as_str().unwrap()).unwrap(),
            info: processor_value
                .pointer("/PopulatedFRU/ProcessorFRUInfo/Model")
                .map(|model| model.as_str().unwrap().to_string()),
        }
    }

    fn from_memory_value(memory_value: Value) -> Self {
        Self {
            xname: memory_value["ID"].as_str().unwrap().to_string(),
            r#type: ArtifactType::from_str(memory_value["Type"].as_str().unwrap()).unwrap(),
            info: memory_value
                .pointer("/PopulatedFRU/MemoryFRUInfo/CapacityMiB")
                .map(|capacity_mib| capacity_mib.as_number().unwrap().to_string() + " MiB"),
        }
    }

    fn from_nodehsnnics_value(nodehsnnic_value: Value) -> Self {
        Self {
            xname: nodehsnnic_value["ID"].as_str().unwrap().to_string(),
            r#type: ArtifactType::from_str(nodehsnnic_value["Type"].as_str().unwrap()).unwrap(),
            info: nodehsnnic_value
                .pointer("/NodeHsnNicLocationInfo/Description")
                .map(|description| description.as_str().unwrap().to_string()),
        }
    }

    fn from_nodeaccel_value(nodeaccel_value: Value) -> Self {
        Self {
            xname: nodeaccel_value["ID"].as_str().unwrap().to_string(),
            r#type: ArtifactType::from_str(nodeaccel_value["Type"].as_str().unwrap()).unwrap(),
            info: nodeaccel_value
                .pointer("/PopulatedFRU/NodeAccelFRUInfo/Model")
                .map(|model| model.as_str().unwrap().to_string()),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// CSM - structs from CSM API documentation. FIXME: need to address FRU structs properly with enums
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
pub struct ProcessorId {
    #[serde(rename = "EffectiveFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_family: Option<String>,
    #[serde(rename = "EffectiveModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efffective_model: Option<String>,
    #[serde(rename = "IdentificationRegisters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identification_registers: Option<String>,
    #[serde(rename = "MicrocodeInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub microcode_info: Option<String>,
    #[serde(rename = "Step")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<String>,
    #[serde(rename = "VendorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishProcessorFRUInfo {
    #[serde(rename = "InstructionSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruction_set: Option<String>,
    #[serde(rename = "Manufacturer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[serde(rename = "MaxSpeedMHz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_speed_mhz: Option<usize>,
    #[serde(rename = "Model")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "ProcessorArchitecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_architecture: Option<String>,
    #[serde(rename = "ProcessorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_id: Option<ProcessorId>,
    #[serde(rename = "ProcessorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_type: Option<String>,
    #[serde(rename = "TotalCores")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cores: Option<usize>,
    #[serde(rename = "TotalThreads")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_threads: Option<usize>,
}

impl From<FrontEndRedfishProcessorFRUInfo> for RedfishProcessorFRUInfo {
    fn from(value: FrontEndRedfishProcessorFRUInfo) -> Self {
        RedfishProcessorFRUInfo {
            instruction_set: value.instruction_set.map(|v| v),
            manufacturer: value.manufacturer.map(|v| v),
            max_speed_mhz: value.max_speed_mhz.map(|v| v),
            model: value.model.map(|v| v),
            processor_architecture: value.processor_architecture.map(|v| v),
            processor_id: None,
            processor_type: value.processor_type.map(|v| v),
            total_cores: value.total_cores.map(|v| v),
            total_threads: value.total_threads.map(|v| v),
        }
    }
}

impl Into<FrontEndRedfishProcessorFRUInfo> for RedfishProcessorFRUInfo {
    fn into(self) -> FrontEndRedfishProcessorFRUInfo {
        FrontEndRedfishProcessorFRUInfo {
            instruction_set: self.instruction_set.map(|v| v),
            manufacturer: self.manufacturer.map(|v| v),
            max_speed_mhz: self.max_speed_mhz.map(|v| v),
            model: self.model.map(|v| v),
            processor_architecture: self.processor_architecture.map(|v| v),
            processor_id: None, // FIXME: Implement From and Into traits for this field/type
            processor_type: self.processor_type.map(|v| v),
            total_cores: self.total_cores.map(|v| v),
            total_threads: self.total_threads.map(|v| v),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByFRUProcessor {
    #[serde(rename = "FRUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fru_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "FRUSubType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fru_sub_type: Option<String>,
    #[serde(rename = "HWInventoryByFRUType")]
    pub hw_inventory_by_fru_type: String,
    #[serde(rename = "ProcessorFRUInfo")]
    pub processor_fru_info: RedfishProcessorFRUInfo,
}

impl From<FrontEndHWInvByFRUProcessor> for HWInvByFRUProcessor {
    fn from(value: FrontEndHWInvByFRUProcessor) -> Self {
        HWInvByFRUProcessor {
            fru_id: value.fru_id.map(|v| v),
            r#type: value.r#type.map(|v| v),
            fru_sub_type: value.fru_sub_type.map(|v| v),
            hw_inventory_by_fru_type: value.hw_inventory_by_fru_type,
            processor_fru_info: RedfishProcessorFRUInfo::from(value.processor_fru_info),
        }
    }
}

impl Into<FrontEndHWInvByFRUProcessor> for HWInvByFRUProcessor {
    fn into(self) -> FrontEndHWInvByFRUProcessor {
        FrontEndHWInvByFRUProcessor {
            fru_id: self.fru_id.map(|v| v),
            r#type: self.r#type.map(|v| v),
            fru_sub_type: self.fru_sub_type.map(|v| v),
            hw_inventory_by_fru_type: self.hw_inventory_by_fru_type,
            processor_fru_info: self.processor_fru_info.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishMemoryFRUInfo {
    #[serde(rename = "BaseModuleType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_module_type: Option<String>,
    #[serde(rename = "BusWidthBits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bus_width_bits: Option<usize>,
    #[serde(rename = "CapacityMiB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_mib: Option<usize>,
    #[serde(rename = "DataWidthBits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_width_bits: Option<usize>,
    #[serde(rename = "ErrorCorrection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_correction: Option<String>,
    #[serde(rename = "Manufacturer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[serde(rename = "MemoryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_type: Option<String>,
    #[serde(rename = "MemoryDeviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_device_type: Option<String>,
    #[serde(rename = "OperatingSpeedMhz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_speed_mhz: Option<usize>,
    #[serde(rename = "PartNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_number: Option<String>,
    #[serde(rename = "RankCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank_count: Option<usize>,
    #[serde(rename = "SerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
}

impl From<FrontEndRedfishMemoryFRUInfo> for RedfishMemoryFRUInfo {
    fn from(value: FrontEndRedfishMemoryFRUInfo) -> Self {
        RedfishMemoryFRUInfo {
            base_module_type: value.base_module_type.map(|v| v),
            bus_width_bits: value.bus_width_bits.map(|v| v),
            capacity_mib: value.capacity_mib.map(|v| v),
            data_width_bits: value.data_width_bits.map(|v| v),
            error_correction: value.error_correction.map(|v| v),
            manufacturer: value.manufacturer.map(|v| v),
            memory_type: value.memory_type.map(|v| v),
            memory_device_type: value.memory_device_type.map(|v| v),
            operating_speed_mhz: value.operating_speed_mhz.map(|v| v),
            part_number: value.part_number.map(|v| v),
            rank_count: value.rank_count.map(|v| v),
            serial_number: value.serial_number.map(|v| v),
        }
    }
}

impl Into<FrontEndRedfishMemoryFRUInfo> for RedfishMemoryFRUInfo {
    fn into(self) -> FrontEndRedfishMemoryFRUInfo {
        FrontEndRedfishMemoryFRUInfo {
            base_module_type: self.base_module_type.map(|v| v),
            bus_width_bits: self.bus_width_bits.map(|v| v),
            capacity_mib: self.capacity_mib.map(|v| v),
            data_width_bits: self.data_width_bits.map(|v| v),
            error_correction: self.error_correction.map(|v| v),
            manufacturer: self.manufacturer.map(|v| v),
            memory_type: self.memory_type.map(|v| v),
            memory_device_type: self.memory_device_type.map(|v| v),
            operating_speed_mhz: self.operating_speed_mhz.map(|v| v),
            part_number: self.part_number.map(|v| v),
            rank_count: self.rank_count.map(|v| v),
            serial_number: self.serial_number.map(|v| v),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByFRUMemory {
    #[serde(rename = "FRUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fru_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "FRUSubType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fru_sub_type: Option<String>,
    #[serde(rename = "HWInventoryByFRUType")]
    pub hw_inventory_by_fru_type: String,
    #[serde(rename = "MemoryFRUInfo")]
    pub memory_fru_info: RedfishMemoryFRUInfo,
}

impl From<FrontEndHWInvByFRUMemory> for HWInvByFRUMemory {
    fn from(value: FrontEndHWInvByFRUMemory) -> Self {
        HWInvByFRUMemory {
            fru_id: value.fru_id.map(|v| v),
            r#type: value.r#type.map(|v| v),
            fru_sub_type: value.fru_sub_type.map(|v| v),
            hw_inventory_by_fru_type: value.hw_inventory_by_fru_type,
            memory_fru_info: RedfishMemoryFRUInfo::from(value.memory_fru_info),
        }
    }
}

impl Into<FrontEndHWInvByFRUMemory> for HWInvByFRUMemory {
    fn into(self) -> FrontEndHWInvByFRUMemory {
        FrontEndHWInvByFRUMemory {
            fru_id: self.fru_id.map(|v| v),
            r#type: self.r#type.map(|v| v),
            fru_sub_type: self.fru_sub_type.map(|v| v),
            hw_inventory_by_fru_type: self.hw_inventory_by_fru_type,
            memory_fru_info: self.memory_fru_info.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByFRUNodeAccel {
    #[serde(rename = "FRUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fru_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "FRUSubType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fru_sub_type: Option<String>,
    #[serde(rename = "HWInventoryByFRUType")]
    pub hw_inventory_by_fru_type: String,
    #[serde(rename = "NodeAccelFRUInfo")]
    pub node_accel_fru_info: RedfishProcessorFRUInfo, // NOTE: according to API
                                                      // docs, yes this is using the redfish for "processor"
}

impl From<FrontEndHWInvByFRUNodeAccel> for HWInvByFRUNodeAccel {
    fn from(value: FrontEndHWInvByFRUNodeAccel) -> Self {
        HWInvByFRUNodeAccel {
            fru_id: value.fru_id.map(|v| v),
            r#type: value.r#type.map(|v| v),
            fru_sub_type: value.fru_sub_type.map(|v| v),
            hw_inventory_by_fru_type: value.hw_inventory_by_fru_type,
            node_accel_fru_info: RedfishProcessorFRUInfo::from(value.node_accel_fru_info),
        }
    }
}

impl Into<FrontEndHWInvByFRUNodeAccel> for HWInvByFRUNodeAccel {
    fn into(self) -> FrontEndHWInvByFRUNodeAccel {
        FrontEndHWInvByFRUNodeAccel {
            fru_id: self.fru_id.map(|v| v),
            r#type: self.r#type.map(|v| v),
            fru_sub_type: self.fru_sub_type.map(|v| v),
            hw_inventory_by_fru_type: self.hw_inventory_by_fru_type,
            node_accel_fru_info: self.node_accel_fru_info.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeHsnNicFRUInfo {
    #[serde(rename = "Manufacturer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[serde(rename = "Model")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "PartNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_number: Option<String>,
    #[serde(rename = "SKU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(rename = "SerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
}

impl From<FrontEndHSNNICFRUInfo> for NodeHsnNicFRUInfo {
    fn from(value: FrontEndHSNNICFRUInfo) -> Self {
        NodeHsnNicFRUInfo {
            manufacturer: value.manufacturer.map(|v| v),
            model: value.model.map(|v| v),
            part_number: value.part_number.map(|v| v),
            sku: value.sku.map(|v| v),
            serial_number: value.serial_number.map(|v| v),
        }
    }
}

impl Into<FrontEndHSNNICFRUInfo> for NodeHsnNicFRUInfo {
    fn into(self) -> FrontEndHSNNICFRUInfo {
        FrontEndHSNNICFRUInfo {
            manufacturer: self.manufacturer.map(|v| v),
            model: self.model.map(|v| v),
            part_number: self.part_number.map(|v| v),
            sku: self.sku.map(|v| v),
            serial_number: self.serial_number.map(|v| v),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByFRUHSNNIC {
    #[serde(rename = "FRUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fru_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "FRUSubType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fru_sub_type: Option<String>,
    #[serde(rename = "HWInventoryByFRUType")]
    pub hw_inventory_by_fru_type: String,
    #[serde(rename = "NodeHsnNicFRUInfo")]
    pub node_hsn_nic_fru_info: NodeHsnNicFRUInfo,
}

impl From<FrontEndHWInvByFRUHSNNIC> for HWInvByFRUHSNNIC {
    fn from(value: FrontEndHWInvByFRUHSNNIC) -> Self {
        HWInvByFRUHSNNIC {
            fru_id: value.fru_id.map(|v| v),
            r#type: value.r#type.map(|v| v),
            fru_sub_type: value.fru_sub_type.map(|v| v),
            hw_inventory_by_fru_type: value.hw_inventory_by_fru_type,
            node_hsn_nic_fru_info: NodeHsnNicFRUInfo::from(value.hsn_nic_fru_info),
        }
    }
}

impl Into<FrontEndHWInvByFRUHSNNIC> for HWInvByFRUHSNNIC {
    fn into(self) -> FrontEndHWInvByFRUHSNNIC {
        FrontEndHWInvByFRUHSNNIC {
            fru_id: self.fru_id.map(|v| v),
            r#type: self.r#type.map(|v| v),
            fru_sub_type: self.fru_sub_type.map(|v| v),
            hw_inventory_by_fru_type: self.hw_inventory_by_fru_type,
            hsn_nic_fru_info: self.node_hsn_nic_fru_info.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInventoryByFRU {
    #[serde(rename = "FRUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fru_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "FRUSubType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fru_sub_type: Option<String>,
    #[serde(rename = "HWInventoryByFRUType")]
    pub hw_inventory_by_fru_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishChassisLocationInfo {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocChassis {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename = "ChassisLocatinInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chassis_location_info: Option<RedfishChassisLocationInfo>,
    #[serde(rename = "ComputeModules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_modules: Option<HWInvByLocComputeModule>,
    #[serde(rename = "RouterModules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_modules: Option<HWInvByLocRouterModule>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocNodeEnclosure {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename = "NodeEnclosureLocationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_enclosure_location_info: Option<RedfishChassisLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocComputeModule {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename = "ComputeModuleLocationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_module_location_info: Option<RedfishChassisLocationInfo>,
    #[serde(rename = "NodeEnclosures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_enclosures: Option<HWInvByLocNodeEnclosure>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocHSNBoard {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename = "HSNBoardLocationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsn_board_location_info: Option<RedfishChassisLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocRouterModule {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename = "RouterModuleLocationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_module_location_info: Option<RedfishChassisLocationInfo>,
    pub hsn_boards: Option<HWInvByLocHSNBoard>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocCabinet {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename = "CabinetLocationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cabinet_location_info: Option<RedfishChassisLocationInfo>,
    #[serde(rename = "Chassis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chassis: Option<HWInvByLocChassis>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocMgmtSwitch {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename = "MgmtSwitchLocationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mgmt_switch_location_info: Option<RedfishChassisLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocMgmtHLSwitch {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename = "MgmtHLSwitchLocationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mgmt_hl_switch_location_info: Option<RedfishChassisLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocCDUMgmtSwitch {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename = "CDUMgmtSwitchLocationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdu_mgmt_switch_location_info: Option<RedfishChassisLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessorSummary {
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<u32>,
    #[serde(rename = "Model")]
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<String>,
}

impl From<FrontEndProcessorSummary> for ProcessorSummary {
    fn from(value: FrontEndProcessorSummary) -> Self {
        ProcessorSummary {
            count: value.count.map(|v| v),
            model: value.model.map(|v| v),
        }
    }
}

impl Into<FrontEndProcessorSummary> for ProcessorSummary {
    fn into(self) -> FrontEndProcessorSummary {
        FrontEndProcessorSummary {
            count: self.count.map(|v| v),
            model: self.model.map(|v| v),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemorySummary {
    #[serde(rename = "TotalSystemMemoryGiB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_system_memory_gib: Option<u32>,
}

impl From<FrontEndMemorySummary> for MemorySummary {
    fn from(value: FrontEndMemorySummary) -> Self {
        MemorySummary {
            total_system_memory_gib: value.total_system_memory_gib.map(|v| v),
        }
    }
}

impl Into<FrontEndMemorySummary> for MemorySummary {
    fn into(self) -> FrontEndMemorySummary {
        FrontEndMemorySummary {
            total_system_memory_gib: self.total_system_memory_gib.map(|v| v),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishSystemLocationInfo {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename = "ProcessorSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_summary: Option<ProcessorSummary>,
    #[serde(rename = "MemorySummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_summary: Option<MemorySummary>,
}

impl From<FrontEndRedfishSystemLocationInfo> for RedfishSystemLocationInfo {
    fn from(value: FrontEndRedfishSystemLocationInfo) -> Self {
        RedfishSystemLocationInfo {
            id: value.id.map(|v| v),
            name: value.name.map(|v| v),
            description: value.description.map(|v| v),
            hostname: value.hostname.map(|v| v),
            processor_summary: value.processor_summary.map(ProcessorSummary::from),
            memory_summary: value.memory_summary.map(MemorySummary::from),
        }
    }
}

impl Into<FrontEndRedfishSystemLocationInfo> for RedfishSystemLocationInfo {
    fn into(self) -> FrontEndRedfishSystemLocationInfo {
        FrontEndRedfishSystemLocationInfo {
            id: self.id.map(|v| v),
            name: self.name.map(|v| v),
            description: self.description.map(|v| v),
            hostname: self.hostname.map(|v| v),
            processor_summary: self.processor_summary.map(|v| v.into()),
            memory_summary: self.memory_summary.map(|v| v.into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishProcessorLocationInfo {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Socket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub socket: Option<String>,
}

impl From<FrontEndRedfishProcessorLocationInfo> for RedfishProcessorLocationInfo {
    fn from(value: FrontEndRedfishProcessorLocationInfo) -> Self {
        RedfishProcessorLocationInfo {
            id: value.id.map(|v| v),
            name: value.name.map(|v| v),
            description: value.description.map(|v| v),
            socket: value.socket.map(|v| v),
        }
    }
}

impl Into<FrontEndRedfishProcessorLocationInfo> for RedfishProcessorLocationInfo {
    fn into(self) -> FrontEndRedfishProcessorLocationInfo {
        FrontEndRedfishProcessorLocationInfo {
            id: self.id.map(|v| v),
            name: self.name.map(|v| v),
            description: self.description.map(|v| v),
            socket: self.socket.map(|v| v),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocProcessor {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInvByFRUProcessor>,
    #[serde(rename = "ProcessorLocationInfo")]
    pub processor_location_info: RedfishProcessorLocationInfo,
}

impl From<FrontEndHWInvByLocProcessor> for HWInvByLocProcessor {
    fn from(value: FrontEndHWInvByLocProcessor) -> Self {
        HWInvByLocProcessor {
            id: value.id,
            r#type: value.r#type.map(|v| v),
            ordinal: value.ordinal.map(|v| v),
            status: value.status.map(|v| v),
            hw_inventory_by_location_type: value.hw_inventory_by_location_type,
            populated_fru: value.populated_fru.map(HWInvByFRUProcessor::from),
            processor_location_info: RedfishProcessorLocationInfo::from(
                value.processor_location_info,
            ),
        }
    }
}

impl Into<FrontEndHWInvByLocProcessor> for HWInvByLocProcessor {
    fn into(self) -> FrontEndHWInvByLocProcessor {
        FrontEndHWInvByLocProcessor {
            id: self.id,
            r#type: self.r#type.map(|v| v),
            ordinal: self.ordinal.map(|v| v),
            status: self.status.map(|v| v),
            hw_inventory_by_location_type: self.hw_inventory_by_location_type,
            populated_fru: self.populated_fru.map(|v| v.into()),
            processor_location_info: self.processor_location_info.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocNodeAccel {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInvByFRUNodeAccel>,
    #[serde(rename = "NodeAccelLocationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_accel_location_info: Option<RedfishProcessorLocationInfo>, // NOTE: according to API
                                                                        // docs, yes this is using the redfish for "processor"
}

impl From<FrontEndHWInvByLocNodeAccel> for HWInvByLocNodeAccel {
    fn from(value: FrontEndHWInvByLocNodeAccel) -> Self {
        HWInvByLocNodeAccel {
            id: value.id,
            r#type: value.r#type.map(|v| v),
            ordinal: value.ordinal.map(|v| v),
            status: value.status.map(|v| v),
            hw_inventory_by_location_type: value.hw_inventory_by_location_type,
            populated_fru: value.populated_fru.map(HWInvByFRUNodeAccel::from),
            node_accel_location_info: value
                .node_accel_location_info
                .map(RedfishProcessorLocationInfo::from),
        }
    }
}

impl Into<FrontEndHWInvByLocNodeAccel> for HWInvByLocNodeAccel {
    fn into(self) -> FrontEndHWInvByLocNodeAccel {
        FrontEndHWInvByLocNodeAccel {
            id: self.id,
            r#type: self.r#type.map(|v| v),
            ordinal: self.ordinal.map(|v| v),
            status: self.status.map(|v| v),
            hw_inventory_by_location_type: self.hw_inventory_by_location_type,
            populated_fru: self.populated_fru.map(|v| v.into()),
            node_accel_location_info: self.node_accel_location_info.map(|v| v.into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishDriveLocationInfo {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocDrive {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename = "DriveLocationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drive_location_info: Option<RedfishDriveLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemoryLocation {
    #[serde(rename = "Socket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub socket: Option<u32>,
    #[serde(rename = "MemoryController")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_controller: Option<u32>,
    #[serde(rename = "Channel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<u32>,
    #[serde(rename = "Slot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<u32>,
}

impl From<FrontEndMemoryLocation> for MemoryLocation {
    fn from(value: FrontEndMemoryLocation) -> Self {
        MemoryLocation {
            socket: value.socket.map(|v| v),
            memory_controller: value.memory_controller.map(|v| v),
            channel: value.channel.map(|v| v),
            slot: value.slot.map(|v| v),
        }
    }
}

impl Into<FrontEndMemoryLocation> for MemoryLocation {
    fn into(self) -> FrontEndMemoryLocation {
        FrontEndMemoryLocation {
            socket: self.socket.map(|v| v),
            memory_controller: self.memory_controller.map(|v| v),
            channel: self.channel.map(|v| v),
            slot: self.slot.map(|v| v),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishMemoryLocationInfo {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "MemoryLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_location: Option<MemoryLocation>,
}

impl From<FrontEndRedfishMemoryLocationInfo> for RedfishMemoryLocationInfo {
    fn from(value: FrontEndRedfishMemoryLocationInfo) -> Self {
        RedfishMemoryLocationInfo {
            id: value.id.map(|v| v),
            name: value.name.map(|v| v),
            description: value.description.map(|v| v),
            memory_location: value.memory_location.map(MemoryLocation::from),
        }
    }
}

impl Into<FrontEndRedfishMemoryLocationInfo> for RedfishMemoryLocationInfo {
    fn into(self) -> FrontEndRedfishMemoryLocationInfo {
        FrontEndRedfishMemoryLocationInfo {
            id: self.id.map(|v| v),
            name: self.name.map(|v| v),
            description: self.description.map(|v| v),
            memory_location: self.memory_location.map(|v| v.into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocMemory {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInvByFRUMemory>,
    #[serde(rename = "MemoryLocationInfo")]
    pub memory_location_info: RedfishMemoryLocationInfo,
}

impl From<FrontEndHWInvByLocMemory> for HWInvByLocMemory {
    fn from(value: FrontEndHWInvByLocMemory) -> Self {
        HWInvByLocMemory {
            id: value.id,
            r#type: value.r#type.map(|v| v),
            ordinal: value.ordinal.map(|v| v),
            status: value.status.map(|v| v),
            hw_inventory_by_location_type: value.hw_inventory_by_location_type,
            populated_fru: value.populated_fru.map(HWInvByFRUMemory::from),
            memory_location_info: RedfishMemoryLocationInfo::from(value.memory_location_info),
        }
    }
}

impl Into<FrontEndHWInvByLocMemory> for HWInvByLocMemory {
    fn into(self) -> FrontEndHWInvByLocMemory {
        FrontEndHWInvByLocMemory {
            id: self.id,
            r#type: self.r#type.map(|v| v),
            ordinal: self.ordinal.map(|v| v),
            status: self.status.map(|v| v),
            hw_inventory_by_location_type: self.hw_inventory_by_location_type,
            populated_fru: self.populated_fru.map(|v| v.into()),
            memory_location_info: self.memory_location_info.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishNodeAccelRiserLocationInfo {
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocNodeAccelRiser {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename = "NodeAccelRiserLocationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_accel_riser_location_info: Option<RedfishNodeAccelRiserLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HSNNICLocationInfo {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl From<FrontEndHSNNICLocationInfo> for HSNNICLocationInfo {
    fn from(value: FrontEndHSNNICLocationInfo) -> Self {
        HSNNICLocationInfo {
            id: value.id.map(|v| v),
            name: value.name.map(|v| v),
            description: value.description.map(|v| v),
        }
    }
}

impl Into<FrontEndHSNNICLocationInfo> for HSNNICLocationInfo {
    fn into(self) -> FrontEndHSNNICLocationInfo {
        FrontEndHSNNICLocationInfo {
            id: self.id.map(|v| v),
            name: self.name.map(|v| v),
            description: self.description.map(|v| v),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocHSNNIC {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInvByFRUHSNNIC>,
    /* #[serde(rename = "NodeHsnNicLocationInfo")]
    pub node_hsn_nic_location_info: HSNNICLocationInfo, */
    #[serde(rename = "NodeHsnNicLocationInfo")]
    pub hsn_nic_location_info: HSNNICLocationInfo,
}

impl From<FrontEndHWInvByLocHSNNIC> for HWInvByLocHSNNIC {
    fn from(value: FrontEndHWInvByLocHSNNIC) -> Self {
        HWInvByLocHSNNIC {
            id: value.id,
            r#type: value.r#type.map(|v| v),
            ordinal: value.ordinal.map(|v| v),
            status: value.status.map(|v| v),
            hw_inventory_by_location_type: value.hw_inventory_by_location_type,
            populated_fru: value.populated_fru.map(HWInvByFRUHSNNIC::from),
            hsn_nic_location_info: HSNNICLocationInfo::from(value.hsn_nic_location_info),
        }
    }
}

impl Into<FrontEndHWInvByLocHSNNIC> for HWInvByLocHSNNIC {
    fn into(self) -> FrontEndHWInvByLocHSNNIC {
        FrontEndHWInvByLocHSNNIC {
            id: self.id,
            r#type: self.r#type.map(|v| v),
            ordinal: self.ordinal.map(|v| v),
            status: self.status.map(|v| v),
            hw_inventory_by_location_type: self.hw_inventory_by_location_type,
            populated_fru: self.populated_fru.map(|v| v.into()),
            hsn_nic_location_info: self.hsn_nic_location_info.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Hardware {
    #[serde(rename = "Hardware")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware: Option<Vec<HWInvByLocNode>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocNode {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInvByFRUNode>,
    #[serde(rename = "NodeLocationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_location_info: Option<RedfishSystemLocationInfo>,
    #[serde(rename = "Processors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processors: Option<Vec<HWInvByLocProcessor>>,
    #[serde(rename = "NodeAccels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_accels: Option<Vec<HWInvByLocNodeAccel>>,
    #[serde(rename = "Dives")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drives: Option<Vec<HWInvByLocDrive>>,
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<Vec<HWInvByLocMemory>>,
    #[serde(rename = "NodeAccelRisers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_accel_risers: Option<Vec<HWInvByLocNodeAccelRiser>>,
    #[serde(rename = "NodeHsnNICs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_hsn_nics: Option<Vec<HWInvByLocHSNNIC>>,
}

impl From<FrontEndHWInvByLocNode> for HWInvByLocNode {
    fn from(value: FrontEndHWInvByLocNode) -> Self {
        HWInvByLocNode {
            id: value.id,
            r#type: value.r#type.map(|v| v),
            ordinal: value.ordinal.map(|v| v),
            status: value.status.map(|v| v),
            hw_inventory_by_location_type: value.hw_inventory_by_location_type,
            populated_fru: value.populated_fru.map(HWInvByFRUNode::from),
            node_location_info: value
                .node_location_info
                .map(RedfishSystemLocationInfo::from),
            processors: value.processors.map(|processor_vec| {
                processor_vec
                    .into_iter()
                    .map(HWInvByLocProcessor::from)
                    .collect()
            }),
            node_accels: value.node_accels.map(|node_accel_vec| {
                node_accel_vec
                    .into_iter()
                    .map(HWInvByLocNodeAccel::from)
                    .collect()
            }),
            drives: None,
            memory: value
                .memory
                .map(|memory_vec| memory_vec.into_iter().map(HWInvByLocMemory::from).collect()),
            node_accel_risers: None,
            node_hsn_nics: value.node_hsn_nics.map(|node_hsn_nic_vec| {
                node_hsn_nic_vec
                    .into_iter()
                    .map(HWInvByLocHSNNIC::from)
                    .collect()
            }),
        }
    }
}

impl Into<FrontEndHWInvByLocNode> for HWInvByLocNode {
    fn into(self) -> FrontEndHWInvByLocNode {
        FrontEndHWInvByLocNode {
            id: self.id,
            r#type: self.r#type.map(|v| v),
            ordinal: self.ordinal.map(|v| v),
            status: self.status.map(|v| v),
            hw_inventory_by_location_type: self.hw_inventory_by_location_type,
            populated_fru: self.populated_fru.map(|v| v.into()),
            node_location_info: self.node_location_info.map(|v| v.into()),
            processors: self.processors.map(|processor_vec| {
                processor_vec
                    .into_iter()
                    .map(|processor| processor.into())
                    .collect()
            }),
            node_accels: self.node_accels.map(|node_accel_vec| {
                node_accel_vec
                    .into_iter()
                    .map(|node_accel| node_accel.into())
                    .collect()
            }),
            drives: None,
            memory: self
                .memory
                .map(|memory_vec| memory_vec.into_iter().map(|memory| memory.into()).collect()),
            node_accel_risers: None,
            node_hsn_nics: self
                .node_hsn_nics
                .map(|node_hsn_nic_vec| node_hsn_nic_vec.into_iter().map(|v| v.into()).collect()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishPDULocationInfo {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "UUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishOutletLocationInfo {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocOutlet {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename = "OutletLocationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outlet_location_info: Option<RedfishOutletLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocPDU {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename = "PDULocationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdu_location_info: Option<RedfishPDULocationInfo>,
    #[serde(rename = "CabinetPDUPowerConnectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cabinet_pdu_power_connectors: Option<Vec<HWInvByLocOutlet>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishCMMRectifierLocationInfo {
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "FirmwareVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocCMMRectifier {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename = "CMMRectifierLocationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    cmm_rectifier_location_info: Option<RedfishCMMRectifierLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishNodeEnclosurePowerSupplyLocationInfo {
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "FirmwareVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocNodePowerSupply {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename = "NodeEnclosurePowerSupplyLocationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_enclosure_power_supply_location_info:
        Option<RedfishNodeEnclosurePowerSupplyLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishManagerLocationInfo {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time: Option<String>,
    #[serde(rename = "DateTimeLocalOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time_local_offset: Option<String>,
    #[serde(rename = "FirmwareVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocNodeBMC {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename = "NodeBMCLocationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_bmc_location_info: Option<RedfishManagerLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocRouterBMC {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename = "RouterBMCLocationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_bmc_location_info: Option<RedfishManagerLocationInfo>,
}

/* #[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInventoryList {
    #[serde(rename = "Hardware")]
    pub hw_inventory: Vec<HWInventory>,
}

impl From<FrontEndHWInventoryList> for HWInventoryList {
    fn from(value: FrontEndHWInventoryList) -> Self {
        HWInventoryList {
            hw_inventory: value
                .hw_inventory
                .into_iter()
                .map(HWInventory::from)
                .collect(),
        }
    }
}

impl Into<FrontEndHWInventoryList> for HWInventoryList {
    fn into(self) -> FrontEndHWInventoryList {
        FrontEndHWInventoryList {
            hw_inventory: self
                .hw_inventory
                .into_iter()
                .map(|hw_inventory| hw_inventory.into())
                .collect(),
        }
    }
} */

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInventory {
    #[serde(rename = "XName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xname: Option<String>,
    #[serde(rename = "Format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "Cabinets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cabinets: Option<Vec<HWInvByLocCabinet>>,
    #[serde(rename = "Chassis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chassis: Option<Vec<HWInvByLocChassis>>,
    #[serde(rename = "ComputeModules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_modules: Option<Vec<HWInvByLocComputeModule>>,
    #[serde(rename = "RouterModules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_modules: Option<Vec<HWInvByLocRouterModule>>,
    #[serde(rename = "NodeEnclosures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_enclosures: Option<Vec<HWInvByLocNodeEnclosure>>,
    #[serde(rename = "HSNBoards")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsn_boards: Option<Vec<HWInvByLocHSNBoard>>,
    #[serde(rename = "MgmtSwitches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mgmt_switches: Option<Vec<HWInvByLocMgmtSwitch>>,
    #[serde(rename = "MgmtHLSwitches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mgmt_hl_switches: Option<Vec<HWInvByLocMgmtHLSwitch>>,
    #[serde(rename = "CDUMgmtSwitches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdu_mgmt_switches: Option<Vec<HWInvByLocCDUMgmtSwitch>>,
    #[serde(rename = "Nodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<HWInvByLocNode>>,
    #[serde(rename = "Processors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processors: Option<Vec<HWInvByLocProcessor>>,
    #[serde(rename = "NodeAccels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_accels: Option<Vec<HWInvByLocNodeAccel>>,
    #[serde(rename = "Drives")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drives: Option<Vec<HWInvByLocDrive>>,
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<Vec<HWInvByLocMemory>>,
    #[serde(rename = "CabinetPDUs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cabinet_pdus: Option<Vec<HWInvByLocPDU>>,
    #[serde(rename = "CabinetPDUPowerConnectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cabinet_pdu_power_connectors: Option<Vec<HWInvByLocOutlet>>,
    #[serde(rename = "CMMRectifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmm_rectifiers: Option<Vec<HWInvByLocCMMRectifier>>,
    #[serde(rename = "NodeAccelRisers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_accel_risers: Option<Vec<HWInvByLocNodeAccelRiser>>,
    #[serde(rename = "NodeHsnNics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_hsn_nics: Option<Vec<HWInvByLocHSNNIC>>,
    #[serde(rename = "NodeEnclosurePowerSupplies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_enclosure_power_supplies: Option<Vec<HWInvByLocNodePowerSupply>>,
    #[serde(rename = "NodeBMC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_bmc: Option<Vec<HWInvByLocNodeBMC>>,
    #[serde(rename = "RouterBMC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_bmc: Option<Vec<HWInvByLocRouterBMC>>,
}

impl From<FrontEndHWInventory> for HWInventory {
    fn from(value: FrontEndHWInventory) -> Self {
        HWInventory {
            xname: value.xname.map(|v| v),
            format: value.format.map(|v| v),
            cabinets: None, // FIXME: Implement From and Into traits for this field/type
            chassis: None,  // FIXME: Implement From and Into traits for this field/type
            compute_modules: None, // FIXME: Implement From and Into traits for this field/type
            router_modules: None, // FIXME: Implement From and Into traits for this field/type
            node_enclosures: None, // FIXME: Implement From and Into traits for this field/type
            hsn_boards: None, // FIXME: Implement From and Into traits for this field/type
            mgmt_switches: None, // FIXME: Implement From and Into traits for this field/type
            mgmt_hl_switches: None, // FIXME: Implement From and Into traits for this field/type
            cdu_mgmt_switches: None, // FIXME: Implement From and Into traits for this field/type
            nodes: value
                .nodes
                .map(|node_vec| node_vec.into_iter().map(HWInvByLocNode::from).collect()),
            processors: value.processors.map(|processor_vec| {
                processor_vec
                    .into_iter()
                    .map(HWInvByLocProcessor::from)
                    .collect()
            }),
            node_accels: value.node_accels.map(|node_accel_vec| {
                node_accel_vec
                    .into_iter()
                    .map(HWInvByLocNodeAccel::from)
                    .collect()
            }),
            drives: None, // FIXME: Implement From and Into traits for this field/type
            memory: value
                .memory
                .map(|memory_vec| memory_vec.into_iter().map(HWInvByLocMemory::from).collect()),
            cabinet_pdus: None, // FIXME: Implement From and Into traits for this field/type
            cabinet_pdu_power_connectors: None, // FIXME: Implement From and Into traits for this field/type
            cmm_rectifiers: None, // FIXME: Implement From and Into traits for this field/type
            node_accel_risers: None, // FIXME: Implement From and Into traits for this field/type
            node_hsn_nics: None,  // FIXME: Implement From and Into traits for this field/type
            node_enclosure_power_supplies: None, // FIXME: Implement From and Into traits for this field/type
            node_bmc: None, // FIXME: Implement From and Into traits for this field/type
            router_bmc: None, // FIXME: Implement From and Into traits for this field/type
        }
    }
}

impl Into<FrontEndHWInventory> for HWInventory {
    fn into(self) -> FrontEndHWInventory {
        FrontEndHWInventory {
            xname: self.xname.map(|v| v),
            format: self.format.map(|v| v),
            cabinets: None, // FIXME: Implement From and Into traits for this field/type
            chassis: None,  // FIXME: Implement From and Into traits for this field/type
            compute_modules: None, // FIXME: Implement From and Into traits for this field/type
            router_modules: None, // FIXME: Implement From and Into traits for this field/type
            node_enclosures: None, // FIXME: Implement From and Into traits for this field/type
            hsn_boards: None, // FIXME: Implement From and Into traits for this field/type
            mgmt_switches: None, // FIXME: Implement From and Into traits for this field/type
            mgmt_hl_switches: None, // FIXME: Implement From and Into traits for this field/type
            cdu_mgmt_switches: None, // FIXME: Implement From and Into traits for this field/type
            nodes: self
                .nodes
                .map(|node_vec| node_vec.into_iter().map(|node| node.into()).collect()),
            processors: self.processors.map(|processor_vec| {
                processor_vec
                    .into_iter()
                    .map(|processor| processor.into())
                    .collect()
            }),
            node_accels: self.node_accels.map(|node_accel_vec| {
                node_accel_vec
                    .into_iter()
                    .map(|node_accel| node_accel.into())
                    .collect()
            }),
            drives: None, // FIXME: Implement From and Into traits for this field/type
            memory: self
                .memory
                .map(|memory_vec| memory_vec.into_iter().map(|memory| memory.into()).collect()),
            cabinet_pdus: None, // FIXME: Implement From and Into traits for this field/type
            cabinet_pdu_power_connectors: None, // FIXME: Implement From and Into traits for this field/type
            cmm_rectifiers: None, // FIXME: Implement From and Into traits for this field/type
            node_accel_risers: None, // FIXME: Implement From and Into traits for this field/type
            node_hsn_nics: None,  // FIXME: Implement From and Into traits for this field/type
            node_enclosure_power_supplies: None, // FIXME: Implement From and Into traits for this field/type
            node_bmc: None, // FIXME: Implement From and Into traits for this field/type
            router_bmc: None, // FIXME: Implement From and Into traits for this field/type
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByFRUNode {
    #[serde(rename = "FRUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fru_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "FRUSubType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fru_sub_type: Option<String>,
    #[serde(rename = "HWInventoryByFRUType")]
    pub hw_inventory_by_fru_type: String,
    #[serde(rename = "NodeFRUInfo")]
    pub node_fru_info: RedfishSystemFRUInfo,
}

impl From<FrontEndHWInvByFRUNode> for HWInvByFRUNode {
    fn from(value: FrontEndHWInvByFRUNode) -> Self {
        HWInvByFRUNode {
            fru_id: value.fru_id.map(|v| v),
            r#type: value.r#type.map(|v| v),
            fru_sub_type: value.fru_sub_type.map(|v| v),
            hw_inventory_by_fru_type: value.hw_inventory_by_fru_type,
            node_fru_info: RedfishSystemFRUInfo::from(value.node_fru_info),
        }
    }
}

impl Into<FrontEndHWInvByFRUNode> for HWInvByFRUNode {
    fn into(self) -> FrontEndHWInvByFRUNode {
        FrontEndHWInvByFRUNode {
            fru_id: self.fru_id.map(|v| v),
            r#type: self.r#type.map(|v| v),
            fru_sub_type: self.fru_sub_type.map(|v| v),
            hw_inventory_by_fru_type: self.hw_inventory_by_fru_type,
            node_fru_info: self.node_fru_info.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishSystemFRUInfo {
    #[serde(rename = "AssetTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_tag: Option<String>,
    #[serde(rename = "BiosVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bios_version: Option<String>,
    #[serde(rename = "Model")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "Manufacturer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[serde(rename = "PartNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_number: Option<String>,
    #[serde(rename = "SerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(rename = "SKU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(rename = "SystemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_type: Option<String>,
    #[serde(rename = "UUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

impl From<FrontEndRedfishSystemFRUInfo> for RedfishSystemFRUInfo {
    fn from(value: FrontEndRedfishSystemFRUInfo) -> Self {
        RedfishSystemFRUInfo {
            asset_tag: value.asset_tag.map(|v| v),
            bios_version: value.bios_version.map(|v| v),
            model: value.model.map(|v| v),
            manufacturer: value.manufacturer.map(|v| v),
            part_number: value.part_number.map(|v| v),
            serial_number: value.serial_number.map(|v| v),
            sku: value.sku.map(|v| v),
            system_type: value.system_type.map(|v| v),
            uuid: value.uuid.map(|v| v),
        }
    }
}

impl Into<FrontEndRedfishSystemFRUInfo> for RedfishSystemFRUInfo {
    fn into(self) -> FrontEndRedfishSystemFRUInfo {
        FrontEndRedfishSystemFRUInfo {
            asset_tag: self.asset_tag.map(|v| v),
            bios_version: self.bios_version.map(|v| v),
            model: self.model.map(|v| v),
            manufacturer: self.manufacturer.map(|v| v),
            part_number: self.part_number.map(|v| v),
            serial_number: self.serial_number.map(|v| v),
            sku: self.sku.map(|v| v),
            system_type: self.system_type.map(|v| v),
            uuid: self.uuid.map(|v| v),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeLocationInfo {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename = "ProcessorSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_summary: Option<ProcessorSummary>,
    #[serde(rename = "MemorySummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_summary: Option<MemorySummary>,
}

impl From<FrontEndNodeLocationInfo> for NodeLocationInfo {
    fn from(value: FrontEndNodeLocationInfo) -> Self {
        NodeLocationInfo {
            id: value.id,
            name: value.name,
            description: value.description,
            hostname: value.hostname,
            processor_summary: value.processor_summary.map(ProcessorSummary::from),
            memory_summary: value.memory_summary.map(MemorySummary::from),
        }
    }
}

impl Into<FrontEndNodeLocationInfo> for NodeLocationInfo {
    fn into(self) -> FrontEndNodeLocationInfo {
        FrontEndNodeLocationInfo {
            id: self.id,
            name: self.name,
            description: self.description,
            hostname: self.hostname,
            processor_summary: self.processor_summary.map(|value| value.into()),
            memory_summary: self.memory_summary.map(|value| value.into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)] // <-- this is important. More info https://serde.rs/enum-representations.html#untagged
pub enum HWInventoryByLocation {
    /* HWInvByLocCabinet(HWInvByLocCabinet),
    HWInvByLocChassis(HWInvByLocChassis),
    HWInvByLocComputeModule(HWInvByLocComputeModule),
    HWInvByLocRouterModule(HWInvByLocRouterModule),
    HWInvByLocNodeEnclosure(HWInvByLocNodeEnclosure),
    HWInvByLocHSNBoard(HWInvByLocHSNBoard),
    HWInvByLocMgmtSwitch(HWInvByLocMgmtSwitch),
    HWInvByLocMgmtHLSwitch(HWInvByLocMgmtHLSwitch),
    HWInvByLocCDUMgmtSwitch(HWInvByLocCDUMgmtSwitch), */
    HWInvByLocNode(HWInvByLocNode),
    HWInvByLocProcessor(HWInvByLocProcessor),
    HWInvByLocNodeAccel(HWInvByLocNodeAccel),
    /*     HWInvByLocDrive(HWInvByLocDrive), */
    HWInvByLocMemory(HWInvByLocMemory),
    /* HWInvByLocPDU(HWInvByLocPDU),
    HWInvByLocOutlet(HWInvByLocOutlet),
    HWInvByLocCMMRectifier(HWInvByLocCMMRectifier),
    HWInvByLocNodeAccelRiser(HWInvByLocNodeAccelRiser), */
    HWInvByLocHSNNIC(HWInvByLocHSNNIC),
    /* HWInvByLocNodePowerSupply(HWInvByLocNodePowerSupply),
    HWInvByLocNodeBMC(HWInvByLocNodeBMC),
    HWInvByLocRouterBMC(HWInvByLocRouterBMC), */
}

impl From<FrontEndHWInventoryByLocation> for HWInventoryByLocation {
    fn from(f: FrontEndHWInventoryByLocation) -> Self {
        match f {
            FrontEndHWInventoryByLocation::HWInvByLocNode(hwinv_by_loc_nnode) => {
                HWInventoryByLocation::HWInvByLocNode(HWInvByLocNode::from(hwinv_by_loc_nnode))
            }
            FrontEndHWInventoryByLocation::HWInvByLocProcessor(hwinv_by_loc_processor) => {
                HWInventoryByLocation::HWInvByLocProcessor(HWInvByLocProcessor::from(
                    hwinv_by_loc_processor,
                ))
            }
            FrontEndHWInventoryByLocation::HWInvByLocNodeAccel(hwinv_by_node_accel) => {
                HWInventoryByLocation::HWInvByLocNodeAccel(HWInvByLocNodeAccel::from(
                    hwinv_by_node_accel,
                ))
            }
            FrontEndHWInventoryByLocation::HWInvByLocMemory(hwinv_by_loc_memory) => {
                HWInventoryByLocation::HWInvByLocMemory(HWInvByLocMemory::from(hwinv_by_loc_memory))
            }
            FrontEndHWInventoryByLocation::HWInvByLocHSNNIC(hwinv_by_loc_hsnnic) => {
                HWInventoryByLocation::HWInvByLocHSNNIC(HWInvByLocHSNNIC::from(hwinv_by_loc_hsnnic))
            }
        }
    }
}

impl Into<FrontEndHWInventoryByLocation> for HWInventoryByLocation {
    fn into(self) -> FrontEndHWInventoryByLocation {
        match self {
            HWInventoryByLocation::HWInvByLocNode(f) => {
                FrontEndHWInventoryByLocation::HWInvByLocNode(f.into())
            }
            HWInventoryByLocation::HWInvByLocProcessor(f) => {
                FrontEndHWInventoryByLocation::HWInvByLocProcessor(f.into())
            }
            HWInventoryByLocation::HWInvByLocNodeAccel(f) => {
                FrontEndHWInventoryByLocation::HWInvByLocNodeAccel(f.into())
            }
            HWInventoryByLocation::HWInvByLocMemory(f) => {
                FrontEndHWInventoryByLocation::HWInvByLocMemory(f.into())
            }
            HWInventoryByLocation::HWInvByLocHSNNIC(hwinv_by_loc_hsnnic) => {
                FrontEndHWInventoryByLocation::HWInvByLocHSNNIC(hwinv_by_loc_hsnnic.into())
            }
        }
    }
}

/// struct used in POST and GET endpoints that manage multiple instances of 'HWInventoryByLocation'
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInventoryByLocationList {
    #[serde(rename = "Hardware")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware: Option<Vec<HWInventoryByLocation>>,
}

impl From<FrontEndHWInventoryByLocationList> for HWInventoryByLocationList {
    fn from(value: FrontEndHWInventoryByLocationList) -> Self {
        HWInventoryByLocationList {
            hardware: value.hardware.map(|hardware_vec| {
                hardware_vec
                    .into_iter()
                    .map(HWInventoryByLocation::from)
                    .collect()
            }),
        }
    }
}

impl Into<FrontEndHWInventoryByLocationList> for HWInventoryByLocationList {
    fn into(self) -> FrontEndHWInventoryByLocationList {
        FrontEndHWInventoryByLocationList {
            hardware: self.hardware.map(|hardware_vec| {
                hardware_vec
                    .into_iter()
                    .map(|hardware_inventory_by_location| hardware_inventory_by_location.into())
                    .collect()
            }),
        }
    }
}
