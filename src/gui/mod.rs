extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::{NwgUi, NwgPartial};
use nwg::NativeUi;

use std::cell::RefCell;
use std::env;
use std::fs::{self, File};
use std::io::Read;

use crate::elf;

mod nav_panel;
mod elf_header;
mod pheaders;
mod sheaders;

// GRID, FULL_ROW_SELECT
const EX_FLAGS: nwg::ListViewExFlags = nwg::ListViewExFlags::from_bits_truncate(nwg::ListViewExFlags::GRID.bits() | nwg::ListViewExFlags::FULL_ROW_SELECT.bits());

#[derive(Default, NwgUi)]
pub struct ElfExplorer {
    #[nwg_resource]
    embed: nwg::EmbedResource,

    #[nwg_control(size: (800, 620), position: (200, 200), title: "ELF Explorer", accept_files: true)]
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

    // Field description view
    #[nwg_control(position: (0, 580), size: (800, 20), flags: "NONE")]
    field_desc_frame: nwg::Frame,

    #[nwg_partial(parent: field_desc_frame)]
    field_desc: FieldDesc,

    // Navigation panel
    #[nwg_control(position: (0, 0), size: (200, 580), flags: "NONE")]
    nav_panel_frame: nwg::Frame,

    #[nwg_layout(parent: nav_panel_frame)]
    nav_panel_layout: nwg::DynLayout,

    #[nwg_control(parent: nav_panel_frame, position: (0, 0), size: (200, 580))]
    #[nwg_events(OnTreeItemSelectionChanged: [ElfExplorer::nav_panel_select_event])]
    nav_panel_tree: nwg::TreeView,

    // ELF header view
    #[nwg_control(position: (200, 0), size: (600, 580), flags: "NONE")]
    elf_header_frame: nwg::Frame,
    
    #[nwg_layout(parent: elf_header_frame)]
    elf_header_layout: nwg::DynLayout,

    #[nwg_control(parent: elf_header_frame, position: (0, 0), size: (600, 348), item_count: 1, list_style: ListViewStyle::Detailed, flags: "VISIBLE | SINGLE_SELECTION | ALWAYS_SHOW_SELECTION", ex_flags: EX_FLAGS)]
    #[nwg_events(OnListViewItemChanged: [ElfExplorer::elf_header_select_event])]
    elf_header_list: nwg::ListView,

    // e_ident view
    #[nwg_control(parent: elf_header_frame, position: (0, 348), size: (600, 232), flags: "NONE")]
    e_ident_frame: nwg::Frame,
    
    #[nwg_layout(parent: e_ident_frame)]
    e_ident_layout: nwg::DynLayout,

    #[nwg_control(parent: e_ident_frame, position: (0, 0), size: (600, 232), item_count: 1, list_style: ListViewStyle::Detailed, flags: "VISIBLE | SINGLE_SELECTION | ALWAYS_SHOW_SELECTION",  ex_flags: EX_FLAGS)]
    #[nwg_events(OnListViewItemChanged: [ElfExplorer::e_ident_select_event])]
    e_ident_list: nwg::ListView,

    // Program header table view
    #[nwg_control(position: (200, 0), size: (600, 580), flags: "NONE")]
    pheaders_frame: nwg::Frame,
    
    #[nwg_layout(parent: pheaders_frame)]
    pheaders_layout: nwg::DynLayout,

    #[nwg_control(parent: pheaders_frame, position: (0, 0), size: (600, 348), item_count: 1, list_style: ListViewStyle::Detailed, flags: "VISIBLE | SINGLE_SELECTION | ALWAYS_SHOW_SELECTION", ex_flags: EX_FLAGS)]
    #[nwg_events(OnListViewItemChanged: [ElfExplorer::pheaders_select_event])]
    pheaders_list: nwg::ListView,

    // phdr view
    #[nwg_control(parent: pheaders_frame, position: (0, 348), size: (600, 232), flags: "NONE")]
    phdr_frame: nwg::Frame,
    
    #[nwg_layout(parent: phdr_frame)]
    phdr_layout: nwg::DynLayout,

    #[nwg_control(parent: phdr_frame, position: (0, 0), size: (600, 232), item_count: 1, list_style: ListViewStyle::Detailed, flags: "VISIBLE | SINGLE_SELECTION | ALWAYS_SHOW_SELECTION",  ex_flags: EX_FLAGS)]
    #[nwg_events(OnListViewItemChanged: [ElfExplorer::phdr_select_event])]
    phdr_list: nwg::ListView,

