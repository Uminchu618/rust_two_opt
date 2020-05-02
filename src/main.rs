mod plot;
use rand::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

const FILE_NAME: &str = "burma14.tsp";

#[derive(Debug)]
struct Node {
    id: i32,
    x: f64,
    y: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut route = read_input_file()?;
    println!("{:?}", route);
    println!("{:?}", total_distance(&route));
    hillclimb(&mut route,1000000);
    println!("{:?}", route);
    println!("{:?}", total_distance(&route));
    let plot_vec =  route.iter().map(|item| (item.x as f32 ,item.y as f32)).collect();
    plot::plot_route(plot_vec);
    Ok(())
}

/// 移動距離の計算
fn distance(node1: &Node, node2: &Node) -> f64 {
    ((node1.x - node2.x).powf(2.0) + (node1.y - node2.y).powf(2.0)).sqrt()
}

/// 総移動距離の計算
fn total_distance(route: &Vec<Node>) -> f64 {
    let mut total: f64 = 0.0;
    for i in 1..route.len() {
        total += distance(&route[i - 1], &route[i]);
    }
    total + distance(&route[0], &route[route.len() - 1])
}

/// 山登り法による経路の最適化
///   route: 初期経路
///   maxIterations: 繰り返し回数
fn hillclimb(route: &mut Vec<Node>, max_iterations: i32) {
    let node_count = route.len();
    let mut rng = rand::thread_rng();
    for _current_iterations in 0..max_iterations {
        let current_total_distance = total_distance(&route);
        let rand_index1: usize = rng.gen::<usize>()  % node_count;
        let rand_index2: usize = rng.gen::<usize>() % node_count;
        route.swap(rand_index1, rand_index2);
        let new_total_distance = total_distance(&route);
        if new_total_distance >= current_total_distance {
            route.swap(rand_index1, rand_index2);
        }
    }
}

fn read_input_file() -> Result<Vec<Node>, Box<dyn Error>> {
    let file = File::open(FILE_NAME).unwrap();
    let reader = BufReader::new(file);

    let mut is_cood_section = false;
    let mut nodes: Vec<Node> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.contains("NODE_COORD_SECTION") {
            is_cood_section = true;
            continue;
        }
        if is_cood_section {
            if line.contains("EOF") {
                break;
            }
            let record: Vec<&str> = line.split_whitespace().collect();
            let node = Node {
                id: record[0].parse()?,
                x: record[1].parse()?,
                y: record[2].parse()?,
            };
            nodes.push(node);
        }
    }
    Ok(nodes)
}
