use clap::ArgMatches;

use crate::{
    backend_api::{
        bss::{self, types::BootParameters},
        hsm::{group, partition},
    },
    error::Error,
};

pub async fn process_cli(
    cli_root: ArgMatches,
    base_url: &str,
    auth_token: &str,
    root_cert: &[u8],
) -> core::result::Result<(), Error> {
    if let Some(cli_get) = cli_root.subcommand_matches("get") {
        if let Some(cli_get_group) = cli_get.subcommand_matches("groups") {
            get_group(cli_get_group, base_url, auth_token, root_cert).await;
        } else if let Some(cli_get_partition) = cli_get.subcommand_matches("partitions") {
            get_partition(cli_get_partition, base_url, auth_token, root_cert);
        } else if let Some(cli_get_bootparameters) = cli_get.subcommand_matches("bootparameters") {
            get_bootparameters(cli_get_bootparameters, base_url, auth_token, root_cert).await;
        }
    } else if let Some(cli_add) = cli_root.subcommand_matches("add") {
        if let Some(cli_add_group) = cli_add.subcommand_matches("group") {
            add_group(cli_add_group, base_url, auth_token, root_cert);
        } else if let Some(cli_add_partition) = cli_add.subcommand_matches("partition") {
            add_partition(cli_add_partition, base_url, auth_token, root_cert);
        } else if let Some(cli_add_boot_parameters) = cli_add.subcommand_matches("bootparameters") {
            add_boot_parameters(cli_add_boot_parameters, base_url, auth_token, root_cert).await;
        }
    }
    Ok(())
}

pub async fn get_group(
    cli_get_group: &ArgMatches,
    base_url: &str,
    auth_token: &str,
    root_cert: &[u8],
) {
    let label: Option<&str> = cli_get_group
        .get_one("label")
        .map(|label: &String| label.as_str());
    let tags_opt: Option<&str> = cli_get_group
        .get_one("tags")
        .map(|tags: &String| tags.as_str());

    let group_vec = group::http_client::get(base_url, auth_token, root_cert, label, tags_opt)
        .await
        .unwrap();

    println!("DEBUG - groups:\n{:#?}", group_vec);
}

pub fn get_partition(
    cli_get_partition: &ArgMatches,
    base_url: &str,
    auth_token: &str,
    root_cert: &[u8],
) {
    let name: Option<&str> = cli_get_partition
        .get_one("name")
        .cloned()
        .map(|name: &String| name.as_str());
    let tags_opt: Option<&str> = cli_get_partition
        .get_one("tags")
        .cloned()
        .map(|tags: &String| tags.as_str());

    let partition_vec =
        partition::http_client::get(base_url, auth_token, root_cert, name, tags_opt).unwrap();

    println!("DEBUG - partitions:\n{:#?}", partition_vec);
}

pub async fn get_bootparameters(
    cli_get_bootparameters: &ArgMatches,
    base_url: &str,
    auth_token: &str,
    root_cert: &[u8],
) {
    let host_vec_opt: Option<Vec<String>> = cli_get_bootparameters
        .get_one::<String>("hosts")
        .and_then(|hosts| {
            Some(
                hosts
                    .split(",")
                    .map(|host| host.trim().to_string())
                    .collect(),
            )
        });

    let partition_vec_rslt =
        bss::http_client::get(base_url, auth_token, root_cert, &host_vec_opt).await;

    match partition_vec_rslt {
        Ok(partition_vec) => println!("{:#?}", partition_vec),
        Err(e) => eprintln!("{}", e),
    }
}

