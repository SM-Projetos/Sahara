mod filesystem;

slint::include_modules!();

use filesystem::{read_directory, resolve_location};
use slint::{ModelRc, VecModel};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

fn main() -> Result<(), slint::PlatformError> {
    let window = MainWindow::new()?;
    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .filter(|path| path.is_dir())
        .unwrap_or_else(|| PathBuf::from("/"));

    let history = Rc::new(RefCell::new(Vec::<PathBuf>::new()));
    let current = Rc::new(RefCell::new(home.clone()));
    show_directory(&window, &home);

    let weak_window = window.as_weak();
    let back_history = Rc::clone(&history);
    let back_current = Rc::clone(&current);
    window.on_go_back(move || {
        if let Some(previous) = back_history.borrow_mut().pop() {
            *back_current.borrow_mut() = previous.clone();
            if let Some(window) = weak_window.upgrade() {
                show_directory(&window, &previous);
            }
        }
    });

    let weak_window = window.as_weak();
    let open_history = Rc::clone(&history);
    let open_current = Rc::clone(&current);
    window.on_open_directory(move |path| {
        let path = PathBuf::from(path.to_string());
        if !path.is_dir() {
            return;
        }
        open_history.borrow_mut().push(open_current.borrow().clone());
        *open_current.borrow_mut() = path.clone();
        if let Some(window) = weak_window.upgrade() {
            show_directory(&window, &path);
        }
    });

    let weak_window = window.as_weak();
    let location_history = Rc::clone(&history);
    let location_current = Rc::clone(&current);
    let location_home = home.clone();
    window.on_open_location(move |location| {
        let Some(path) = resolve_location(&location_home, &location.to_string()) else {
            return;
        };

        location_history.borrow_mut().push(location_current.borrow().clone());
        *location_current.borrow_mut() = path.clone();
        if let Some(window) = weak_window.upgrade() {
            show_directory(&window, &path);
        }
    });

    window.run()
}

fn show_directory(window: &MainWindow, path: &std::path::Path) {
    window.set_current_path(path.display().to_string().into());
    match read_directory(path) {
        Ok(entries) => {
            let items = entries.into_iter().map(|entry| FileEntry {
                name: entry.name.into(),
                path: entry.path.display().to_string().into(),
                is_directory: entry.is_directory,
            }).collect::<Vec<_>>();
            let count = items.len();
            window.set_entries(ModelRc::new(VecModel::from(items)));
            window.set_status(format!("{count} itens").into());
        }
        Err(error) => {
            window.set_entries(ModelRc::new(VecModel::from(Vec::<FileEntry>::new())));
            window.set_status(error.into());
        }
    }
}
