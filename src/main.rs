mod structures;
mod utils;
use nfd2::Response;
use std::fs::{DirEntry, File, ReadDir};
use std::io::{BufRead, ErrorKind, Write};
use std::{fs, thread};
use structures::*;
use utils::*;
use web_view::*;
#[macro_use]
use sciter::*;
use sciter::dom::event::MethodParams;
use sciter::dom::SCDOM_RESULT;
use sciter::window::Options;
use sciter::{Element, EventHandler, Value, HELEMENT};
use std::ffi::OsStr;
use std::fs::metadata;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::Thread;
use std::time::Duration;

static MANAGER_IS_OPEN: AtomicBool = AtomicBool::new(false);

struct Handler;
impl Handler {
    fn hide_these(&self, list: Element) {
        //println!("{}",list);
        list.children().for_each(|e| {
            let this = e;
            this.eval_script("hide_this(this)");
            this.refresh();
        });
    }

    fn toggle_checkbox_hide(&self, folders: Element) {
        //println!("{}",filelist);
        folders.children().for_each(|e| {
            //println!("{}",e);
            let this = e;
            this.eval_script("update_checkbox(this.$(.slctr))");
            this.refresh();
        });
    }

    fn open_manager(&self) {
        // get/set config

        if !MANAGER_IS_OPEN.load(Ordering::SeqCst) {
            MANAGER_IS_OPEN.store(true, Ordering::SeqCst);
            let cwd = std::env::current_dir().unwrap();
            let mut path = cwd.display().to_string();
            let mut cfgpath = cwd.display().to_string();
            path.push_str("\\web\\manage.html");
            cfgpath.push_str("sampsort.config");

            //init sciter

            let mut frame2 = sciter::window::Builder::main_window()
                .with_size((400i32, 400i32))
                .create();

            frame2.event_handler(ManagerHandle);
            frame2.set_title("pack folder manager");
            frame2.load_file(&*path);
            //get and enter config entries
            let dirs = get_config_dirs("sampsort.conf");
            for d in dirs {
                println!("{}", d);
                ticmd("add_dir", &make_args!(d.as_str(), d.as_str()), &frame2);
            }

            frame2.run_app();
            //manager closed
            MANAGER_IS_OPEN.store(false, Ordering::SeqCst);
        } else {
            println!("denying manager reopen.");
        }
    }

    fn populate_me(&self, pathatt: String, elem: Element) {
        for de in Path::new(pathatt.as_str()).read_dir().unwrap() {
            match de {
                Ok(e) => {
                    let filename = extract_filename(fix_pathstr(e.path().display().to_string()));
                    let mut path = fix_pathstr(e.path().display().to_string());

                    let tmp = sort_path(path.clone());
                    match tmp {
                        Pathmeta::file(filelink) => {
                            elem.call_function("add_item_tolist", &make_args!(filename, path.clone()));
                        }

                        Pathmeta::folder(folderlink) => {
                            elem.call_function("add_folder_tolist", &make_args!(filename, path.clone()));
                        }

                        _ => (),
                    };
                }
                Err(_) => {}
            }
        }
    }
}

impl EventHandler for Handler {
    dispatch_script_call! {
        fn open_manager();
        fn toggle_checkbox_hide(Element);
        fn hide_these(Element);
        fn populate_me(String,Element);
    }
}

struct ManagerHandle;
impl ManagerHandle {
    fn pick_add_dir(&self) {
        println!("opening dir...");
        match nfd2::open_pick_folder(None).expect("oh no it didnt work :////") {
            Response::Okay(path) => {}
            Response::OkayMultiple(paths) => {}
            Response::Cancel => {
                println!("user cancelled.");
            }
        };
    }

    fn remove_selected(&self, filelist: Element) {
        println!("{}", filelist);
        filelist.children().for_each(|e| {
            //println!("{}",e);
            let this = e.children().nth(0).unwrap();
            this.eval_script("update_checkbox(this)");
            this.refresh();
        });
    }
}

impl EventHandler for ManagerHandle {
    dispatch_script_call! {
        fn pick_add_dir();
    }
}

fn get_config_dirs(path: &str) -> Vec<String> {
    let mut tmp: Vec<String> = vec![];

    let conf = std::fs::File::open(Path::new(path)).unwrap();
    let conf = match std::fs::File::open(Path::new(path)) {
        Ok(f) => f,
        Err(e) => {
            println!("config not found, creating a new one.");
            std::fs::File::create(Path::new(path)).unwrap()
        }
    };

    let reader = std::io::BufReader::<File>::new(conf);
    let lines = Vec::from_iter(reader.lines());

    let mut mode = 0;
    //modes: 0=default, 1=DIR
    for line in lines {
        match line {
            Ok(l) => match l.as_str() {
                "[DIR]" => {
                    mode = 1;
                }

                _ => match mode {
                    0 => {
                        println!("{}", l);
                    }
                    1 => {
                        tmp.push(l);
                    }
                    _ => {}
                },
            },
            Err(e) => {
                eprintln!("[ERROR] {}", e);
            }
        }
    }

    tmp
}

fn ticmd(cmd: &str, args: &[Value], frame: &Window) {
    let mut root = Element::from_window(frame.get_hwnd());
    match root {
        Ok(e) => {
            e.call_function(cmd, args);
        }
        Err(e) => {
            eprintln!("[ERROR] {}", e);
        }
    }
}

fn main() -> std::io::Result<()> {
    // get/set config
    let cwd = std::env::current_dir()?;
    let mut path = cwd.display().to_string();
    path.push_str("\\web\\index.html");

    //init sciter
    let mut frame = sciter::window::Builder::main_window()
        .with_size((400i32, 600i32))
        .create();

    frame.event_handler(Handler);
    frame.set_title("sampsort");
    frame.load_file(&*path);

    let host = frame.get_host();

    let mut rootlist: Vec<Pathmeta> = vec![];

    let dirs = get_config_dirs("sampsort.conf");

    for d in dirs {
        for de in Path::new(d.as_str()).read_dir().unwrap() {
            match de {
                Ok(e) => {
                    let filename = extract_filename(fix_pathstr(e.path().display().to_string()));
                    let mut path = fix_pathstr(e.path().display().to_string());

                    let tmp = sort_path(path.clone());
                    match tmp {
                        Pathmeta::file(filelink) => {
                            println!("{}",fix_pathstr_double(path.clone()));
                            ticmd("add_item", &make_args!(filename, fix_pathstr_double(path.clone())), &frame);
                        }

                        Pathmeta::folder(folderlink) => {
                            ticmd("add_folder", &make_args!(filename, fix_pathstr_double(path.clone())), &frame);
                        }

                        _ => (),
                    };
                }
                Err(_) => {}
            }
        }
    }

    //run
    frame.run_app();

    Ok(())
}
