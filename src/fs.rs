
fn check_dir() -> anyhow::Result<String> {
    let username = whoami::username();
    let path = format!("/home/{username}/.rustdo/");
    std::fs::create_dir_all(&path)?;

    Ok(path)
}
fn check_file() -> anyhow::Result<()> {
    let username = whoami::username();
    let path = format!("/home/{username}/.rustdo/todos.json");
    std::fs::File::create_new(path)?;
    Ok(())
}

use crate::Screen;
pub fn read_todos() -> Screen {
    let path = check_dir().unwrap_or_else(|_| {
        // println!("creating dir error");
        std::process::exit(1)
    });
    let _ = check_file();
    let file = std::fs::File::open(format!("{path}todos.json")).unwrap_or_else(|_| {
        // println!("creating or opening file error");
        std::process::exit(1)
    });

    let screen_from_file: Screen = serde_json::from_reader(file).unwrap_or_else(|err| {
        println!("reading from json error with {err}");
        Screen::new(None)
    });
    screen_from_file
}
pub fn save_todos(screen: &Screen) {
    let path = check_dir().unwrap_or_else(|_| {
        // println!("existing dir error");
        std::process::exit(1)
    });
    let file = std::fs::File::create(format!("{path}todos.json")).unwrap_or_else(|_| {
        // println!("creating or opening file error");
        std::process::exit(1)
    });
    serde_json::to_writer(file, screen).unwrap_or_else(|_| {
        // println!("saving todos to file error with {err}");
        std::process::exit(1)
    });
}
