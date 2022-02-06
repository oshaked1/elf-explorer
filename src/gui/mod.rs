extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgUi;
use nwg::NativeUi;

use std::cell::RefCell;
use std::env;
use std::fs::{self, File};
use std::io::Read;

use crate::elf;

mod nav_panel;
use nav_panel::*;
mod elf_header;
use elf_header::*;

#[derive(Default, NwgUi)]
pub struct ElfExplorer {
    #[nwg_resource]
    embed: nwg::EmbedResource,

    #[nwg_control(size: (800, 600), position: (200, 200), title: "ELF Explorer", accept_files: true)]
    #[nwg_events(OnInit: [ElfExplorer::init], OnResize: [ElfExplorer::size], OnWindowMaximize: [ElfExplorer::size], OnWindowClose: [ElfExplorer::exit], OnFileDrop: [ElfExplorer::drop_file(SELF, EVT_DATA)])]
    window: nwg::Window,

    #[nwg_layout(parent: window)]
    main_layout: nwg::DynLayout,

    // Menus
    #[nwg_resource(title: "Open File", action: nwg::FileDialogAction::Open)]
    dialog: nwg::FileDialog,

    #[nwg_control(parent: window, text: "File")]
    file_menu: nwg::Menu,

    #[nwg_control(parent: file_menu, text: "Open")]
    #[nwg_events(OnMenuItemSelected: [ElfExplorer::file_selection])]
    file_open: nwg::MenuItem,

    // File data
    file_name: RefCell<String>,
    elf: RefCell<Option<elf::Elf>>,

    // Navigation panel
    #[nwg_control(position: (0, 0), size: (200, 580), flags: "NONE")]
    nav_panel_frame: nwg::Frame,

    #[nwg_partial(parent: nav_panel_frame)]
    nav_panel: NavPanel,

    // ELF header view
    #[nwg_control(position: (200, 0), size: (600, 580), flags: "NONE")]
    elf_header_frame: nwg::Frame,
    
    #[nwg_partial(parent: elf_header_frame)]
    elf_header_view: ElfHeaderView,
}

impl ElfExplorer {
    fn init(&self) {
        self.window.set_icon(self.embed.icon_str("MAINICON", None).as_ref());
        
        self.nav_panel_frame.set_visible(false);
        self.elf_header_frame.set_visible(false);

        self.main_layout.add_child((0, 0), (0, 100), &self.nav_panel_frame);
        self.main_layout.add_child((0, 0), (100, 100), &self.elf_header_frame);

        self.nav_panel.init(&self.nav_panel_frame);
        self.elf_header_view.init(&self.elf_header_frame);
    }

    fn init_elf_view(&self) {
        self.nav_panel_frame.set_visible(true);
        self.nav_panel.select(0);

        self.elf_header_view.reset();
        let elf = self.elf.borrow();
        self.elf_header_view.populate(&elf.as_ref().unwrap());
        self.elf_header_frame.set_visible(true);
    }

    fn size(&self) {
        self.main_layout.fit();
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

    fn drop_file(&self, data: &nwg::EventData) {
        let files = data.on_file_drop().files();
        if files.len() != 1 {
            nwg::modal_error_message(
                &self.window,
                "Error Loading File",
                "Please drag a single file.",
            );
            return;
        }
        let file = files[0].to_owned();
        self.open_file(file);
    }

    fn file_selection(&self) {
        if let Ok(d) = env::current_dir() {
            if let Some(d) = d.to_str() {
                self.dialog
                    .set_default_folder(d)
                    .expect("Failed to set default folder.");
            }
        }

        if self.dialog.run(Some(&self.window)) {
            *self.file_name.borrow_mut() = String::from("");
            if let Ok(directory) = self.dialog.get_selected_item() {
                let dir = directory.into_string().unwrap();
                self.open_file(dir);
            }
        }
    }

    fn open_file(&self, filename: String) {
        let metadata = fs::metadata(&*filename).expect("Unable to read metadata");
        if metadata.is_dir() {
            nwg::modal_error_message(
                &self.window,
                "Error Loading File",
                "Please drag a single file.",
            );
            return;
        }
        
        let mut f = File::open(&*filename).expect("Cannot open file");
        let mut contents = vec![0; metadata.len() as usize];
        f.read(&mut contents).expect("Buffer overflow");
        let elf = elf::Elf::from(contents);
        *self.elf.borrow_mut() = Some(elf);

        *self.file_name.borrow_mut() = filename;
        self.set_title();
        self.init_elf_view();
    }

    fn set_title(&self) {
        let fullpath = &*self.file_name.borrow();
        let filename = fullpath.split("\\").last().unwrap();
        self.window
            .set_text(&format!("ELF Explorer - {}", filename));
    }
}

pub fn run() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let mut font = nwg::Font::default();
    nwg::Font::builder()
        .family("MS Shell Dlg")
        .size(15)
        .build(&mut font)
        .expect("Failed to build font");
    nwg::Font::set_global_default(Some(font));

    let _ui = ElfExplorer::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
