extern crate ddl;

use ddl::Env;

#[test]
fn cmap() {
    const SRC: &str = include_str!("../examples/ddl/cmap.ddl");

    let mut env = Env::default();
    let defs = ddl::parser::parse(&env, SRC).unwrap();
    env.check_defs(defs).unwrap();
}

#[test]
fn edid() {
    const SRC: &str = include_str!("../examples/ddl/edid.ddl");

    let mut env = Env::default();
    let defs = ddl::parser::parse(&env, SRC).unwrap();
    env.check_defs(defs).unwrap();
}

#[test]
fn heroes_of_might_and_magic_bmp() {
    const SRC: &str = include_str!("../examples/ddl/heroes_of_might_and_magic_bmp.ddl");

    let mut env = Env::default();
    let defs = ddl::parser::parse(&env, SRC).unwrap();
    env.check_defs(defs).unwrap();
}

#[test]
fn object_id() {
    const SRC: &str = include_str!("../examples/ddl/object_id.ddl");

    let mut env = Env::default();
    let defs = ddl::parser::parse(&env, SRC).unwrap();
    env.check_defs(defs).unwrap();
}