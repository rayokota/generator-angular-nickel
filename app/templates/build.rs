
mod inner {
    extern crate syntex;
    extern crate diesel_codegen;

    use std::env;
    use std::path::Path;

    pub fn main() {
        let out_dir = env::var_os("OUT_DIR").unwrap();
        let mut registry = syntex::Registry::new();
        diesel_codegen::register(&mut registry);

        let src = Path::new("src/schema.in.rs");
        let dst = Path::new(&out_dir).join("schema.rs");

        registry.expand("", &src, &dst).unwrap();
    }
}

extern crate diesel;
use diesel::*;

fn main() {
    let connection = Connection::establish("postgres://postgres:postgres@localhost/my_db").unwrap();
    migrations::run_pending_migrations(&connection).unwrap();
    inner::main();
}
