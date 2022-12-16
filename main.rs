

// SIX DEGREES OF SEPARATION:

// What is the usual distance between pairs of vertices?
// Is the answer very different fot this vs another graph?

use std::fs::File;
use std::io::prelude::*;
use rand::Rng;
use std::collections::HashMap;
use std::collections::VecDeque;

fn read_file(path: &str) -> Vec<(usize, usize)> {
    let mut count:usize = 0;
    let mut result: Vec<(usize, usize)> = Vec::new();
    let mut tbl: HashMap<String, usize> = HashMap::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        match line {
            Err(e) => println!("Got an error {:?}", e),
            Ok(st)=> {
                let x; 
                let y;
                let v: Vec<&str> = st.trim().split(' ').collect();
                let entry = tbl.get(v[0]);
                match entry {
                    None => {
                        tbl.insert(v[0].to_string(), count); // for the first column of numbers, create a new value that is smaller and call it x 
                        x = count;
                        count += 1; }
                    Some(val) => x = *val,
                }
                let entry = tbl.get(v[1]);
                match entry {
                    None => {
                        tbl.insert(v[1].to_string(), count); // for the second column of numbers, create a new value that is smaller and call it y
                        y = count;
                        count += 1; }
                    Some(val) => y = *val,
                }
                result.push((x, y))
            },
        }
    }
    return result;
}

type Vertex = usize;
type ListOfEdges = Vec<(Vertex,Vertex)>;
type AdjacencyLists = Vec<Vec<Vertex>>;

#[derive(Debug)]
struct Graph {
    n: usize, // vertex labels in {0,...,n-1}
    outedges: AdjacencyLists,
}

impl Graph {
    fn add_directed_edges(&mut self,
                          edges:&ListOfEdges) {
        for (u,v) in edges {
            self.outedges[*u].push(*v);
        }
    }
    fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }
    fn create_directed(n:usize,edges:&ListOfEdges)
                                            -> Graph {
        let mut g = Graph{n,outedges:vec![vec![];n]};
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g                                        
    }
    
    fn create_undirected(n:usize,edges:&ListOfEdges)
                                            -> Graph {
        let mut g = Self::create_directed(n,edges);
        g.add_directed_edges(&edges);
        g.sort_graph_lists();
        g                                        
    }
}

fn option_vector_to_int_vector(input: Vec<Option<u32>>) -> Vec<u32> {
    let mut output: Vec<u32> = Vec::new();
    for elem in input {
        if let Some(val) = elem {
            output.push(val);
        }
    }
    output
}

fn length_of_options_vector(options: Vec<Option<u32>>) -> usize {
    options.iter().filter(|x| x.is_some()).count()
}



fn main() {
    let mut edges = read_file("gplus_combined.txt");
    edges.sort();
    let n: usize = 107614;
    let graph = Graph::create_undirected(n,&edges);
    //for (i, l) in graph.outedges.iter().enumerate() {
    //println!("{} {:?}", i, *l);
    //}
    type Component = usize;
    fn mark_component_bfs(vertex:Vertex, graph:&Graph, component:&mut Vec<Option<Component>>, component_no:Component) {
    component[vertex] = Some(component_no);
    
    let mut component_queue = std::collections::VecDeque::new();
    component_queue.push_back(vertex);
    while let Some(v) = component_queue.pop_front() {
        for w in graph.outedges[v].iter() {
            if let None = component[*w] {
                component[*w] = Some(component_no);
                component_queue.push_back(*w);
            }
        }
    }
}
let mut component: Vec<Option<Component>> = vec![None;n];
let mut component_count = 0;
let start: Vertex = 2; // <= we'll start from this vertex
let mut distance: Vec<Option<u32>> = vec![None;graph.n];
distance[start] = Some(0); // <= we know this distance
let mut distance_queue: VecDeque<Vertex> = VecDeque::new();
distance_queue.push_back(start);
//println!("{:?}",distance_queue);
for v in 0..n {
    if let None = component[v] {
        component_count += 1;
        mark_component_bfs(v, &graph, &mut component, component_count);
        for u in graph.outedges[v].iter(){
            while let Some(v) = distance_queue.pop_front() { // new unprocessed component
                //println!("top {:?}",distance_queue);
                 for u in graph.outedges[v].iter() {
                    if let None = distance[*u] { // consider all unprocessed neighbors of v
                        distance[*u] = Some(distance[v].unwrap() + 1);
                        distance_queue.push_back(*u);
                        //println!("In {:?}",distance_queue);
                     }
                    }
                }
            };
        }
    };
    print!("There are {} components in this graph:\n[  ",component_count); //found 335 components 

    let distances = option_vector_to_int_vector(distance);
    //let total = length_of_options_vector(distance);
    
    let average: f64 = (distances.iter().fold(0,|sum,x|sum+x)/distances.len() as u32).into();
    


    println!("The average distance between vertices is {}.", average);



}


