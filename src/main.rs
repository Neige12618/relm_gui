use relm4::*;
mod app;

use app::appmodel::{AppModel, AppMode};

fn main() {
    let relm = RelmApp::new("relm4.test.components");
    relm.run::<AppModel>(AppMode::Edit);
}