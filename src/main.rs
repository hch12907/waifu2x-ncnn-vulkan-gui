#![windows_subsystem = "windows"]

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use std::cell::RefCell;
use std::ffi::OsString;
use std::os::windows::prelude::{OsStrExt, OsStringExt};
use std::path::PathBuf;
use std::process::{Child, Command};

use nwd::NwgUi;
use nwg::{
    AnimationTimer, CheckBox, CheckBoxState, EventData, Font, NativeUi, RadioButton, Tab,
    TabsContainer, TextInput,
};

const WHITE: Option<[u8; 3]> = Some([255, 255, 255]);

#[derive(Clone, Debug, Default, PartialEq)]
enum Format {
    #[default]
    Png,
    Jpg,
    Webp,
}

#[derive(Default, NwgUi)]
pub struct Waifu2xApp {
    #[nwg_control(size: (700, 430), title: "waifu2x-ncnn-vulkan")]
    #[nwg_events(
        OnInit: [Waifu2xApp::on_init],
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
    #[nwg_layout_item(layout: grid, row: 0, col: 2, col_span: 8)]
    input_path: nwg::TextInput,

    #[nwg_control(text: "...")]
    #[nwg_events(OnButtonClick: [Waifu2xApp::select_input_file])]
    #[nwg_layout_item(layout: grid, row: 0, col: 10)]
    input_button: nwg::Button,

    #[nwg_control(text: "Output path:")]
    #[nwg_layout_item(layout: grid, row: 1, col: 0, col_span: 2)]
    output_label: nwg::Label,

    #[nwg_control(text: "")]
    #[nwg_layout_item(layout: grid, row: 1, col: 2, col_span: 8)]
    output_path: nwg::TextInput,

    #[nwg_control(text: "...")]
    #[nwg_layout_item(layout: grid, row: 1, col: 10)]
    #[nwg_events(OnButtonClick: [Waifu2xApp::select_output_file])]
    output_button: nwg::Button,

    #[nwg_control(text: "Start")]
    #[nwg_layout_item(layout: grid, col: 0, row: 2, row_span: 1, col_span: 11)]
    #[nwg_events( OnButtonClick: [Waifu2xApp::start_clicked] )]
    start_button: nwg::Button,

    // `tabs` begin here
    #[nwg_control]
    #[nwg_layout_item(layout: grid, col: 0, row: 3, row_span: 9, col_span: 11)]
    tabs: TabsContainer,

    // `tabs::processing_tab` begins here
    #[nwg_control(text: "Processing")]
    processing_tab: Tab,

    #[nwg_layout(parent: processing_tab, spacing: 5, margin: [0, 0, 0, 0])]
    processing_tab_grid: nwg::GridLayout,

    #[nwg_control(text: "Denoise Level", background_color: WHITE)]
    #[nwg_layout_item(layout: grid, col: 0, row: 0, col_span: 2)]
    denoise_label: nwg::Label,

