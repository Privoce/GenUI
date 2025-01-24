use std::{
    collections::HashMap,
    fmt::Display,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::error::{ConvertError, Error, ParseError};
use nom::{
    branch::alt, bytes::complete::take_until, character::complete::multispace0,
    multi::separated_list0,
};
use nom::{
    bytes::complete::{tag, take_until1},
    combinator::map,
    sequence::{delimited, separated_pair},
    IResult,
};
use toml_edit::{value, InlineTable, Item, Table};

use super::fs;

/// ## Rust Dependence
/// Describe the writing of dependencies in Cargo.toml
///
/// format: `name = { version = "0.1.0", features = ["feature1", "feature2"], default-features = false, git/path = "git/path", branch = "git branch", rev = "git rev", tag = "git tag"}`
/// ### Example
/// ```rust
/// let mut makepad_widget = RustDependence::new("makepad-widgets");
/// makepad_widget.set_ty(DepType::local(
///     "E:/Rust/try/makepad/makepad/rik/makepad/widgets",
/// ));
/// ```
/// ### Builder
/// I recommend using the builder pattern to build the dependence
///
/// the builder pattern is more readable and easy to use(the methods in RustDependence is a cheap supplement)
///
/// you can get example in `generate/makepad/src/compiler/builder/dep.rs`
///
/// ### Why builder is not in this project
/// builder is a pattern, the pattern is not a must in the project.
/// In most case, the builder need a parent struct to build, so the builder is not a good choice to bind
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RustDependence {
    /// name of the dependence
    pub name: String,
    /// version of the dependence
    pub version: Option<String>,
    /// features of the dependence (optional)
    pub features: Option<Vec<String>>,
    /// default features of the dependence (optional)
    pub default_features: Option<bool>,
    /// type of the dependence: crate, remote, local
    pub ty: DepType,
}

impl RustDependence {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            version: None,
            features: None,
            default_features: None,
            ty: DepType::Crate,
        }
    }
    pub fn set_version(&mut self, version: &str) -> &mut Self {
        self.version.replace(version.to_string());
        self
    }
    pub fn set_features(&mut self, features: Vec<String>) -> &mut Self {
        self.features.replace(features);
        self
    }
    pub fn set_default_features(&mut self, default_features: bool) -> &mut Self {
        self.default_features.replace(default_features);
        self
    }
    pub fn set_ty(&mut self, ty: DepType) -> &mut Self {
        self.ty = ty;
        self
    }
    /// convert to toml edit table value
    /// which can call insert fn when need to insert into `TableKeyValue`
    /// return (name, value)
    pub fn to_table_kv(&self) -> (String, Item) {
        let mut item = InlineTable::new();

        match &self.ty {
            DepType::Crate => {}
            DepType::Remote(remote) => {
                item.insert("git", value(remote.url.as_str()).into_value().unwrap());

                // item["git"] = );
                if let Some(branch) = remote.branch.as_ref() {
                    // item["branch"] = value(branch);
                    item.insert("branch", value(branch).into_value().unwrap());
                }
                if let Some(rev) = remote.rev.as_ref() {
                    // item["rev"] = value(rev);
                    item.insert("rev", value(rev).into_value().unwrap());
                }
                if let Some(tag) = remote.tag.as_ref() {
                    // item["tag"] = value(tag);
                    item.insert("tag", value(tag).into_value().unwrap());
                }
            }
            DepType::Local(local) => {
                // item["path"] = value(local.to_str().unwrap());
                item.insert("path", value(fs::path_to_str(local)).into_value().unwrap());
            }
        }

        if let Some(version) = self.version.as_ref() {
            // item["version"] = value(version);
            item.insert("version", value(version).into_value().unwrap());
        }
        if let Some(features) = self.features.as_ref() {
            // item["features"] = value(features.join(", "));
            item.insert("features", value(features.join(", ")).into_value().unwrap());
        }
        if let Some(default_features) = self.default_features.as_ref() {
            // item["default-features"] = value(*default_features);
            item.insert(
                "default-features",
                value(*default_features).into_value().unwrap(),
            );
        }

        (
            self.name.to_string(),
            Item::Value(toml_edit::Value::InlineTable(item)),
        )
    }
    pub fn from_item(value: &Item) -> Result<Vec<RustDependence>, Error> {
        value.as_table().map_or_else(
            || {
                Err(ConvertError::FromTo {
                    from: "toml".to_string(),
                    to: "toml table".to_string(),
                }
                .into())
            },
            |deps| {
                let mut rust_deps = Vec::new();
                for (k, v) in deps.iter() {
                    rust_deps.push(RustDependence::from_str(&format!("{} = {}", k, v))?);
                }
                Ok(rust_deps)
            },
        )
    }
    pub fn eq(left: &Vec<RustDependence>, right: &Vec<RustDependence>) -> bool {
        left == right
    }
    pub fn vec_to_item(deps: &Vec<RustDependence>) -> Item {
        Item::Table(deps.iter().fold(Table::new(), |mut table, dep| {
            let (name, item) = dep.to_table_kv();
            table.insert(&name, item);
            table
        }))
    }
}

