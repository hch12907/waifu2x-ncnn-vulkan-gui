#![windows_subsystem = "windows"]

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;

#[derive(Default, NwgUi)]
pub struct Waifu2xApp {
    #[nwg_control(size: (300, 115), title: "waifu2x-ncnn-vulkan")]
    #[nwg_events( OnWindowClose: [Waifu2xApp::on_quit] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 3)]
    grid: nwg::GridLayout,

    #[nwg_control(text: "Input path:")]
    #[nwg_layout_item(layout: grid, row: 0, col: 0, col_span: 3)]
    input_label: nwg::Label,

    #[nwg_control(text: "")]
    #[nwg_layout_item(layout: grid, row: 0, col: 3, col_span: 6)]
    input_path: nwg::TextInput,

    #[nwg_control(text: "...")]
    #[nwg_layout_item(layout: grid, row: 0, col: 9)]
    input_button: nwg::Button,

    #[nwg_control(text: "Output path:")]
    #[nwg_layout_item(layout: grid, row: 1, col: 0, col_span: 3)]
    output_label: nwg::Label,

    #[nwg_control(text: "")]
    #[nwg_layout_item(layout: grid, row: 1, col: 3, col_span: 6)]
    output_path: nwg::TextInput,

    #[nwg_control(text: "...")]
    #[nwg_layout_item(layout: grid, row: 1, col: 9)]
    output_button: nwg::Button,

    #[nwg_control(text: "Say my name")]
    #[nwg_layout_item(layout: grid, col: 0, row: 2, row_span: 2, col_span: 5)]
    #[nwg_events( OnButtonClick: [Waifu2xApp::say_hello] )]
    hello_button: nwg::Button
}

impl Waifu2xApp {

    fn say_hello(&self) {
        nwg::modal_info_message(&self.window, "Hello", &format!("Hello {}", self.input_path.text()));
    }
    
    fn on_quit(&self) {
        nwg::modal_info_message(&self.window, "Goodbye", &format!("Goodbye {}", self.output_path.text()));
        nwg::stop_thread_dispatch();
    }

}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _app = Waifu2xApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
