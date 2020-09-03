use todolst::components::{ group::*, list::*, item::*, todolst::* };
use clap::{ App, Arg, SubCommand, ArgMatches };
use futures::executor::block_on;
use crate::utils::{ load_todolst, save_todolst };

pub fn add_command(sub_m: &ArgMatches) {
    match sub_m.subcommand() {
        ("group", Some(sub_m)) => { add_group_command(sub_m) },
        ("list", Some(sub_m)) => { add_list_command(sub_m) },
        ("item", Some(sub_m)) => { add_item_command(sub_m) },
        _ => (),
    }
}

fn add_group_command(sub_m: &ArgMatches) {
    let title = sub_m.value_of("GROUP").expect("GROUP should not be empty.");
    let parent = if sub_m.is_present("PARENT") { 
        let parent = sub_m.value_of("PARENT").expect("PARENT should not be empty."); 
        if parent.len() > 0 { Some(parent) }
        else { None }
    } else { None };
    let mut system = load_todolst();
    let group = system.new_group(title).expect("Fail to add new group");
    if let Some(parent) = parent {
        let parent = system.group(parent).unwrap();
        system.set_group_parent(title, Some(parent)).unwrap();
    }
    save_todolst(&system);
}

fn add_list_command(sub_m: &ArgMatches) {
    let title = sub_m.value_of("LIST").expect("LIST should not be empty.");
    let group = if sub_m.is_present("GROUP") { 
        let group = sub_m.value_of("GROUP").expect("GROUP should not be empty."); 
        if group.len() > 0 { Some(group) }
        else { None }
    } else { None };
    let mut system = load_todolst();
    let list = system.new_list(title).expect("Fail to add new list");
    if let Some(group) = group {
        let group = system.group(group).unwrap();
        system.set_list_group(title, Some(group)).unwrap();
    }
    save_todolst(&system);
}

fn add_item_command(sub_m: &ArgMatches) {

}