use clap::{arg, Command};

pub fn build_cli() -> Command {
    Command::new(env!("CARGO_PKG_NAME"))
        .term_width(100)
        .version(env!("CARGO_PKG_VERSION"))
        .arg_required_else_help(true)
        .subcommand(get_command())
        .subcommand(add_command())
        .subcommand(update_command())
}

pub fn get_command() -> Command {
    Command::new("get")
        .visible_alias("g")
        .arg_required_else_help(true)
        .about("Get data from backend")
        .subcommand(get_group_command())
        .subcommand(get_partition_command())
        .subcommand(get_bootparameters_command())
}

pub fn add_command() -> Command {
    Command::new("add")
        .visible_alias("a")
        .arg_required_else_help(true)
        .about("Create new data to backend")
        .subcommand(add_group_command())
        .subcommand(add_partition_command())
        .subcommand(add_boot_parameters_command())
}

pub fn update_command() -> Command {
    Command::new("update")
        .visible_alias("u")
        .arg_required_else_help(true)
        .about("Update existing data to backend")
        .subcommand(update_group_command())
        .subcommand(update_partition_command())
}

pub fn get_group_command() -> Command {
    Command::new("groups")
        .visible_alias("g")
        .about("Get partitions")
        .arg(arg!(-l --label <VALUE> "Group label"))
        .arg(arg!(-t --tags <VALUE> "Group tags"))
}

pub fn get_partition_command() -> Command {
    Command::new("partitions")
        .visible_alias("p")
        .about("Get partitions")
        .arg(arg!(-n --name <VALUE> "Partition name"))
        .arg(arg!(-t --tags <VALUE> "Partition tags"))
}

pub fn get_bootparameters_command() -> Command {
    Command::new("bootparameters")
        .visible_alias("b")
        .about("Get partitions")
        .arg(arg!(-H --hosts <VALUE> "Comma separated list of hosts"))
    // TODO: add the rest of arguments
}

pub fn add_group_command() -> Command {
    Command::new("group")
        .visible_alias("g")
        .arg_required_else_help(true)
        .about("Create new HSM group")
        .arg(arg!(-l --label <VALUE> "Group label").required(true))
        .arg(arg!(-d --description <VALUE> "Group description"))
        .arg(arg!(-m --members <VALUE> "Comma separated list of nodes."))
        .arg(arg!(-t --tags <VALUE> "Comma separated list of tags"))
        .arg(arg!(-x --"exclusive-group" <VALUE> "Exclusive group name"))
}

pub fn add_partition_command() -> Command {
    Command::new("partition")
        .visible_alias("p")
        .arg_required_else_help(true)
        .about("Create new HSM group")
        .arg(arg!(-n --name <VALUE> "Partition name").required(true))
        .arg(arg!(-m --members <VALUE> "Comma separated list of nodes."))
        .arg(arg!(-d --description <VALUE> "Partition description"))
        .arg(arg!(-m --members <VALUE> "Comma separated list of nodes."))
        .arg(arg!(-t --tags <VALUE> "Comma separated list of tags"))
}

pub fn add_boot_parameters_command() -> Command {
    Command::new("bootparameters")
        .visible_alias("b")
        .arg_required_else_help(true)
        .about("Create new boot parameters")
        .arg(arg!(-H --hosts <VALUE> "Comma separated list of ").required(true))
        .arg(arg!(-m --macs <VALUE> "Comma separated list of mac addresses."))
        .arg(arg!(-n --nids <VALUE> "Comma separated list of nids"))
        .arg(arg!(-p --params <VALUE> "List of kernel parameters").required(true))
        .arg(arg!(-k --kernel <VALUE> "S3 url to kernel image file").required(true))
        .arg(arg!(-i --initrd <VALUE> "S3 url to initrd image file").required(true))
        .arg(arg!(-c --"cloud-init" <VALUE> "Cloud-init data"))
}

pub fn update_group_command() -> Command {
    Command::new("group")
        .visible_alias("g")
        .arg_required_else_help(true)
        .about("Update new HSM group")
        .arg(arg!(-n --name <VALUE> "Group name").required(true))
        .arg(arg!(-m --members <VALUE> "Comma separated list of nodes."))
}

pub fn update_partition_command() -> Command {
    Command::new("partition")
        .visible_alias("p")
        .arg_required_else_help(true)
        .about("Update new HSM group")
        .arg(arg!(-n --name <VALUE> "Group name").required(true))
        .arg(arg!(-m --members <VALUE> "Comma separated list of nodes."))
}
