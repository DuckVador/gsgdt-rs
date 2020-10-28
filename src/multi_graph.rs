use crate::graph::*;
use std::io::{self, Write};
use serde::{Deserialize, Serialize};

/// A collection of graphs.
#[derive(Deserialize, Serialize)]
pub struct MultiGraph {
    name: String,
    graphs: Vec<Graph>,
}

impl MultiGraph {
    pub fn new(name: String, graphs: Vec<Graph>) -> MultiGraph {
        MultiGraph { name, graphs }
    }

    // TODO: figure a better api to pass graphviz settings.
    // label should be a part of Graph struct maybe
    // maybe settings should be part of the struct
    pub fn to_dot<W: Write>(&self, w: &mut W, settings: &GraphvizSettings) -> io::Result<()> {
        let subgraphs = self.graphs.len() > 1;
        if subgraphs {
            writeln!(w, "digraph {} {{", self.name)?;
        }

        for graph in &self.graphs {
            graph.to_dot(w, settings, subgraphs)?;
        }

        if subgraphs {
            writeln!(w, "}}")?;
        }

        Ok(())
    }
}
