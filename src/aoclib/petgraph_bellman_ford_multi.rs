/* This is a modified version of bellman-ford to find all shortest paths
 * from https://github.com/petgraph/petgraph/issues/503 */
use petgraph::prelude::*;

use petgraph::visit::{IntoEdges, IntoNodeIdentifiers, NodeCount, NodeIndexable};

use petgraph::algo::{FloatMeasure, NegativeCycle};

#[derive(Debug, Clone)]
pub struct Paths<NodeId, EdgeWeight> {
    pub distances: Vec<EdgeWeight>,
    pub predecessors: Vec<Option<NodeId>>,
}

#[derive(Debug, Clone)]
pub struct MultiPaths<NodeId, EdgeWeight> {
    pub distances: Vec<EdgeWeight>,
    pub predecessors: Vec<Option<Vec<NodeId>>>,
}

/// Same as bellman_ford, but return all shortest paths
///
/// # Example
/// ```rust
/// use petgraph::Graph;
/// use aoclib::petgraph_bellman_ford_multi::bellman_ford_multi_predecessors;
/// use petgraph::prelude::*;
///
/// let mut g = Graph::new();
/// let a = g.add_node(()); // node with no weight
/// let b = g.add_node(());
/// let c = g.add_node(());
/// g.extend_with_edges(&[
///     (0, 1, 1.0),
///     (1, 2, 1.0),
///     (0, 2, 2.0),
/// ]);
///
///
/// let path = bellman_ford_multi_predecessors(&g, a);
/// assert!(path.is_ok());
/// let path = path.unwrap();
/// assert_eq!(path.distances, vec![    0.0,     1.0,    2.0 ]);
/// assert_eq!(path.predecessors, vec![None, Some(vec![a]), Some(vec![a, b])]);
///
pub fn bellman_ford_multi_predecessors<G>(
    g: G,
    source: G::NodeId,
) -> Result<MultiPaths<G::NodeId, G::EdgeWeight>, NegativeCycle>
where
    G: NodeCount + IntoNodeIdentifiers + IntoEdges + NodeIndexable,
    G::EdgeWeight: FloatMeasure,
{
    let ix = |i| g.to_index(i);

    // Step 1 and Step 2: initialize and relax
    let (distances, predecessors) = bellman_ford_initialize_relax_multi_predecessors(g, source);

    // Step 3: check for negative weight cycle
    for i in g.node_identifiers() {
        for edge in g.edges(i) {
            let j = edge.target();
            let w = *edge.weight();
            if distances[ix(i)] + w < distances[ix(j)] {
                return Err(NegativeCycle(()));
            }
        }
    }

    Ok(MultiPaths {
        distances,
        predecessors,
    })
}

// Perform Step 1 and Step 2 of the Bellman-Ford algorithm.
#[inline(always)]
fn bellman_ford_initialize_relax_multi_predecessors<G>(
    g: G,
    source: G::NodeId,
) -> (Vec<G::EdgeWeight>, Vec<Option<Vec<G::NodeId>>>)
where
    G: NodeCount + IntoNodeIdentifiers + IntoEdges + NodeIndexable,
    G::EdgeWeight: FloatMeasure,
{
    // Step 1: initialize graph
    let mut predecessor = vec![None; g.node_bound()];
    let mut distance = vec![<_>::infinite(); g.node_bound()];
    let ix = |i| g.to_index(i);
    distance[ix(source)] = <_>::zero();

    // Step 2: relax edges repeatedly
    for _ in 1..g.node_count() {
        let mut did_update = false;
        for i in g.node_identifiers() {
            for edge in g.edges(i) {
                let j = edge.target();
                let w = *edge.weight();
                if distance[ix(i)] + w < distance[ix(j)] {
                    distance[ix(j)] = distance[ix(i)] + w;
                    predecessor[ix(j)] = Some(vec![i]);
                    did_update = true;
                } else if distance[ix(i)] + w == distance[ix(j)]
                    && distance[ix(j)] != <_>::infinite()
                {
                    // In this branch we find predecessor with same cost
                    if let Some(v) = &mut predecessor[ix(j)] {
                        // TODO: need improvement
                        if !v.contains(&i) {
                            v.push(i);
                        }
                    }
                }
            }
        }
        if !did_update {
            break;
        }
    }
    (distance, predecessor)
}
