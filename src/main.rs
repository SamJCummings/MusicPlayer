use cursive;
use cursive::views::{Dialog, LinearLayout, ScrollView, SelectView};

use std::path::{Path, PathBuf};
use std::{env, fs, io};

const MUSIC_FOLDER: &str = "documents/music";

fn main() -> io::Result<()> {
    let mut app = create_app();

    let mut list = SelectView::new();
    list.add_all_str(load_files(None)?);

    let layout =
        LinearLayout::horizontal().child(Dialog::around(ScrollView::new(list)).title("Artists"));
    app.add_layer(layout);

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

fn create_app() -> cursive::CursiveRunnable {
    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());
    siv.set_theme(cursive::theme::Theme::terminal_default());

    siv
}