#[derive(Debug)]
enum Value<'a> {
    Version(&'a str),
    Git(&'a str),
    Path(&'a str),
    Branch(&'a str),
    Rev(&'a str),
    Tag(&'a str),
    DefaultFeatures(bool),
    Features(Vec<String>),
}

impl<'a> Value<'a> {
    pub fn to_string(&self) -> String {
        match self {
            Value::Version(v) => v.to_string(),
            Value::Git(v) => v.to_string(),
            Value::Path(v) => v.to_string(),
            Value::Branch(v) => v.to_string(),
            Value::Rev(v) => v.to_string(),
            Value::Tag(v) => v.to_string(),
            _ => panic!("not support to string"),
        }
    }
    pub fn to_vec(&self) -> Vec<String> {
        match self {
            Value::Features(v) => v.clone(),
            _ => panic!("not support to vec"),
        }
    }
    pub fn to_bool(&self) -> bool {
        match self {
            Value::DefaultFeatures(v) => *v,
            _ => panic!("not support to bool"),
        }
    }
}

fn holder(s: &str) -> IResult<&str, &str> {
    delimited(trim(tag("{")), trim(take_until1("}")), trim(tag("}")))(s)
}

/// use in remote dependence
fn git(s: &str) -> IResult<&str, (&str, Value)> {
    separated_pair(
        trim(tag("git")),
        trim(tag("=")),
        map(parse_string, |v| Value::Git(v)),
    )(s)
}
/// use in local
fn path(s: &str) -> IResult<&str, (&str, Value)> {
    separated_pair(
        trim(tag("path")),
        trim(tag("=")),
        map(parse_string, |v| Value::Path(v)),
    )(s)
}
fn default_features(s: &str) -> IResult<&str, (&str, Value)> {
    separated_pair(
        trim(tag("default-features")),
        trim(tag("=")),
        map(parse_string, |s| Value::DefaultFeatures(s == "true")),
    )(s)
}
fn features(s: &str) -> IResult<&str, (&str, Value)> {
    separated_pair(
        trim(tag("features")),
        trim(tag("=")),
        map(parse_string, |s| {
            Value::Features(s.split(",").map(|s| s.trim().to_string()).collect())
        }),
    )(s)
}
fn branch(s: &str) -> IResult<&str, (&str, Value)> {
    separated_pair(
        trim(tag("branch")),
        trim(tag("=")),
        map(parse_string, |v| Value::Branch(v)),
    )(s)
}
fn version(s: &str) -> IResult<&str, (&str, Value)> {
    separated_pair(
        trim(tag("version")),
        trim(tag("=")),
        map(parse_string, |v| Value::Version(v)),
    )(s)
}
fn rev(s: &str) -> IResult<&str, (&str, Value)> {
    separated_pair(
        trim(tag("rev")),
        trim(tag("=")),
        map(parse_string, |v| Value::Rev(v)),
    )(s)
}

fn dep_tag(s: &str) -> IResult<&str, (&str, Value)> {
    separated_pair(
        trim(tag("tag")),
        trim(tag("=")),
        map(parse_string, |v| Value::Tag(v)),
    )(s)
}

impl FromStr for RustDependence {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn kvs(s: &str) -> IResult<&str, Vec<(&str, Value)>> {
            // remove left `{` and right `}`
            let (_, kvs) = holder(s)?;

            separated_list0(
                tag(","),
                alt((
                    version,
                    features,
                    default_features,
                    git,
                    path,
                    branch,
                    rev,
                    dep_tag,
                )),
            )(kvs)
        }
        /// parse the dependence from crate -------------------------------------------------------------------------------------------------
        fn from_crate(s: &str) -> IResult<&str, RustDependence> {
            fn only_version(s: &str) -> IResult<&str, RustDependence> {
                map(parse_string, |version| {
                    let mut dep = RustDependence::new("");
                    dep.set_ty(DepType::Crate).set_version(version);
                    return dep;
                })(s)
            }
            fn normal_dep(s: &str) -> IResult<&str, RustDependence> {
                let (s, kvs) = kvs(s)?;
                let kvs: HashMap<&str, Value> = kvs.into_iter().collect();
                // get version, features, default-features in kvs, if not exist, return None
                if kvs.contains_key("git") || kvs.contains_key("path") {
                    return Err(nom::Err::Error(nom::error::Error::new(
                        "crate dependence must not contain git or path",
                        nom::error::ErrorKind::Tag,
                    )));
                }

                let dep = RustDependence {
                    name: "".to_string(),
                    version: kvs.get("version").map(|v| v.to_string()),
                    features: kvs.get("features").map(|v| v.to_vec()),
                    default_features: kvs.get("default-features").map(|v| v.to_bool()),
                    ty: DepType::Crate,
                };
                if !s.trim().is_empty() {
                    dbg!("remain");
                    return Err(nom::Err::Error(nom::error::Error::new(
                        s,
                        nom::error::ErrorKind::Tag,
                    )));
                }
                Ok(("", dep))
            }
            alt((only_version, normal_dep))(s)
        }
        /// parse the dependence from remote ------------------------------------------------------------------------------------------------
        fn from_remote(s: &str) -> IResult<&str, RustDependence> {
            let (s, kvs) = kvs(s)?;

            let kvs: HashMap<&str, Value> = kvs.into_iter().collect();

            let url = match kvs.get("git") {
                Some(v) => v.to_string(),
                None => {
                    return Err(nom::Err::Error(nom::error::Error::new(
                        "git dependence must contain git url",
                        nom::error::ErrorKind::Tag,
                    )))
                }
            };

            let dep = RustDependence {
                name: "".to_string(),
                version: kvs.get("version").map(|v| v.to_string()),
                features: kvs.get("features").map(|v| v.to_vec()),
                default_features: kvs.get("default-features").map(|v| v.to_bool()),
                ty: DepType::Remote(RemoteDep {
                    url,
                    branch: kvs.get("branch").map(|v| v.to_string()),
                    rev: kvs.get("rev").map(|v| v.to_string()),
                    tag: kvs.get("tag").map(|v| v.to_string()),
                }),
            };

            if !s.trim().is_empty() {
                return Err(nom::Err::Error(nom::error::Error::new(
                    s,
                    nom::error::ErrorKind::Tag,
                )));
            }
            Ok(("", dep))
        }
        /// parse the dependence from local -------------------------------------------------------------------------------------------------
        fn from_local(s: &str) -> IResult<&str, RustDependence> {
            let (s, kvs) = kvs(s)?;
            let kvs: HashMap<&str, Value> = kvs.into_iter().collect();
            // get path, version, features, default-features in kvs, if not exist, return None

            let path = match kvs.get("path") {
                Some(v) => v.to_string(),
                None => {
                    return Err(nom::Err::Error(nom::error::Error::new(
                        "local dependence must contain path",
                        nom::error::ErrorKind::Tag,
                    )))
                }
            };

            let dep = RustDependence {
                name: "".to_string(),
                version: kvs.get("version").map(|v| v.to_string()),
                features: kvs.get("features").map(|v| v.to_vec()),
                default_features: kvs.get("default-features").map(|v| v.to_bool()),
                ty: DepType::Local(PathBuf::from(path)),
            };
            if !s.trim().is_empty() {
                return Err(nom::Err::Error(nom::error::Error::new(
                    s,
                    nom::error::ErrorKind::Tag,
                )));
            }
            Ok(("", dep))
        }

        match separated_pair(
            trim(take_until1("=")),
            trim(tag("=")),
            alt((from_crate, from_remote, from_local)),
        )(s)
        {
            Ok((s, (name, mut dep))) => {
                if !s.trim().is_empty() {
                    return Err(ParseError::rust_dep(s).into());
                }
                dep.name = name.trim().to_string();
                Ok(dep)
            }
            Err(_) => Err(ParseError::rust_dep(s).into()),
        }
    }
}

/// ## The type of dependence
///
/// - Crate
/// - Remote
/// - Local
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DepType {
    /// crate 表示来自crates.io的依赖使用cargo install安装
    Crate,
    /// remote 表示来自远程的依赖, 可能是Github等
    Remote(RemoteDep),
    /// local 表示本地的依赖
    Local(PathBuf),
}

impl DepType {
    pub fn local<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        DepType::Local(path.as_ref().to_path_buf())
    }
}

impl Display for DepType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DepType::Crate => write!(f, ""),
            DepType::Remote(remote) => remote.fmt(f),
            DepType::Local(local) => {
                f.write_fmt(format_args!("path = \"{}\"", local.to_str().unwrap()))
            }
        }
    }
}

