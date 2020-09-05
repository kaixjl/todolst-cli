use clap::{ ArgMatches };
// use futures::executor::block_on;
use crate::utils::*;

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
        system.set_group_parent(&group, Some(parent)).unwrap();
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
        system.set_list_group(&list, Some(group)).unwrap();
    }
    save_todolst(&system);
}

fn add_item_command(sub_m: &ArgMatches) {
    let message = sub_m.value_of("MESSAGE").expect("MESSAGE should not be empty.");
    let list = sub_m.value_of("LIST").expect("LIST should not be empty.");

    let mut system = load_todolst();

    let list = system.list(list).expect(format!("No list named {} found.", list).as_ref());

    let item = system.new_item(message, list);

    if sub_m.is_present("level") {
        let level = sub_m.value_of("level").expect("level should not be empty.").parse().expect("level should be a number ranging from 0 to 127. No more than 3 is recommanded.");
        system.set_item_level(&item, level).unwrap();
    }
    if sub_m.is_present("today") {
        let today = sub_m.value_of("today").expect("today should not be empty.").parse().expect("today should be able to parsed to a boolean value.");
        system.set_item_today(&item, today).unwrap();
    }
    if sub_m.is_present("notice") {
        let notice = sub_m.value_of("notice").expect("notice should not be none.");
        let notice = parse_NaiveDateTime(notice);
        system.set_item_notice(&item, notice).unwrap();
    }
    if sub_m.is_present("deadline") {
        let deadline = sub_m.value_of("deadline").expect("deadline should not be none.");
        let deadline = parse_NaiveDate(deadline);
        system.set_item_deadline(&item, deadline).unwrap();
    }
    if sub_m.is_present("plan") {
        let plan = sub_m.value_of("plan").expect("plan should not be none.");
        let plan = parse_NaiveDate(plan);
        system.set_item_plan(&item, plan).unwrap();
    }
    if sub_m.is_present("repeat") {
        let repeat = sub_m.value_of("repeat").expect("repeat should not be none.");
        let repeat = parse_RepeatSpan(repeat);
        system.set_item_repeat(&item, repeat).unwrap();
    }
    if sub_m.is_present("finished") {
        let finished = sub_m.value_of("finished").expect("finished should not be empty.").parse().expect("finished should be parsed to a boolean value.");
        system.set_item_finished(&item, finished).unwrap();
    }
    if sub_m.is_present("note") {
        let note = sub_m.value_of("note").expect("note should not be empty.");
        system.set_item_note(&item, note).unwrap();
    }

    save_todolst(&system);
}