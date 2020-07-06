use web_view::*;
use std::fs;
use std::fs::{ReadDir, File};

fn main() {
    let html_content = "<html><head><script type='text/javascript'>external.invoke('%PACKDIR%C:/Users/Yomama/Packs')</script><script src='https://ajax.googleapis.com/ajax/libs/jquery/2.1.1/jquery.min.js'></script></head><body><input type='file' id='selector-input'></body></html>";
    let mut prefs = File::create("./prefs.conf").unwrap();
    web_view::builder()
        .title("sampsort")
        .content(Content::Html(html_content))
        .size(320, 480)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| {

            let tmp = (&_arg[0..=8], &_arg[9.._arg.len()] );
            println!("arg: {}|{}", tmp.0, tmp.1);

            if "%PACKDIR%" == tmp.0 {
                //let paths = fs::read_dir(tmp.1).unwrap();
            }

            Ok(())
        })
        .run()
        .unwrap();
}