    #[nwg_control(
        text: "None",
        background_color: WHITE,
        flags: "VISIBLE|GROUP", 
        check_state: RadioButtonState::Checked
    )]
    #[nwg_layout_item(layout: grid, col: 2, row: 0, col_span: 1)]
    #[nwg_events( OnButtonClick: [Waifu2xApp::denoise_clicked(SELF, CTRL)] )]
    denoise_disable: RadioButton,

    #[nwg_control(text: "Level 0", background_color: WHITE)]
    #[nwg_layout_item(layout: grid, col: 3, row: 0, col_span: 1)]
    #[nwg_events( OnButtonClick: [Waifu2xApp::denoise_clicked(SELF, CTRL)] )]
    denoise_level0: RadioButton,

    #[nwg_control(text: "Level 1", background_color: WHITE)]
    #[nwg_layout_item(layout: grid, col: 4, row: 0, col_span: 1)]
    #[nwg_events( OnButtonClick: [Waifu2xApp::denoise_clicked(SELF, CTRL)] )]
    denoise_level1: RadioButton,

    #[nwg_control(text: "Level 2", background_color: WHITE)]
    #[nwg_layout_item(layout: grid, col: 5, row: 0, col_span: 1)]
    #[nwg_events( OnButtonClick: [Waifu2xApp::denoise_clicked(SELF, CTRL)] )]
    denoise_level2: RadioButton,

    #[nwg_control(text: "Level 3", background_color: WHITE)]
    #[nwg_layout_item(layout: grid, col: 6, row: 0, col_span: 1)]
    #[nwg_events( OnButtonClick: [Waifu2xApp::denoise_clicked(SELF, CTRL)] )]
    denoise_level3: RadioButton,

    #[nwg_control(text: "Upscale Ratio", background_color: WHITE)]
    #[nwg_layout_item(layout: grid, col: 0, row: 1, col_span: 2)]
    upscale_label: nwg::Label,

    #[nwg_control(
        text: "1x",
        background_color: WHITE,
        flags: "VISIBLE|GROUP", 
        check_state: RadioButtonState::Checked
    )]
    #[nwg_layout_item(layout: grid, col: 2, row: 1, col_span: 1)]
    #[nwg_events(OnButtonClick: [Waifu2xApp::upscale_clicked(SELF, CTRL)])]
    upscale_level1: RadioButton,

    #[nwg_control(text: "2x", background_color: WHITE)]
    #[nwg_layout_item(layout: grid, col: 3, row: 1, col_span: 1)]
    #[nwg_events(OnButtonClick: [Waifu2xApp::upscale_clicked(SELF, CTRL)])]
    upscale_level2: RadioButton,

    #[nwg_control(text: "4x", background_color: WHITE)]
    #[nwg_layout_item(layout: grid, col: 4, row: 1, col_span: 1)]
    #[nwg_events(OnButtonClick: [Waifu2xApp::upscale_clicked(SELF, CTRL)])]
    upscale_level4: RadioButton,

    #[nwg_control(text: "8x", background_color: WHITE)]
    #[nwg_layout_item(layout: grid, col: 5, row: 1, col_span: 1)]
    #[nwg_events(OnButtonClick: [Waifu2xApp::upscale_clicked(SELF, CTRL)])]
    upscale_level8: RadioButton,

    #[nwg_control(text: "16x", background_color: WHITE)]
    #[nwg_layout_item(layout: grid, col: 6, row: 1, col_span: 1)]
    #[nwg_events(OnButtonClick: [Waifu2xApp::upscale_clicked(SELF, CTRL)])]
    upscale_level16: RadioButton,

    #[nwg_control(text: "32x", background_color: WHITE)]
    #[nwg_layout_item(layout: grid, col: 7, row: 1, col_span: 1)]
    #[nwg_events(OnButtonClick: [Waifu2xApp::upscale_clicked(SELF, CTRL)])]
    upscale_level32: RadioButton,

    #[nwg_control(text: "Enable TTA Mode (performance intensive)", background_color: WHITE)]
    #[nwg_layout_item(layout: grid, col: 0, row: 2, col_span: 5)]
    #[nwg_events(OnButtonClick: [Waifu2xApp::tta_mode_clicked])]
    tta_mode: CheckBox,

    #[nwg_control(text: "Advanced Options", background_color: WHITE)]
    #[nwg_layout_item(layout: grid, col: 0, row: 3, col_span: 5)]
    #[nwg_events(OnButtonClick: [Waifu2xApp::advanced_options_clicked])]
    advanced_options: CheckBox,

    #[nwg_control(text: "Thread Count", background_color: WHITE)]
    #[nwg_layout_item(layout: grid, col: 0, row: 4, col_span: 2)]
    thread_label: nwg::Label,

    #[nwg_control(text: "1:2:2", background_color: WHITE, readonly: true)]
    #[nwg_layout_item(layout: grid, col: 2, row: 4, col_span: 8)]
    #[nwg_events(OnTextInput: [Waifu2xApp::thread_count_changed])]
    thread_count: TextInput,

    #[nwg_control(text: "GPU ID", background_color: WHITE)]
    #[nwg_layout_item(layout: grid, col: 0, row: 5, col_span: 2)]
    gpu_id_label: nwg::Label,

    #[nwg_control(text: "auto", background_color: WHITE, readonly: true)]
    #[nwg_layout_item(layout: grid, col: 2, row: 5, col_span: 8)]
    #[nwg_events(OnTextInput: [Waifu2xApp::gpu_id_changed])]
    gpu_id: TextInput,

    #[nwg_control(text: "Waifu2x Model", background_color: WHITE)]
    #[nwg_layout_item(layout: grid, col: 0, row: 6, col_span: 2)]
    model_label: nwg::Label,

    #[nwg_control(text: "models-cunet", background_color: WHITE, readonly: true)]
    #[nwg_layout_item(layout: grid, col: 2, row: 6, col_span: 8)]
    #[nwg_events(OnTextInput: [Waifu2xApp::model_path_changed])]
    model_path: TextInput,

    // `tabs::processing_tab` ends here
    // `tabs::output_tab` begins here
    #[nwg_control(parent: tabs, text: "Output")]
    output_tab: Tab,

    #[nwg_control(text: "Output Format", background_color: WHITE)]
    #[nwg_layout_item(layout: grid, col: 0, row: 0, col_span: 2)]
    format_label: nwg::Label,

    #[nwg_control(
        text: "PNG",
        background_color: WHITE,
        flags: "VISIBLE|GROUP", 
        check_state: RadioButtonState::Checked
    )]
    #[nwg_layout_item(layout: grid, col: 2, row: 0, col_span: 1)]
    #[nwg_events(OnButtonClick: [Waifu2xApp::format_clicked(SELF, CTRL)])]
    format_png: RadioButton,

    #[nwg_control(text: "JPG", background_color: WHITE)]
    #[nwg_layout_item(layout: grid, col: 3, row: 0, col_span: 1)]
    #[nwg_events(OnButtonClick: [Waifu2xApp::format_clicked(SELF, CTRL)])]
    format_jpg: RadioButton,

    #[nwg_control(text: "WebP", background_color: WHITE)]
    #[nwg_layout_item(layout: grid, col: 4, row: 0, col_span: 1)]
    #[nwg_events(OnButtonClick: [Waifu2xApp::format_clicked(SELF, CTRL)])]
    format_webp: RadioButton,

    #[nwg_control(text: "Output Name", background_color: WHITE)]
    #[nwg_layout_item(layout: grid, col: 0, row: 1, col_span: 2)]
    filename_label: nwg::Label,

    #[nwg_control(text: "{name}_{scale}x_{denoise}n", background_color: WHITE)]
    #[nwg_layout_item(layout: grid, col: 2, row: 1, col_span: 8)]
    #[nwg_events(OnTextInput: [Waifu2xApp::filename_changed])]
    filename_format: TextInput,

    #[nwg_control(text: "{name}, {scale}, {denoise}, {model} will be replaced with specific values.\nAn extension will be automatically appended to the filename.", background_color: WHITE)]
    #[nwg_layout_item(layout: grid, col: 0, row: 2, col_span: 10, row_span: 2)]
    filename_advice_label: nwg::Label,

    // `tabs::output_tab` ends here
    #[nwg_resource(
        title: "Open File",
        action: nwg::FileDialogAction::Open,
        multiselect: true,
        filters: "PNG(*.png)|JPEG(*.jpg;*.jpeg)|WebP(*.webp)|>Supported image files(*.png;*.jpg;*.jpeg;*.webp)"
    )]
    open_file_dialog: nwg::FileDialog,

    #[nwg_resource(
        title: "Save File",
        action: nwg::FileDialogAction::OpenDirectory
    )]
    save_file_dialog: nwg::FileDialog,

    #[nwg_control(parent: window, interval: std::time::Duration::from_millis(100))]
    #[nwg_events(OnTimerTick: [Waifu2xApp::timer_ticked])]
    timer: AnimationTimer,

    #[nwg_resource(family: "Segoe UI", size: 16)]
    advice_font: Font,

    state: RefCell<Waifu2xState>,
}

