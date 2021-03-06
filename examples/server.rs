#![cfg_attr(all(feature="serde_type"), feature(proc_macro))]

extern crate iron;
extern crate router;
extern crate env_logger;
extern crate handlebars_iron as hbs;
#[cfg(not(feature = "serde_type"))]
extern crate rustc_serialize;
#[cfg(feature = "serde_type")]
extern crate serde;
#[cfg(feature = "serde_type")]
extern crate serde_json;
#[cfg(feature = "serde_type")]
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate maplit;

use iron::prelude::*;
use iron::status;
use router::Router;
use hbs::{Template, HandlebarsEngine, DirectorySource, MemorySource};

#[cfg(not(feature = "serde_type"))]
mod data {
    use rustc_serialize::json::{ToJson, Json};
    use std::collections::BTreeMap;

    pub struct Team {
        name: String,
        pts: u16,
    }

    impl ToJson for Team {
        fn to_json(&self) -> Json {
            let mut m: BTreeMap<String, Json> = BTreeMap::new();
            m.insert("name".to_string(), self.name.to_json());
            m.insert("pts".to_string(), self.pts.to_json());
            m.to_json()
        }
    }

    pub fn make_data() -> BTreeMap<String, Json> {
        let mut data = BTreeMap::new();

        data.insert("year".to_string(), "2015".to_json());

        let teams = vec![Team {
                             name: "Jiangsu Sainty".to_string(),
                             pts: 43u16,
                         },
                         Team {
                             name: "Beijing Guoan".to_string(),
                             pts: 27u16,
                         },
                         Team {
                             name: "Guangzhou Evergrand".to_string(),
                             pts: 22u16,
                         },
                         Team {
                             name: "Shandong Luneng".to_string(),
                             pts: 12u16,
                         }];

        data.insert("teams".to_string(), teams.to_json());
        data.insert("engine".to_string(), "rustc_serialize".to_json());
        data
    }
}

#[cfg(feature = "serde_type")]
mod data {
    use serde_json::value::{self, Value};
    use std::collections::BTreeMap;

    #[derive(Serialize, Debug)]
    pub struct Team {
        name: String,
        pts: u16,
    }

    pub fn make_data() -> BTreeMap<String, Value> {
        let mut data = BTreeMap::new();

        data.insert("year".to_string(), value::to_value(&"2015"));

        let teams = vec![Team {
                             name: "Jiangsu Sainty".to_string(),
                             pts: 43u16,
                         },
                         Team {
                             name: "Beijing Guoan".to_string(),
                             pts: 27u16,
                         },
                         Team {
                             name: "Guangzhou Evergrand".to_string(),
                             pts: 22u16,
                         },
                         Team {
                             name: "Shandong Luneng".to_string(),
                             pts: 12u16,
                         }];

        data.insert("teams".to_string(), value::to_value(&teams));
        data.insert("engine".to_string(), value::to_value(&"serde_json"));
        data
    }
}

/// the handlers
fn index(_: &mut Request) -> IronResult<Response> {
    use data::*;

    let mut resp = Response::new();
    let data = make_data();
    resp.set_mut(Template::new("some/path/hello", data)).set_mut(status::Ok);
    Ok(resp)
}

fn memory(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    resp.set_mut(Template::new("memory", ())).set_mut(status::Ok);
    Ok(resp)
}

fn temp(_: &mut Request) -> IronResult<Response> {
    use data::*;

    let mut resp = Response::new();
    let data = make_data();
    resp.set_mut(Template::with("<h1>{{engine}}</h1>", data)).set_mut(status::Ok);
    Ok(resp)
}

fn main() {
    env_logger::init().unwrap();

    let mut hbse = HandlebarsEngine::new();

    // add a directory source, all files with .hbs suffix will be loaded as template
    hbse.add(Box::new(DirectorySource::new("./examples/templates/", ".hbs")));

    let mem_templates = btreemap! {
        "memory".to_owned() => "<h1>Memory Template</h1>".to_owned()
    };
    // add a memory based source
    hbse.add(Box::new(MemorySource(mem_templates)));

    // load templates from all registered sources
    if let Err(r) = hbse.reload() {
        panic!("{}", r);
    }


    let mut router = Router::new();
    router.get("/", index)
          .get("/mem", memory)
          .get("/temp", temp);
    let mut chain = Chain::new(router);
    chain.link_after(hbse);
    println!("Server running at http://localhost:3000/");
    Iron::new(chain).http("localhost:3000").unwrap();
}
