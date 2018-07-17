#[macro_use]
extern crate mongodb;
extern crate rand;
extern crate libc;

use mongodb::{Client, ClientOptions, ThreadedClient};
use mongodb::common::{ReadMode, ReadPreference};
use mongodb::db::ThreadedDatabase;
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::time;
use std::fs;
use std::io;
use std::io::prelude::*;

extern crate actix_web;

use actix_web::server;
use std::path::PathBuf;
use actix_web::*;
use actix_web::{fs as ActixFs, fs::NamedFile};
use actix_web::{http::*};

fn index(req: HttpRequest) -> Result<NamedFile> {
    Ok(NamedFile::open("./dist/index.html")?)
}

fn second(req: HttpRequest) -> Result<String> {
    let name: PathBuf = req.match_info().query("name")?;
    Ok(format!("{:?} {:?}", "second ".to_string(), name))
}

fn redirect(req: HttpRequest) -> Result<String> {
    let name: PathBuf = req.match_info().query("name")?;
    Ok(format!("{:?} {:?}", "second ".to_string(), name))
}

fn callOne<F>(x: F) -> i32 where F: Fn(i32) -> i32 {
    x(1)
}

fn main() {
    println!("{}", callOne(|x| x + 2));

    let paths = fs::read_dir("./").unwrap();
    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }

    let mut x = 1;
    let y = &mut x;
    *y += 1;
    println!("{}", y);

    let mut y = vec![1, 2, 4];
    for i in &y {
        println!("iter {},", i);
    }

    let a = 2;
    let mut c: &i32;
    c = &a;
    println!("{},", c);

    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    let origin = Point { x: 1, y: 2 };
    match origin {
        Point { x, .. } => {
            println!("{:?}", origin)
        }
    }

    let mut s = "ss".to_string();
    let s = s + "ddd";
    println!("{}", s);

//    let client = Client::connect("localhost", 27017)
//        .unwrap();
//    let coll = client.db("test").collection("ctest");
//    let mut cursor = coll.find(None, None).unwrap();
//    for result in cursor {
//        if let Ok(item) = result {
//            if let Some(Bson::String(name)) = item.get("name") {
//                println!("title: {}", name);
//            }
//        }
//    }
//    coll.insert_one(doc!{ "name": "Back to the Future" ,"age":"13"}, None).unwrap();
//    coll.delete_many(doc!{"title": "Back to the Future"}, None).unwrap();
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    if let Some(a) = y {
        let sum = x + a;
        println!("sum: {}", sum);
    }
    //
    let mut data = Arc::new(Mutex::new(vec![1u32, 2, 3]));

    let (send, recv): (Sender<u32>, Receiver<u32>) = mpsc::channel();
    thread::spawn(move || {
        let r = recv.recv().ok().expect("Could not receive answer");
        println!("thread {}", r);
        let r = recv.recv().ok().expect("Could not receive answer");
        println!("thread {}", r);
        let r = recv.recv().ok().expect("Could not receive answer");
        println!("thread {}", r);
    });

    for mut i in 0..3 {
        let (data, sender) = (data.clone(), send.clone());
//        let data = data.clone();
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data[i] += 1;
            sender.send(data[i]);
//            println!("thread {}", data[i])
        });
    }

    server::new(|| {
        App::new().resource("/", |r| r.f(index))
            .default_resource(|r| r.method(Method::GET).h(redirect))
            .handler("/dist", ActixFs::StaticFiles::new("./dist"))
            .handler("/static", ActixFs::StaticFiles::new("./dist/static"))
            .route("/user/{name}", Method::GET, second)
            .resource("/path", |resource| {
                resource
                    .route()
                    .filter(pred::Get())
                    .filter(pred::Header("content-type", "text/plain"))
                    .f(|req| HttpResponse::Ok())
            })
            .middleware(middleware::Logger::default())
//            .allowed_origin("http://localhost:8080")// let CORS default to all
    }).shutdown_timeout(3)
        .bind("127.0.0.1:8080").unwrap()
        .run();
    println!("On 3000");
}