/// ## Git remote dependence
/// format: `url = "git/url", branch = "git branch", rev = "git rev", tag = "git tag"`
/// ### Example
/// ```toml
/// serde = { git = "https://serde/git/url", branch = "master" }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteDep {
    pub url: String,
    pub branch: Option<String>,
    /// HEAD commit of PR (SHA1 hash)
    pub rev: Option<String>,
    pub tag: Option<String>,
}

impl RemoteDep {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            branch: None,
            rev: None,
            tag: None,
        }
    }
}

impl Display for RemoteDep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut other = String::new();

        if let Some(branch) = self.branch.as_ref() {
            other.push_str(&format!("branch = \"{}\", ", branch));
        }
        if let Some(rev) = self.rev.as_ref() {
            other.push_str(&format!("rev = \"{}\", ", rev));
        }
        if let Some(tag) = self.tag.as_ref() {
            other.push_str(&format!("tag = \"{}\", ", tag));
        }

        f.write_fmt(format_args!("git = \"{}\"", self.url))
    }
}

fn parse_string(input: &str) -> IResult<&str, &str> {
    delimited(tag("\""), take_until("\""), tag("\""))(input)
}

#[allow(unused_mut)]
fn trim<'a, P, O>(mut parser: P) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    P: FnMut(&'a str) -> IResult<&'a str, O>,
{
    delimited(multispace0, parser, multispace0)
}

