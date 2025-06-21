#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;
use rocket::response::content::RawHtml;
use std::fs;
use std::fs::metadata;

#[get("/")]
pub fn index() -> RawHtml<String> {
    let dir_string = get_file_structure(&"/media".to_string(), &"".to_string(), true);
    RawHtml(dir_string)
}
#[allow(dead_code)]
fn get_inside_files<P: AsRef<std::path::Path>>(file: P) -> String {
    fs::read_dir(file).unwrap().fold("".to_string(), |acc, v| {
        format!(
            "{}{}\n",
            acc,
            if v.as_ref().unwrap().metadata().unwrap().is_dir() {
                get_inside_files(v.unwrap().path())
            } else {
                v.unwrap().path().display().to_string()
            }
        )
    })
}
pub fn get_file_structure(location: &String, start_string: &String, last: bool) -> String {
    let mut output = format!(
        "<pre style=\"margin:0;\">{}L <a href=\"http://mari-rzepka.net:8000{}\">{}</a></pre>",
        start_string,
        location,
        location.split('/').collect::<Vec<&str>>().last().unwrap()
    );

    let md = metadata(location).unwrap();
    if md.is_dir() {
        let mut paths = fs::read_dir(location).unwrap();
        let last_path = paths.next();
        let mut new_char = "|	";
        if last {
            new_char = "    ";
        }
        for path in paths {
            output += &get_file_structure(
                &path.unwrap().path().display().to_string(),
                &(start_string.clone() + new_char),
                false,
            );
        }

        output += &get_file_structure(
            &last_path.unwrap().unwrap().path().display().to_string(),
            &(start_string.clone() + new_char),
            true,
        );
    }

    output
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/media", FileServer::from("/media"))
}
mod testlib;
