use std::collections::HashSet;
use petgraph::graphmap::UnGraphMap;
use regex::Regex;


fn main() {
    let filename = "input.txt";
    let contents = std::fs::read_to_string(filename).unwrap();
    let graph = build_graph(&contents);
    let cliques = find_triangles(&graph);
    // filter the cliques to those that contain a node with a t in it
    let t_cliques = find_cliques_with_t_len3(cliques);
    println!("Number of triangles with a node containing a t: {}", t_cliques);
    let password = find_password(&graph);
    println!("Largest clique: {}", password);

}

fn build_graph(contents: &str) -> UnGraphMap<&str, ()> {
    let mut graph: UnGraphMap<&str, ()> = UnGraphMap::new();
    for line in contents.lines() {
        let regex = Regex::new(r"(\w+)-(\w+)").unwrap();
        let captures = regex.captures(line).unwrap();
        let from = captures.get(1).unwrap().as_str();
        let to = captures.get(2).unwrap().as_str();
        graph.add_edge(from, to, ());
    }
    graph
}

fn find_triangles<'a>(graph: &'a UnGraphMap<&'a str, ()>) -> Vec<Vec<&'a str>> {
    let nodes: Vec<&'a str> = graph.nodes().collect();
    let mut triangles = Vec::new();

    for i in 0..nodes.len() {
        for j in i + 1..nodes.len() {
            for k in j + 1..nodes.len() {
                let a = nodes[i];
                let b = nodes[j];
                let c = nodes[k];

                // Check if all edges exist for a 3-clique.
                if graph.contains_edge(a, b)
                    && graph.contains_edge(b, c)
                    && graph.contains_edge(c, a)
                {
                    let mut tri = vec![a, b, c];
                    tri.sort();
                    triangles.push(tri);
                }
            }
        }
    }

    // Deduplicate if needed.
    triangles.sort();
    triangles.dedup();
    triangles
}

fn find_cliques_with_t_len3(cliques: Vec<Vec<&str>>) -> usize {
    let t_cliques: Vec<&Vec<&str>> = cliques.iter().filter(|clique| {
        for node in *clique {
            if node.starts_with("t") && clique.len() == 3 {
                return true;
            }
        }
        false
    }).collect();
    t_cliques.len()
}


fn bron_kerbosch<'a>(
    graph: &UnGraphMap<&'a str, ()>,
    r: &mut HashSet<&'a str>,
    p: &mut HashSet<&'a str>,
    x: &mut HashSet<&'a str>,
    cliques: &mut Vec<Vec<&'a str>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r.iter().copied().collect());
    } else {
        let p_snapshot: Vec<&'a str> = p.iter().copied().collect();
        for &v in &p_snapshot {
            let mut r_new = r.clone();
            r_new.insert(v);

            let neighbors: HashSet<&str> = graph.neighbors(v).collect();

            let mut p_new = p.intersection(&neighbors).copied().collect();
            let mut x_new = x.intersection(&neighbors).copied().collect();

            bron_kerbosch(graph, &mut r_new, &mut p_new, &mut x_new, cliques);

            p.remove(v);
            x.insert(v);
        }
    }
}

pub fn find_cliques<'a>(graph: &UnGraphMap<&'a str, ()>) -> Vec<Vec<&'a str>> {
    let mut cliques = Vec::new();
    let mut r = HashSet::new();
    let mut p: HashSet<&str> = graph.nodes().collect();
    let mut x = HashSet::new();

    bron_kerbosch(graph, &mut r, &mut p, &mut x, &mut cliques);

    cliques
}

fn find_password(graph: &UnGraphMap<&str, ()>) -> String {
    let cliques = find_cliques(&graph);
    let largest_clique = cliques.iter().max_by_key(|clique| clique.len()).unwrap();
    // order the nodes in the clique alphabetically
    let mut sorted_clique = largest_clique.clone();
    sorted_clique.sort();
    let joined_clique = sorted_clique.join(",");
    joined_clique
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_graph() {
        let filename = "test_input.txt";
        let graph_string = std::fs::read_to_string(filename).unwrap();
        let graph = build_graph(&graph_string);
        assert_eq!(graph.node_count(), 16);

    }

    #[test]
    fn test_find_cliques_in_input() {
        let filename = "test_input.txt";
        let graph_string = std::fs::read_to_string(filename).unwrap();
        let graph = build_graph(&graph_string);
        let cliques = find_triangles(&graph);
        //print all cliques
        for clique in &cliques {
            println!("{:?}", clique);
        }
        // filter the cliques to those that contain a node with a t in it
        let t_cliques = find_cliques_with_t_len3(cliques);
        assert_eq!(t_cliques, 7);
    }

    #[test]
    fn test_find_largest_clique() {
        let filename = "test_input.txt";
        let graph_string = std::fs::read_to_string(filename).unwrap();
        let graph = build_graph(&graph_string);
        let joined_clique = find_password(&graph);
        assert_eq!(joined_clique, "co,de,ka,ta");
    }


}