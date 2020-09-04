use todolst::components::{ item::* };
use clap::{ ArgMatches };
use chrono::prelude::*;
// use futures::executor::block_on;
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
    system.new_group(title).expect("Fail to add new group");
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
    system.new_list(title).expect("Fail to add new list");
    if let Some(group) = group {
        let group = system.group(group).unwrap();
        system.set_list_group(title, Some(group)).unwrap();
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
        system.set_item_level(&item, level);
    }
    if sub_m.is_present("today") {
        let today = sub_m.value_of("today").expect("today should not be empty.").parse().expect("today should be able to parsed to a boolean value.");
        system.set_item_today(&item, today);
    }
    if sub_m.is_present("notice") {
        let notice = sub_m.value_of("notice").expect("notice should not be none.");
        let notice = if notice.len()==0 || notice.to_lowercase()=="none" {
            None
        } else {
            Some(NaiveDateTime::parse_from_str(notice, "%Y-%m-%dT%H:%M:%S").expect("Fail to parse notice to a NaiveDateTime. Notice should format in \"%Y-%m-%dT%H:%M:%S\"."))
        };
        system.set_item_notice(&item, notice);
    }
    if sub_m.is_present("deadline") {
        let deadline = sub_m.value_of("deadline").expect("deadline should not be none.");
        let deadline = if deadline.len()==0 || deadline.to_lowercase()=="none" {
            None
        } else {
            Some(NaiveDate::parse_from_str(deadline, "%Y-%m-%d").expect("Fail to parse notice to a NaiveDate. Notice should format in \"%Y-%m-%d\"."))
        };
        system.set_item_deadline(&item, deadline);
    }
    if sub_m.is_present("plan") {
        let plan = sub_m.value_of("plan").expect("plan should not be none.");
        let plan = if plan.len()==0 || plan.to_lowercase()=="none" {
            None
        } else {
            Some(NaiveDate::parse_from_str(plan, "%Y-%m-%d").expect("Fail to parse notice to a NaiveDate. Notice should format in \"%Y-%m-%d\"."))
        };
        system.set_item_plan(&item, plan);
    }
    if sub_m.is_present("repeat") {
        let repeat = sub_m.value_of("repeat").expect("repeat should not be none.");
        let repeat = repeat.to_lowercase();
        let repeat_len = repeat.len();
        let repeat = if repeat=="none" || repeat_len==0 {
            None
        } else {
            let last_char = &repeat[repeat_len-1..];
            match last_char {
                "y" => { Some(RepeatSpan::Years(repeat[..repeat_len-1].parse().expect("Fail to parse repeat"))) },
                "m" => { Some(RepeatSpan::Months(repeat[..repeat_len-1].parse().expect("Fail to parse repeat"))) },
                "w" => { Some(RepeatSpan::Weeks(repeat[..repeat_len-1].parse().expect("Fail to parse repeat"))) },
                "d" => { Some(RepeatSpan::Days(repeat[..repeat_len-1].parse().expect("Fail to parse repeat"))) },
                _ => { panic!("Fail to parse repeat. Use none instead."); }
            }
        };
        system.set_item_repeat(&item, repeat);
    }
    if sub_m.is_present("finished") {
        let finished = sub_m.value_of("finished").expect("finished should not be empty.").parse().expect("finished should be parsed to a boolean value.");
        system.set_item_finished(&item, finished);
    }
    if sub_m.is_present("note") {
        let note = sub_m.value_of("note").expect("note should not be empty.");
        system.set_item_note(&item, note);
    }

    save_todolst(&system);
}