use hashbrown::HashMap;

use rocket::{
  fairing::{Fairing, Info, Kind},
  request::Request,
  response::Response,
};

pub struct Csp;

impl Fairing for Csp {
  fn info(&self) -> Info {
    Info {
      name: "Content-Security-Policy addition",
      kind: Kind::Response,
    }
  }

  fn on_response(&self, _: &Request, response: &mut Response) {
    // create default CSP header
    let mut header = CspHeader::new();

    // update CSP header with any custom route headers
    for directives in response.headers().get("Content-Security-Policy") {
      for directive in directives.split(';') {
        let mut split = directive.trim().split(' ');
        let directive = split.next().unwrap(); // must be one element in a split
        let value = split.collect::<Vec<_>>().join(" ");
        header.update_directive(directive, &value);
      }
    }

    // overwrite CSP header
    response.set_raw_header("Content-Security-Policy", header.to_string());
  }
}

lazy_static! {
  static ref DEFAULT_CSP: HashMap<String, String> = {
    let mut map = HashMap::with_capacity(9);
    map.insert("default-src".into(), "'self'".into());
    map.insert("object-src".into(), "'none'".into());
    map.insert("script-src".into(), "'self'".into());
    map.insert("style-src".into(), "'self'".into());
    map.insert("font-src".into(), "'self'".into());
    let img_src = match crate::CAMO_URL.as_ref() {
      Some(ref url) if url.host_str().is_some() => format!("'self' {}", url.host_str().unwrap()),
      _ => "'self'".into(),
    };
    map.insert("img-src".into(), img_src);
    map.insert("require-sri-for".into(), "script style".into());
    map.insert("frame-ancestors".into(), "'none'".into());
    map.insert("base-uri".into(), "'none'".into());
    map.insert("block-all-mixed-content".into(), "".into());
    map
  };
}

struct CspHeader {
  directives: HashMap<String, String>,
}

impl CspHeader {
  fn new() -> Self {
    CspHeader {
      directives: DEFAULT_CSP.clone(),
    }
  }

  fn update_directive(&mut self, dir: &str, update: &str) {
    let entry = self.directives.entry(dir.to_string()).or_insert_with(Default::default);
    if !entry.is_empty() {
      *entry += " ";
    }
    *entry += update;
  }
}

impl ToString for CspHeader {
  fn to_string(&self) -> String {
    self.directives.iter()
      .map(|(name, value)| {
        if value.is_empty() {
          name.clone()
        } else {
          format!("{} {}", name, value)
        }
      })
      .collect::<Vec<_>>()
      .join("; ")
  }
}
