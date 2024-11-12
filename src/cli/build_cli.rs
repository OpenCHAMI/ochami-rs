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
}

pub fn add_command() -> Command {
    Command::new("add")
        .visible_alias("a")
        .arg_required_else_help(true)
        .about("Create new data to backend")
        .subcommand(add_group_command())
        .subcommand(add_partition_command())
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
