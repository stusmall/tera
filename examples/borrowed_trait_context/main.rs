extern crate tera;
extern crate serde_json;

use std::collections::HashMap;
use std::sync::Arc;

use serde_json::value::{to_value, Value};
use tera::{Result, Tera, Function};


#[derive(Debug)]
struct GetPage<'a> {
    pages: HashMap<&'a str, &'a str>
}
impl<'a> GetPage<'a> {
    pub fn new(data: &'a HashMap<String, String>) -> Self {
        let mut pages = HashMap::new();
        for (key, value) in data {
            pages.insert(key.as_ref(), value.as_ref());
        }
        Self {
            pages
        }
    }
}

impl<'a> Function for GetPage<'a> {
    fn call(&self, _: &HashMap<String, Value>) -> Result<Value> {
        // In practice it should get the key from the args but that's not the important part so
        // skipping it
        Ok(to_value(self.pages.get("page1")).unwrap())
    }
}

#[derive(Debug)]
struct Site<'site> {
    pages: HashMap<String, String>,
    tera: Tera<'site>,
}

impl<'site> Site<'site> {
    pub fn new() -> Self {
        let tera = Tera::default();
        let mut pages = HashMap::new();
        pages.insert("page1".to_string(), "Some content".to_string());
        pages.insert("page2".to_string(), "Some other content".to_string());
        Self {
            tera,
            pages
        }
    }

    pub fn register_tera_fns(&mut self) {
        let get_page = GetPage::new(&self.pages);
        self.tera.register_function("get_page", Arc::new(get_page));
    }
}

fn rebuild<'site>(site: &'site mut Site<'site>) {
    site.register_tera_fns();
}

fn main() {
    let mut site = Site::new();

    site.register_tera_fns();
    rebuild(&mut site);
    println!("Done!");
}
