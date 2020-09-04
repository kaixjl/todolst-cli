use todolst::components::{ group::*, list::*, item::*, todolst::* };
use clap::{ App, Arg, SubCommand, ArgMatches };
use futures::executor::block_on;
use crate::utils::{ load_todolst, save_todolst };

pub fn modify_command(sub_m: &ArgMatches) {
    match sub_m.subcommand() {
        ("group", Some(sub_m)) => { modify_group_command(sub_m) },
        ("list", Some(sub_m)) => { modify_list_command(sub_m) },
        ("item", Some(sub_m)) => { modify_item_command(sub_m) },
        _ => (),
    }
}

fn modify_group_command(sub_m: &ArgMatches) {

}

fn modify_list_command(sub_m: &ArgMatches) {

}

fn modify_item_command(sub_m: &ArgMatches) {
    
}