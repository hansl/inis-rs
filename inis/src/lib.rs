use crate::de::Deserializer;
use serde::de::DeserializeOwned;
use serde::Deserialize;

pub mod de;
pub mod error;
pub mod parser;

pub use error::Error;
pub use parser::Ini;

pub fn parse_ini(input: &str) -> Result<Ini, Error> {
    parser::parse(input)
}

pub fn from_str<T: DeserializeOwned>(input: &str) -> Result<T, Error> {
    let ini = parse_ini(input)?;
    let mut de = Deserializer::new(&ini);
    let value = Deserialize::deserialize(&mut de)?;

    Ok(value)
}

#[test]
fn empty() {
    let content = "";
    let ini = parse_ini(content).unwrap();
    assert!(ini.is_empty());
}

#[test]
fn root() {
    let content = "foo=bar";
    let ini = parse_ini(content).unwrap();
    assert!(!ini.is_empty());
    assert!(ini.root().contains("foo"));
    assert_eq!(ini.root().get("foo"), Some("bar"));
}

#[test]
fn simple() {
    let content = r#"
        foo=bar
        [section]
        baz=qux
    "#;

    let ini = parse_ini(content).unwrap();
    assert!(!ini.is_empty());
    assert!(ini.root().contains("foo"));
    assert_eq!(ini.root().get("foo"), Some("bar"));
    assert!(ini.contains("section", "baz"));
    assert_eq!(ini.get("section", "baz"), Some("qux"));
}

#[test]
fn complex() {
    let content = r#"
        key1=value1
        key2 = value2

        [section1]
        key3=  value3  ; Inline comment.

        ; Comment
        [section2]
        key4=value4

        [section3]
        ; Other Comment
        key5=value5


        key6   =   value6 with space  ; ... and comment.

        [section2]
        key7=value7
    "#;

    let ini = parse_ini(content).unwrap();
    assert!(!ini.is_empty());
    assert_eq!(ini.root().get("key1"), Some("value1"));
    assert_eq!(ini.root().get("key2"), Some("value2"));
    assert_eq!(ini.root().get("key3"), None);
    assert!(ini.contains("section1", "key3"));
    assert!(!ini.contains("section1", "key4"));
    assert_eq!(ini.get("section1", "key3"), Some("value3"));
    assert_eq!(ini.get("section2", "key4"), Some("value4"));
    assert_eq!(ini.get("section3", "key5"), Some("value5"));
    assert_eq!(ini.get("section3", "key6"), Some("value6 with space"));
    assert_eq!(ini.get("section2", "key7"), Some("value7"));
    assert_eq!(ini.get("section999", "baz"), None);
}

#[test]
fn serde() {
    #[derive(Debug, serde::Deserialize)]
    struct T1 {
        b: bool,
        s: String,
        u: u64,
    }

    #[derive(Debug, serde::Deserialize)]
    struct T2 {
        section1: T1,
        key1: String,
    }

    let content = r#"
        key1=value1
        key2 = value2

        [section1]
        b=true
        s=string
        u=12345

        [section2]
        ignored=yup
    "#;

    let t: Result<T2, _> = from_str(content);
    eprintln!("{:?}", t);
    assert!(t.is_ok());
}