    // Section header table view
    #[nwg_control(position: (200, 0), size: (600, 580), flags: "NONE")]
    sheaders_frame: nwg::Frame,

    #[nwg_layout(parent: sheaders_frame)]
    sheaders_layout: nwg::DynLayout,

    #[nwg_control(parent: sheaders_frame, position: (0, 0), size: (600, 348), item_count: 1, list_style: ListViewStyle::Detailed, flags: "VISIBLE | SINGLE_SELECTION | ALWAYS_SHOW_SELECTION", ex_flags: EX_FLAGS)]
    #[nwg_events(OnListViewItemChanged: [ElfExplorer::sheaders_select_event])]
    sheaders_list: nwg::ListView,

    // shdr view
    #[nwg_control(parent: sheaders_frame, position: (0, 348), size: (600, 232), flags: "NONE")]
    shdr_frame: nwg::Frame,

    #[nwg_layout(parent: shdr_frame)]
    shdr_layout: nwg::DynLayout,

    #[nwg_control(parent: shdr_frame, position: (0, 0), size: (600, 232), item_count: 1, list_style: ListViewStyle::Detailed, flags: "VISIBLE | SINGLE_SELECTION | ALWAYS_SHOW_SELECTION", ex_flags: EX_FLAGS)]
    #[nwg_events(OnListViewItemChanged: [ElfExplorer::shdr_select_event])]
    shdr_list: nwg::ListView
}

impl ElfExplorer {
    fn init(&self) {
        self.window.set_icon(self.embed.icon_str("MAINICON", None).as_ref());
        
        self.nav_panel_frame.set_visible(false);
        self.elf_header_frame.set_visible(false);
        self.pheaders_frame.set_visible(false);
        self.sheaders_frame.set_visible(false);

        self.field_desc_frame.set_visible(true);

        self.field_desc.set("Open a file using the top menu, or by dragging it into the window");

        self.main_layout.add_child((0, 0), (0, 100), &self.nav_panel_frame);
        self.main_layout.add_child((0, 100), (100, 0), &self.field_desc_frame);
        self.main_layout.add_child((0, 0), (100, 100), &self.elf_header_frame);
        self.main_layout.add_child((0, 0), (100, 100), &self.pheaders_frame);
        self.main_layout.add_child((0, 0), (100, 100), &self.sheaders_frame);

        self.nav_panel_init();
        self.field_desc.init(&self.field_desc_frame);
        self.elf_header_init();
        self.pheaders_init();
        self.sheaders_init();
    }

    fn init_elf_view(&self) {
        let elf = self.elf.borrow();
        let elf = elf.as_ref().unwrap();

        self.nav_panel_init_items(elf);

        if let Some(root) = self.nav_panel_tree.root() {
            self.nav_panel_tree.select_item(&root);
        }

        self.set_all_frames_invisible();
        self.elf_header_frame.set_visible(true);
        self.nav_panel_frame.set_visible(true);

        self.elf_header_reset();
        self.elf_header_populate(elf);
        self.elf_header_frame.set_visible(true);

        self.field_desc.set("Select any item to display a brief explanation");
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
        let elf = match elf::Elf::from(contents) {
            Ok(val) => val,
            Err(err) => match err {
                elf::ParsingError::InvalidByteOrder(msg) |
                elf::ParsingError::InvalidNativeSize(msg) => {
                    nwg::modal_error_message(
                        &self.window,
                        "Error parsing file",
                        &msg);
                    return;
                }
            },
        };
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


#[derive(Default, NwgPartial)]
pub struct FieldDesc {
    #[nwg_layout]
    layout: nwg::DynLayout,

    #[nwg_control(position: (0, 0), size: (800, 29), readonly: true, flags: "VISIBLE | DISABLED")]
    description: nwg::TextBox
}

impl FieldDesc {
    fn init(&self, frame: &nwg::Frame) {
        self.layout.parent(frame);
        self.layout.add_child((0, 0), (100, 100), &self.description);

        let mut font = nwg::Font::default();

        nwg::Font::builder()
            .family("MS Shell Dlg")
            .size(16)
            .build(&mut font)
            .expect("Failed to build font");
        
        self.description.set_font(Some(&font));
    }

    fn set(&self, text: &str) {
        self.description.set_text(text);
    }
}