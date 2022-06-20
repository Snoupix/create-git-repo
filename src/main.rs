mod hl_chrome;

fn main() {
    if std::env::args().len() < 2 {
        println!("Missing repo name arg");
        std::process::exit(1);
    }

    hl_chrome::goto_github();
}

pub fn format_input(input: String) -> String {
    input.replace("\r", "").replace("\n", "")
}