#[cfg(test)]
mod test_dep {
    use std::fs::read_to_string;

    use toml_edit::{value, Item, Table};

    use crate::common::RustDependence;
    #[test]
    fn read() {
        let content = read_to_string(
            "/Users/shengyifei/projects/gen_ui/GenUI/gen/compiler/gpiler/Cargo.toml",
        )
        .unwrap();
        let toml = content.parse::<toml_edit::DocumentMut>().unwrap();

        dbg!(&toml["dependencies"]);
    }

    #[test]
    fn toml_item() {
        let mut item = Item::Table(Table::new());
        item["version"] = value("0.1.0");
        // item.as_inline_table_mut().map(|t| t.fmt());
        dbg!(item.to_string());
    }

    #[test]
    fn toml_str_parser() {
        let crate1 = r#"serde = "0.1.0""#;
        let crate2 = r#"serde = { version = "0.1.0" }"#;

        let remote1 = r#"serde = { git = "https://serde/git/url", branch = "master" }"#;
        let remote2 = r#"serde = { version = "0.2.0", git = "https://serde/git/url", branch = "master", rev = "123456" }"#;

        let local = r#"serde = { path = "E:/Rust/try/makepad/makepad/rik/makepad/widgets" }"#;

        let crate1_res = crate1.parse::<RustDependence>().unwrap();
        let crate2_res = crate2.parse::<RustDependence>().unwrap();
        let remote1_res = remote1.parse::<RustDependence>().unwrap();
        let remote2_res = remote2.parse::<RustDependence>().unwrap();
        let local_res = local.parse::<RustDependence>().unwrap();

        dbg!(crate1_res);
        dbg!(crate2_res);
        dbg!(remote1_res);
        dbg!(remote2_res);
        dbg!(local_res);
    }
}
