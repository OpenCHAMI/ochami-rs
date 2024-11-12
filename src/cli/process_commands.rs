use clap::ArgMatches;
use config::Config;

use crate::{
    backend_api::hsm::{group, partition},
    error::Error,
};

pub fn process_cli(
    cli_root: ArgMatches,
    base_url: &str,
    auth_token: &str,
    root_cert: &[u8],
    settings: &Config,
) -> core::result::Result<(), Error> {
    if let Some(cli_get) = cli_root.subcommand_matches("get") {
        if let Some(cli_get_group) = cli_get.subcommand_matches("groups") {
            let label: Option<&str> = cli_get_group
                .get_one("label")
                .map(|label: &String| label.as_str());
            let tags_opt: Option<&str> = cli_get_group
                .get_one("tags")
                .map(|tags: &String| tags.as_str());

            let group_vec =
                group::http_client::get(base_url, auth_token, root_cert, label, tags_opt).unwrap();

            println!("DEBUG - groups:\n{:#?}", group_vec);
        } else if let Some(cli_get_partition) = cli_get.subcommand_matches("partitions") {
            let name: Option<&str> = cli_get_partition
                .get_one("name")
                .cloned()
                .map(|name: &String| name.as_str());
            let tags_opt: Option<&str> = cli_get_partition
                .get_one("tags")
                .cloned()
                .map(|tags: &String| tags.as_str());

            let partition_vec =
                partition::http_client::get(base_url, auth_token, root_cert, name, tags_opt)
                    .unwrap();

            println!("DEBUG - partitions:\n{:#?}", partition_vec);
        }
    } else if let Some(cli_add) = cli_root.subcommand_matches("add") {
        if let Some(cli_add_group) = cli_add.subcommand_matches("group") {
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

            let group = group::types::HsmGroup {
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
        } else if let Some(cli_add_partition) = cli_add.subcommand_matches("partition") {
            let name: &String = cli_add_partition.get_one("name").unwrap();
            let description_opt: Option<&String> = cli_add_partition.get_one("description");
            let member_vec_opt: Option<&Vec<String>> = cli_add_partition.get_one("members");
            let tags_opt: Option<Vec<String>> = cli_add_partition
                .get_one("tags")
                .map(|tags: &String| tags.split(",").map(|tag| tag.trim().to_string()).collect());

            let member = partition::types::Member {
                ids: member_vec_opt.cloned(),
            };

            let partition = partition::types::Partition {
                name: name.to_string(),
                members: Some(member),
                description: description_opt.cloned(),
                tags: tags_opt,
            };

            let _ = partition::http_client::post(base_url, auth_token, root_cert, partition);
        } else {
        }
    } else if let Some(cli_update) = cli_root.subcommand_matches("update") {
        if let Some(_cli_update_group) = cli_update.subcommand_matches("group") {
            println!("Update group");
        } else if let Some(_cli_update_partition) = cli_update.subcommand_matches("partition") {
            println!("Update group");
        } else {
        }
    }

    Ok(())
}
