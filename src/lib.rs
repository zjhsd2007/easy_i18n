use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fs, fs::File, io::BufReader, path::Path};
use std::sync::{Mutex, MutexGuard};

const INTER_REG: Lazy<Regex> = Lazy::new(|| Regex::new(r"%\d+").unwrap());

static I18N:Lazy<Mutex<I18n>> = Lazy::new(|| {
    Mutex::new(I18n::new("zh"))
});

pub fn set_lang(lang:&str) {
    let mut i18n = I18N.lock().unwrap();
    i18n.set_lang(lang);
}

pub fn set_source(path:&Path) {
    let mut i18n = I18N.lock().unwrap();
    i18n.set_source(path);
}


type Namespace = String;

#[derive(Debug, Clone, Default)]
pub struct I18n {
    pub(crate) lang:String,
    pub(crate) source: HashMap<String, Source>,
}

impl I18n {
    pub fn new(lang:&str) -> I18n {
        I18n {
            lang:lang.to_string(),
            source: HashMap::new(),
        }
    }

    pub fn set_lang(&mut self, lang:&str){
        self.lang = lang.to_string();
    }

    pub fn set_source(&mut self, path:&Path){
        self.source = load_source(path);
    }

    pub fn translate(&self, text: &str, ns: Option<Namespace>) -> String {
        self.source
            .get(self.lang.as_str())
            .and_then(|source| source.get_val(text, ns))
            .unwrap_or(text.to_string())
    }

    pub fn trans_with_inter(&self, text: &str, vals: Vec<String>, ns: Option<Namespace>) -> String {
        let new_text = self.translate(text, ns);
        INTER_REG
            .replace_all(new_text.as_str(), |caps: &Captures| {
                caps.get(0)
                    .and_then(|m| m.as_str().replace('%', "").parse::<u8>().ok())
                    .and_then(|v| vals.get(v as usize - 1))
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            })
            .into_owned()
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Source(HashMap<Namespace, HashMap<String, String>>);
impl Source {
    pub fn from_path(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut json_val = serde_json::Deserializer::from_reader(reader);
        Source::deserialize(&mut json_val).context("[source error]: source parse error.")
    }

    pub fn get_val(&self, key: &str, ns: Option<Namespace>) -> Option<String> {
        let ns = ns.unwrap_or("common".to_string());
        self.0
            .get(ns.as_str())
            .and_then(|map| map.get(key).map(|v| v.to_string()))
    }
}


fn load_source(path:&Path) -> HashMap<String, Source> {
    let mut map = HashMap::new();
    if let Ok(dir) = fs::read_dir(path) {
        for entry in dir {
            if let Ok(entry) = entry {
                let path = entry.path();
                if !path.is_dir() {
                    if let Some((file_name, file_type)) = path
                        .file_name()
                        .and_then(|f| f.to_str())
                        .and_then(|f| f.rsplit_once('.'))
                    {
                        if file_type.to_lowercase() == "json".to_string() {
                            if let Ok(source) = Source::from_path(&path) {
                                map.insert(file_name.to_lowercase(), source);
                            }
                        }
                    }
                }
            }
        }
    }
    map
}

#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! i18n {
    ($key:expr) => {
        {
            let i18n = I18N.lock().unwrap();
            i18n.translate($key, None)
        }
    };

    ($key:expr, ns=$ns:expr) => {
        {
            let i18n = I18N.lock().unwrap();
            i18n.translate($key, Some($ns.to_string()))
        }
    };

    ($key:expr, $($args:expr),*) => {
        {
            let i18n = I18N.lock().unwrap();
            let text = i18n.translate($key, None);
            let mut vals = vec![];
            $(vals.push($args.to_string());)*
            i18n.trans_with_inter($key, vals, None)
        }
    };

    ($key:expr, $($args:expr)*, ns=$ns:expr) => {
        {
            let i18n = I18N.lock().unwrap();
            let text = i18n.translate($key, Some($ns.to_string()));
            let mut vals = vec![];
            $(vals.push($args.to_string());)*
            i18n.trans_with_inter($key, vals, None)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        set_lang("en");
        set_source(Path::new("./source"));
        dbg!(i18n!("这是一个测试"));
        dbg!(i18n!("这是一个测试", ns="namespace1"));
        dbg!(i18n!("这是一个有插值的%1测试%2", 11, 22));
    }
}
