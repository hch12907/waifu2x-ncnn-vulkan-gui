//#![windows_subsystem = "windows"]

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use std::{ffi::OsString, cell::RefCell};

use nwd::NwgUi;
use nwg::{NativeUi, EventData, subclass_control, Button, ButtonFlags};
use winapi::um::winuser as win32;

#[derive(Default)]
pub struct GroupBox {
    base: nwg::Button,
}

pub struct GroupBoxBuilder<'a> {
    button_builder: nwg::ButtonBuilder<'a>,
}

impl<'a> GroupBoxBuilder<'a> {
    fn text(self, s: &'a str) -> GroupBoxBuilder<'a> {
        Self { button_builder: self.button_builder.text(s) }
    }

    pub fn parent<C: Into<nwg::ControlHandle>>(mut self, p: C) -> GroupBoxBuilder<'a> {
        self.button_builder = self.button_builder.parent(p);
        self
    }

    pub fn build(self, btn: &mut GroupBox) -> Result<(), nwg::NwgError> {
        self.button_builder.build(&mut btn.base)?;
        Ok(())
    }
}

impl GroupBox {
    fn builder<'a>() -> GroupBoxBuilder<'a> {
        let builder = Button::builder()
            .flags(unsafe {
                ButtonFlags::from_bits_unchecked(win32::WS_CHILD | win32::WS_VISIBLE | win32::WS_TABSTOP | win32::BS_NOTIFY | win32::BS_GROUPBOX) 
            })
            .position((20, 30))
            .size((100, 200))
            .enabled(true);

        GroupBoxBuilder {
            button_builder: builder,
        }
    }
}

subclass_control!(GroupBox, Button, base);

#[derive(Default, NwgUi)]
pub struct Waifu2xApp {
    #[nwg_control(size: (500, 115), title: "waifu2x-ncnn-vulkan")]
    #[nwg_events(
        OnMinMaxInfo: [Waifu2xApp::on_minmax(SELF, EVT_DATA)],
        OnWindowClose: [Waifu2xApp::on_quit]
    )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 3)]
    grid: nwg::GridLayout,

    #[nwg_control(text: "Input path:")]
    #[nwg_layout_item(layout: grid, row: 0, col: 0, col_span: 2)]
    input_label: nwg::Label,

    #[nwg_control(text: "")]
    #[nwg_layout_item(layout: grid, row: 0, col: 2, col_span: 7)]
    input_path: nwg::TextInput,

    #[nwg_control(text: "...")]
    #[nwg_events(OnButtonClick: [Waifu2xApp::select_input_file])]
    #[nwg_layout_item(layout: grid, row: 0, col: 9)]
    input_button: nwg::Button,

    #[nwg_control(text: "Output path:")]
    #[nwg_layout_item(layout: grid, row: 1, col: 0, col_span: 2)]
    output_label: nwg::Label,

    #[nwg_control(text: "")]
    #[nwg_events(OnButtonClick: [Waifu2xApp::select_output_file])]
    #[nwg_layout_item(layout: grid, row: 1, col: 2, col_span: 7)]
    output_path: nwg::TextInput,

    #[nwg_control(text: "...")]
    #[nwg_layout_item(layout: grid, row: 1, col: 9)]
    output_button: nwg::Button,

    #[nwg_control(text: "Say my name")]
    #[nwg_layout_item(layout: grid, col: 0, row: 2, row_span: 2, col_span: 5)]
    #[nwg_events( OnButtonClick: [Waifu2xApp::say_hello] )]
    hello_button: nwg::Button,

    #[nwg_resource(
        title: "Open File",
        action: nwg::FileDialogAction::Open,
        multiselect: true,
        filters: "PNG(*.png)|JPEG(*.jpg;*.jpeg)|WebP(*.webp)"
    )]
    open_file_dialog: nwg::FileDialog,

    #[nwg_resource(
        title: "Save File",
        action: nwg::FileDialogAction::OpenDirectory
    )]
    save_file_dialog: nwg::FileDialog,

    #[nwg_control(text: "text 1 2 3")]
    frame: GroupBox,

    state: RefCell<Waifu2xState>,
}

pub struct Waifu2xState {
    selected_files: Vec<OsString>,
}

impl Default for Waifu2xState {
    fn default() -> Self {
        Self {
            selected_files: Vec::new()
        }
    }
}

impl Waifu2xApp {
    fn on_minmax(&self, data: &EventData) {
        data.on_min_max().set_min_size(450, 150);
    }

    fn on_quit(&self) {
        nwg::modal_info_message(&self.window, "Goodbye", &format!("Goodbye {}", self.output_path.text()));
        nwg::stop_thread_dispatch();
    }

    fn select_input_file(&self) {
        if self.open_file_dialog.run(Some(&self.window)) {
            self.input_path.set_text("");
            if let Ok(paths) = self.open_file_dialog.get_selected_items() {
                let viewable_paths = paths.iter()
                    .take(10)
                    .map(|p| p.to_string_lossy().into_owned())
                    .collect::<Vec<_>>()
                    .join(";");

                self.state.borrow_mut().selected_files = paths.clone();
                self.input_path.set_text(&viewable_paths);
            }
        }
    }

    fn say_hello(&self) {
        nwg::modal_info_message(&self.window, "Selected files", &format!("{:#?}", self.state.borrow().selected_files));
    }

    fn select_output_file(&self) {
        
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _app = Waifu2xApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
