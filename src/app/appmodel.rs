use gtk::prelude::ApplicationExt;
use relm4::{*, gtk::prelude::*};
use gtk::{WrapMode, TextBuffer};

use super::header::{HeaderModel, HeaderOutput};
use super::dialog::{DialogInput, DialogOutput, DialogModel};



#[derive(Debug)]
pub enum AppMode {
    View,
    Edit,
    Export,
}

#[derive(Debug)]
pub enum AppMsg {
    SetMode(AppMode),
    CloseRequest,
    Close,
}

pub struct AppModel {
    mode: AppMode,
    buffer: TextBuffer,
    header: Controller<HeaderModel>,
    dialog: Controller<DialogModel>,
}

#[relm4::component(pub)]
impl SimpleComponent for AppModel {
    type Init = AppMode;
    type Input = AppMsg;
    type Output = ();

    view! {
        main_window = gtk::Window {
            set_default_width: 800,
            set_default_height: 600,
            set_titlebar: Some(model.header.widget()),
            set_resizable: true,
            set_margin_all: 1,

            
            gtk::TextView {
                set_buffer: Some(&model.buffer),
                set_wrap_mode: WrapMode::Char,
                set_editable: true,
            },


            connect_close_request[sender] => move |_| {
                sender.input(AppMsg::CloseRequest);
                gtk::Inhibit(true)
            }
        }
    }

    fn init(
        params: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let header: Controller<HeaderModel> =
            HeaderModel::builder()
                .launch(())
                .forward(sender.input_sender(), |msg| match msg {
                    HeaderOutput::View => AppMsg::SetMode(AppMode::View),
                    HeaderOutput::Edit => AppMsg::SetMode(AppMode::Edit),
                    HeaderOutput::Export => AppMsg::SetMode(AppMode::Export),
                });

        let dialog = DialogModel::builder()
            .transient_for(root)
            .launch(true)
            .forward(sender.input_sender(), |msg| match msg {
                DialogOutput::Close => AppMsg::Close,
            });

        let buffer = TextBuffer::builder().build();

        let model = AppModel {
            mode: params,
            buffer,
            header,
            dialog,
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::SetMode(mode) => {
                self.mode = mode;
            }
            AppMsg::CloseRequest => {
                self.dialog.sender().send(DialogInput::Show).unwrap();
            }
            AppMsg::Close => {
                relm4::main_application().quit();
            }
        }
    }
}