pub struct Waifu2xState {
    selected_files: Vec<OsString>,
    output_dir: OsString,
    scale_level: i32,
    denoise_level: i32,
    tta_mode: bool,
    format: Format,
    thread_count: String,
    gpu_id: String,
    model_path: String,
    filename_format: String,
    children: Vec<Child>,
}

impl Default for Waifu2xState {
    fn default() -> Self {
        Self {
            selected_files: Vec::new(),
            output_dir: OsString::new(),
            scale_level: 1,
            denoise_level: -1,
            tta_mode: false,
            format: Format::Png,
            thread_count: String::new(),
            gpu_id: String::new(),
            model_path: String::new(),
            filename_format: String::from("{name}_{scale}x_{denoise}n"),
            children: Vec::new(),
        }
    }
}

impl Waifu2xState {
    fn set_denoise_level(&mut self, level: i32) {
        self.denoise_level = level;
    }

    fn set_scale_level(&mut self, level: i32) {
        self.scale_level = level;
    }
}

impl Waifu2xApp {
    fn on_init(&self) {
        self.filename_advice_label.set_font(Some(&self.advice_font));
    }

    fn on_minmax(&self, data: &EventData) {
        data.on_min_max().set_min_size(700, 450);
    }

    fn on_quit(&self) {
        nwg::stop_thread_dispatch();
    }

