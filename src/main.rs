use std::path::{Path, PathBuf};
use std::{env, fs, io};

use cursive::traits::*;
use cursive::views::{Dialog, DummyView, LinearLayout, ResizedView, ScrollView, SelectView};
use cursive::{Cursive, CursiveRunnable};

const MUSIC_FOLDER: &str = "documents/music";

fn main() -> io::Result<()> {
    let mut app = create_app();

    let layout = LinearLayout::horizontal()
        .child(create_list_items("Artists", load_files(None)?))
        .child(create_list_empty("Albums"))
        .child(create_list_empty("Songs"))
        .full_screen();

    app.add_fullscreen_layer(layout);
    app.run();
    Ok(())
}

fn load_files(folder: Option<String>) -> Result<Vec<String>, io::Error> {
    let mut path = env::home_dir().unwrap();
    path.push(MUSIC_FOLDER.to_string());
    path.push(folder.unwrap_or(String::new()));

    let visible = |p: &Path| {
        p.file_name()
            .and_then(|n| n.to_str())
            .map(|name| !name.starts_with('.'))
            .unwrap()
    };

    let entries = fs::read_dir(path)?
        .filter_map(|res| res.ok())
        .map(|e| e.path())
        .filter(|p| visible(p))
        .collect::<Vec<PathBuf>>();

    let mut list = entries
        .iter()
        .filter_map(|path| path.file_name()?.to_str())
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    list.sort();
    Ok(list)
}

fn create_app() -> CursiveRunnable {
    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());
    siv.set_theme(cursive::theme::Theme::terminal_default());

    siv
}

fn create_list_empty(title: &str) -> ResizedView<Dialog> {
    Dialog::around(ScrollView::new(DummyView))
        .title(title)
        .full_screen()
}

fn create_list_items(title: &str, contents: Vec<String>) -> ResizedView<Dialog> {
    let select = SelectView::new()
        .with_inactive_highlight(false)
        .with_all_str(contents)
        .on_submit(select_item);

    Dialog::around(ScrollView::new(select))
        .title(title)
        .full_screen()
}

fn select_item(siv: &mut Cursive, choice: &String) {
    siv.quit();
}
