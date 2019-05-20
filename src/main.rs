#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
use std::fs::File;
use std::io::prelude::*;
use actix_web::{server::HttpServer, App, Form,  http, HttpRequest, HttpResponse, Result};

#[derive(Deserialize)]
struct Params {
  host: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct StorageFile {
  hosts: Vec<String>
}

fn handle_save(info: Form<Params>) -> String {
  let mut file = File::open("/home/mateusz/sources/skype_serial_killer/hosts.json").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  let mut storage: StorageFile = serde_json::from_str(&contents).unwrap();
  
  storage.hosts.push(info.host.to_string());

  let mut file = File::create("/home/mateusz/sources/skype_serial_killer/hosts.json").unwrap();
  let bytes = serde_json::to_string(&storage).unwrap().into_bytes();
  file.write_all(&bytes[..]).unwrap();
  format!("Saved: {}", serde_json::to_string_pretty(&storage).unwrap())
}

fn show_me_the_magic_get(_req: &HttpRequest) -> Result<HttpResponse> {
  println!("Got request");
  let mut file = File::open("/home/mateusz/sources/skype_serial_killer/hosts.json").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  let storage: StorageFile = serde_json::from_str(&contents).unwrap();
  Ok(HttpResponse::Ok()
    .content_type("application/json")
    .body(serde_json::to_string(&storage).unwrap())
  )
}

fn show_me_the_magic(_req: &HttpRequest) -> Result<HttpResponse> {
  let mut file = File::open("/home/mateusz/sources/skype_serial_killer/hosts.json").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  let storage: StorageFile = serde_json::from_str(&contents).unwrap();

  Ok(HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(format!("<i>No One, Bart and The Code Evangelist,<br>are proud to present<br>THE MARAUDER'S MAP</i><br><br>Locations:<br><pre>{}</pre>", serde_json::to_string_pretty(&storage).unwrap()))
  )
}

fn main() {
  HttpServer::new(|| App::new()
      .resource(r"/marauders_map/set", |r| r.method(http::Method::POST).with(handle_save))
      .resource(r"/marauders_map/i_solemnly_swear_that_i_am_up_to_no_good/raw", |r| r.f(show_me_the_magic_get))
      .resource(r"/marauders_map/i_solemnly_swear_that_i_am_up_to_no_good", |r| r.f(show_me_the_magic))
    )
    .bind("192.168.0.197:22666")
    .unwrap()
    .run();  
}