    fn select_input_file(&self) {
        if self.open_file_dialog.run(Some(&self.window)) {
            self.input_path.set_text("");
            if let Ok(paths) = self.open_file_dialog.get_selected_items() {
                let viewable_paths = paths
                    .iter()
                    .take(10)
                    .map(|p| p.to_string_lossy().into_owned())
                    .collect::<Vec<_>>()
                    .join(";");

                self.state.borrow_mut().selected_files = paths.clone();
                self.input_path.set_text(&viewable_paths);
            }
        }
    }

    fn denoise_clicked(&self, control: &RadioButton) {
        let level = *control.text().as_bytes().last().unwrap();

        if level == b'e' {
            // "None"
            self.state.borrow_mut().set_denoise_level(-1);
        } else {
            self.state
                .borrow_mut()
                .set_denoise_level((level - b'0') as i32);
        }
    }

    fn format_clicked(&self, control: &RadioButton) {
        let format = match control.text().as_ref() {
            "PNG" => Format::Png,
            "JPG" => Format::Jpg,
            "WebP" => Format::Webp,
            _ => unreachable!("invalid format detected"),
        };

        self.state.borrow_mut().format = format;
    }

    fn upscale_clicked(&self, control: &RadioButton) {
        let text = control.text();
        let level = text.trim_end_matches('x').parse::<i32>().unwrap();
        self.state.borrow_mut().set_scale_level(level);
    }

    fn tta_mode_clicked(&self) {
        self.state.borrow_mut().tta_mode = self.tta_mode.check_state() == CheckBoxState::Checked;
    }

    fn advanced_options_clicked(&self) {
        let advanced = self.advanced_options.check_state() == CheckBoxState::Checked;

        self.gpu_id.set_readonly(!advanced);
        self.thread_count.set_readonly(!advanced);
        self.model_path.set_readonly(!advanced);
    }

    fn select_output_file(&self) {
        if self.save_file_dialog.run(Some(&self.window)) {
            self.output_path.set_text("");
            if let Ok(path) = self.save_file_dialog.get_selected_item() {
                self.state.borrow_mut().output_dir = path.clone();
                self.output_path.set_text(path.to_string_lossy().as_ref());
            }
        }
    }

    fn thread_count_changed(&self) {
        self.state.borrow_mut().thread_count = self.thread_count.text();
    }

