#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;
use rocket::http::Method;
use rocket::response::content::RawHtml;
use rocket_cors::{AllowedOrigins, CorsOptions};
use std::fs;
use std::fs::metadata;
use std::path::PathBuf;

#[get("/")]
pub fn index() -> RawHtml<String> {
    let dir_string = get_file_structure("/media".to_string(), "".to_string(), true);
    RawHtml(dir_string)
}
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
pub fn get_file_structure(location: String, start_string: String, last: bool) -> String {
    if '.'
        == location
            .split('/')
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .chars()
            .nth(0)
            .unwrap()
    {
        return "".into();
    }

    println!("location: {:?}", location);
    let mut output: String = if location
        .split('/')
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .split('.')
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        == &"mp4"
    {
        format!(
            "{}{}-<video width=75vw height=75vh controls> <source src=\"http://mari-rzepka.net:8000{}\" type=\"video/mp4\"></video>",
            start_string,
            if last { "└" } else { "├" },
            location,
        )
    } else {
        format!(
        "<pre style=\"margin:-2px;\">{}{}─<a href=\"http://mari-rzepka.net:8000{}\">{}</a></pre>",
        start_string,
        if last { "└" } else { "├" },
        location,
        location.split('/').collect::<Vec<&str>>().last().unwrap()
    )
    };
    let md = metadata(&location).unwrap();
    if md.is_dir() {
        let mut paths = fs::read_dir(location).unwrap();
        let last_path = paths.next();
        let mut new_char = "│	";
        if last {
            new_char = "    ";
        }
        output += &paths.fold(
            "".to_string(),
            |acc: String, path: Result<std::fs::DirEntry, std::io::Error>| {
                acc + &get_file_structure(
                    path.unwrap().path().display().to_string(),
                    start_string.clone() + new_char,
                    false,
                )
            },
        );

        output += &get_file_structure(
            last_path.unwrap().unwrap().path().display().to_string(),
            start_string.clone() + new_char,
            true,
        );
    }

    output
}

#[get("/video/<file..>")]
pub fn get_video_player(file: PathBuf) -> RawHtml<String> {
    return RawHtml(format!("<video height=200px controls> <source src=\"http://mari-rzepka.net:8000/{}\" type=\"video/mp4\"></video>",file.display()));
}

#[get("/browse/<location..>")]
pub fn get_pretty_directory(location: PathBuf) -> RawHtml<String> {
    println!("{:?}", location);
    let files = fs::read_dir(format!("/{}", location.display().to_string())).unwrap();

    let mut path = location.clone();
    path.pop();
    let mut output = format!(
        "<pre style=\"margin:-2px;\">/{}</pre>
        <pre style=\"margin:-2px;\">│</pre>
        <pre style=\"margin:-2px;\">├ <a href=\"http://mari-rzepka.net:8000/browse/{}\">..</a></pre>",
        location.display(),
        path.display()
    );
    for x in files {
        let location = x.unwrap().path().display().to_string();
        let md = metadata(&location).unwrap();

        if location
            .split('/')
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .chars()
            .next()
            .unwrap()
            == ".".chars().next().unwrap()
        {
            continue;
        }
        if md.is_dir() {
            output = format!(
                "{}<pre style=\"margin:-2px;\">├ <a href=\"http://mari-rzepka.net:8000/browse/{}\">{}</a></pre>",
                output,
                location,
                location.split('/').collect::<Vec<&str>>().last().unwrap()
            );
        } else {
            output = if location
                .split('/')
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .split('.')
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                == &"mp4"
            {
                println!("{}", location);
                format!(

                "{}<pre style=\"margin:-2px;\">─<a href=\"http://mari-rzepka.net:8000/video/{}\">{}</a></pre>",
                                    output,
                                    location,
        location.split('/').collect::<Vec<&str>>().last().unwrap()
                                )
            } else {
                format!(
        "{}<pre style=\"margin:-2px;\">─<a href=\"http://mari-rzepka.net:8000{}\">{}</a></pre>",
        output, location,
        location.split('/').collect::<Vec<&str>>().last().unwrap()
    )
            };
        }
    }

    RawHtml(output)
}

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![
                Method::Get,
                Method::Post,
                Method::Patch,
                Method::Delete,
                Method::Options,
            ]
            .into_iter()
            .map(From::from)
            .collect(),
        )
        .allow_credentials(true)
        .to_cors()
        .expect("Failed to create CORS fairing");

    rocket::build()
        .attach(cors)
        .mount("/", routes![index, get_pretty_directory, get_video_player])
        .mount("/media", FileServer::from("/media"))
}
mod testlib;