pub fn add_group(cli_add_group: &ArgMatches, base_url: &str, auth_token: &str, root_cert: &[u8]) {
    let label: &String = cli_add_group.get_one("label").unwrap();
    let description_opt: Option<&String> = cli_add_group.get_one("description");
    let member_vec_opt: Option<&Vec<String>> = cli_add_group.get_one("members");
    let tags_opt: Option<Vec<String>> = cli_add_group
        .get_one("tags")
        .map(|tags: &String| tags.split(",").map(|tag| tag.trim().to_string()).collect());
    let exclusive_group_opt: Option<&String> = cli_add_group.get_one("exclusive-group");

    let member = group::types::Member {
        ids: member_vec_opt.cloned(),
    };

    let group = group::types::Group {
        label: label.to_string(),
        members: Some(member),
        description: description_opt.cloned(),
        tags: tags_opt,
        exclusive_group: exclusive_group_opt.cloned(),
    };

    let group_rslt = group::http_client::post(base_url, auth_token, root_cert, group);

    match group_rslt {
        Ok(group) => println!("DEBUG - group:\n{:#?}", group),
        Err(e) => eprintln!("ERROR - Could not create group. Reason:\n{:#?}", e),
    }
}

pub fn add_partition(
    cli_add_partition: &ArgMatches,
    base_url: &str,
    auth_token: &str,
    root_cert: &[u8],
) {
    let name: &String = cli_add_partition.get_one("name").unwrap();
    let description_opt: Option<&String> = cli_add_partition.get_one("description");
    let member_vec_opt: Option<Vec<String>> = cli_add_partition
        .get_one("members")
        .map(|tags: &String| tags.split(",").map(|tag| tag.trim().to_string()).collect());
    let tag_vec_opt: Option<Vec<String>> = cli_add_partition
        .get_one("tags")
        .map(|tags: &String| tags.split(",").map(|tag| tag.trim().to_string()).collect());

    let member = partition::types::Member {
        ids: member_vec_opt,
    };

    let partition = partition::types::Partition {
        name: name.to_string(),
        members: Some(member),
        description: description_opt.cloned(),
        tags: tag_vec_opt,
    };

    let _ = partition::http_client::post(base_url, auth_token, root_cert, partition);
}

pub async fn add_boot_parameters(
    cli_add_boot_parameters: &ArgMatches,
    base_url: &str,
    auth_token: &str,
    root_cert: &[u8],
) {
    let host_vec: Vec<String> = cli_add_boot_parameters
        .get_one::<String>("hosts")
        .unwrap()
        .split(",")
        .map(|host| host.trim().to_string())
        .collect();
    let mac_vec_opt: Option<Vec<String>> = cli_add_boot_parameters
        .get_one::<String>("macs")
        .map(|macs| macs.split(",").map(|mac| mac.trim().to_string()).collect());
    let nid_vec_opt: Option<Vec<u32>> =
        cli_add_boot_parameters
            .get_one::<String>("nids")
            .map(|nids| {
                nids.split(",")
                    .map(|nid| {
                        nid.trim()
                            .parse::<u32>()
                            .expect(format!("nid '{}' not valid", nid).as_str())
                    })
                    .collect()
            });
    let params_opt: &String = cli_add_boot_parameters.get_one("params").unwrap();
    let kernel_opt: &String = cli_add_boot_parameters.get_one("kernel").unwrap();
    let initrd_opt: &String = cli_add_boot_parameters.get_one("initrd").unwrap();
    let cloud_init_opt: Option<&String> = cli_add_boot_parameters.get_one("cloud-init");

    let boot_parameters = BootParameters {
        hosts: host_vec,
        macs: mac_vec_opt,
        nids: nid_vec_opt,
        params: params_opt.to_string(),
        kernel: kernel_opt.to_string(),
        initrd: initrd_opt.to_string(),
        cloud_init: cloud_init_opt.and_then(|value| serde_json::from_str(value).unwrap()),
    };

    /* let boot_parameters_rslt =
    bss::http_client::post(base_url, auth_token, root_cert, boot_parameters); */

    let boot_parameters_rslt =
        bss::http_client::put(base_url, auth_token, root_cert, &boot_parameters).await;

    match boot_parameters_rslt {
        Ok(_) => println!("Boot parameters created"),
        Err(e) => eprintln!("{}", e),
    }
}
