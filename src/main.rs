use std::path::{Path, PathBuf};
use std::{env, fs, io};

use cursive::traits::*;
use cursive::views::{Dialog, LinearLayout, ResizedView, ScrollView, SelectView};
use cursive::{Cursive, CursiveRunnable};

const MUSIC_FOLDER: &str = "documents/music";

fn main() -> io::Result<()> {
    let mut app = create_app()?;
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

fn create_app() -> Result<CursiveRunnable, io::Error> {
    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());
    siv.set_theme(cursive::theme::Theme::terminal_default());

    let layout = LinearLayout::horizontal()
        .child(create_list("Artists", load_files(None)?))
        .child(create_list("Albums", vec![]))
        .child(create_list("Songs", vec![]))
        .with_name("Layout")
        .full_screen();

    siv.add_fullscreen_layer(layout);

    Ok(siv)
}

fn create_list(title: &str, contents: Vec<String>) -> ResizedView<Dialog> {
    let select = SelectView::new()
        .with_inactive_highlight(false)
        .with_all_str(contents)
        .on_submit(select_item)
        .with_name(title);

    Dialog::around(ScrollView::new(select))
        .title(title)
        .full_screen()
}

fn select_item(siv: &mut Cursive, choice: &String) {
    let focus = siv
        .call_on_name("Layout", |view: &mut LinearLayout| view.get_focus_index())
        .unwrap();

    siv.call_on_name("Albums", |view: &mut SelectView| {
        view.clear();
        view.add_all_str(load_files(Some(choice.clone())).unwrap());
        view.add_item_str(focus.to_string());
    });
}
