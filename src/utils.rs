#![allow(non_snake_case)]
use todolst::components::{ todolst::* };
use futures::executor::block_on;
use chrono::prelude::*;
use todolst::components::item::RepeatSpan;

pub fn load_todolst() -> TodoLst {
    let system = block_on(TodoLst::load());
    system
}

pub fn save_todolst(system: &TodoLst) {
    block_on(system.save()).expect("save failed.");
}

pub fn parse_NaiveDateTime(txt: &str) -> Option<NaiveDateTime> {
    if txt.len()==0 || txt.to_lowercase()=="none" {
        None
    } else {
        Some(NaiveDateTime::parse_from_str(txt, "%Y-%m-%dT%H:%M:%S").expect("Fail to parse notice to a NaiveDateTime. Notice should format in \"%Y-%m-%dT%H:%M:%S\"."))
    }
}

pub fn parse_NaiveDate(txt: &str) -> Option<NaiveDate> {
    if txt.len()==0 || txt.to_lowercase()=="none" {
        None
    } else {
        Some(NaiveDate::parse_from_str(txt, "%Y-%m-%d").expect("Fail to parse notice to a NaiveDate. Notice should format in \"%Y-%m-%d\"."))
    }
}

pub fn parse_RepeatSpan(txt: &str) -> Option<RepeatSpan> {
    let repeat = txt.to_lowercase();
    let repeat_len = repeat.len();
    if repeat=="none" || repeat_len==0 {
        None
    } else {
        let last_char = &repeat[repeat_len-1..];
        match last_char {
            "y" => { Some(RepeatSpan::Years(repeat[..repeat_len-1].parse().expect("Fail to parse repeat"))) },
            "m" => { Some(RepeatSpan::Months(repeat[..repeat_len-1].parse().expect("Fail to parse repeat"))) },
            "w" => { Some(RepeatSpan::Weeks(repeat[..repeat_len-1].parse().expect("Fail to parse repeat"))) },
            "d" => { Some(RepeatSpan::Days(repeat[..repeat_len-1].parse().expect("Fail to parse repeat"))) },
            _ => { panic!("Fail to parse repeat."); }
        }
    }
}