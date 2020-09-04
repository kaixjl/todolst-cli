use todolst::components::{ todolst::* };
use futures::executor::block_on;

pub fn load_todolst() -> TodoLst {
    let system = block_on(TodoLst::load());
    system
}

pub fn save_todolst(system: &TodoLst) {
    block_on(system.save()).expect("save failed.");
}