    fn gpu_id_changed(&self) {
        self.state.borrow_mut().gpu_id = self.gpu_id.text();
    }

    fn model_path_changed(&self) {
        self.state.borrow_mut().model_path = self.model_path.text();
    }

    fn filename_changed(&self) {
        self.state.borrow_mut().filename_format = self.filename_format.text();
    }

    fn timer_ticked(&self) {
        let mut state = self.state.borrow_mut();

        let mut i = 0;
        while i < state.children.len() {
            let c = &mut state.children[i];

            let should_remove = match c.try_wait() {
                Ok(None) => false,
                Ok(Some(status)) => {
                    if !status.success() {
                        self.start_button.set_text("Processing... (error occured!)");
                        true
                    } else {
                        true
                    }
                }
                Err(e) => {
                    nwg::error_message(
                        "Error",
                        &format!("Unexpected error occured while running Waifu2x: {}", e),
                    );
                    true
                }
            };

            if should_remove {
                state.children.remove(i);
            } else {
                i += 1;
            }
        }

        if state.children.is_empty() {
            self.timer.stop();
            if self.start_button.text().contains("error") {
                self.start_button.set_text("Start (error occured!)");
            } else {
                self.start_button.set_text("Start")
            }
            self.start_button.set_enabled(true);
        }
    }

    fn start_clicked(&self) {
        let state = self.state.borrow();
        let mut children = Vec::new();

        for f in state.selected_files.iter() {
            let mut input = PathBuf::from(f);
            let mut output = PathBuf::from(state.output_dir.clone());
            let output_ext = match state.format {
                Format::Png => "png",
                Format::Jpg => "jpg",
                Format::Webp => "webp",
            };

            input.set_extension("");

            output.push(match input.file_name() {
                Some(x) => {
                    let template = state
                        .filename_format
                        .replace("{scale}", &format!("{}", state.scale_level))
                        .replace("{denoise}", &format!("{}", state.denoise_level))
                        .replace("{model}", &state.model_path);

                    let name_start = match template.find("{name}") {
                        Some(x) => x,
                        None => {
                            nwg::error_message(
                                "Error",
                                "Output filename must contain a {name} section!",
                            );
                            return;
                        }
                    };

                    let name_end = name_start + "{name}".len();

                    let encoded = template
                        .encode_utf16()
                        .take(name_start)
                        .chain(OsStrExt::encode_wide(x))
                        .chain(template.encode_utf16().skip(name_end))
                        .collect::<Vec<_>>();

                    OsString::from_wide(&encoded)
                }
                None => {
                    nwg::error_message(
                        "Error",
                        &format!(
                            "The following input file has an invalid path: {}",
                            f.to_string_lossy()
                        ),
                    );
                    return;
                }
            });

            output.set_extension(output_ext);

            let mut waifu2x_command = Command::new("waifu2x-ncnn-vulkan-cli");
            let mut waifu2x = waifu2x_command
                .arg("-i")
                .arg(f)
                .arg("-o")
                .arg(output)
                .arg("-s")
                .arg(state.scale_level.to_string())
                .arg("-n")
                .arg(state.denoise_level.to_string());

            if !state.thread_count.is_empty() {
                waifu2x = waifu2x.arg("-j").arg(&state.thread_count);
            }

            if !state.gpu_id.is_empty() {
                waifu2x = waifu2x.arg("-g").arg(&state.gpu_id);
            }

            if !state.model_path.is_empty() {
                waifu2x = waifu2x.arg("-m").arg(&state.model_path)
            }

            let child = match waifu2x.spawn() {
                Ok(x) => x,
                Err(e) => {
                    nwg::error_message(
                        "Error",
                        &format!("Unable to spawn a waifu2x instance:\n{:?}", e),
                    );
                    return;
                }
            };

            children.push(child);
        }

        drop(state);

        self.start_button.set_text("Processing...");
        self.start_button.set_enabled(false);
        self.timer.start();
        self.state.borrow_mut().children = children;
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _app = Waifu2xApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
