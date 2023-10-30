use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};
use serde_yaml::{Error};

type GraphType = HashMap::<String, HashSet<String>>;


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Node {
  name: String,
  #[serde(rename = "fn")]
  func: String,
  attrs: Option<HashMap<String, String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GraphDefinition {
  name: String,
  nodes: Vec<Node>,
  allowed_connections: HashMap<String, HashSet<String>>,
  graph: GraphType
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct File {
  #[serde(skip)]
  #[serde(rename = "fns")]
  func: Option<String>,
  graph_defn: GraphDefinition,
}

pub fn parse_graph2(graph_code: &String) -> Result<File, Error> {
  let data : Result<File, Error> = serde_yaml::from_str(&graph_code);
  data
}

pub fn parse_graph(graph_code: &String) -> Result<GraphType, Error> {
  let data : Result<File, Error> = serde_yaml::from_str(&graph_code);
  match data {
    Ok(d) => return Ok(d.graph_defn.graph),
    Err(e) => return Err(e)
  }
}

#[test]
fn parse_graph_test() {
  let code = r#"
fns:
  - &server_fn |
      def fn():
        log("hello server")

  - &client_fn |
      def fn():
        log("hello client")

graph_defn:
  name: "test graph"
  nodes:
    - name: server
      fn: *server_fn
      attrs:
        tick_timer: 5s
        color: "yellow"
    - name: client1
      fn: *client_fn
    - name: client2
      fn: *client_fn
  allowed_connections:
    server: [client1, client2]
  graph:
    server: [client1, client2]
    client1: [client2, server]
"#;

  let res = parse_graph(&code.to_string());
  println!("{:?}", res);
  assert!(res.is_ok());
}
