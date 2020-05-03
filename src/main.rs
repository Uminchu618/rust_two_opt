mod plot;

use rand::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

const FILE_NAME: &str = "att48.tsp";

#[derive(Debug, Clone, Copy)]
pub struct Node {
    pub id: i32,
    pub x: f64,
    pub y: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let mut route = read_input_file()?;
    println!("random {:?}", total_distance(&route));
    plot::plot_route(&route, "random.png", "random")?;
    nearest_neighbor(&mut route, &mut rng);
    println!("nearest_neighbor {:?}", total_distance(&route));
    plot::plot_route(&route, "nearest_neighbor.png", "nearest neighbor")?;
    let mut nr_route = route.clone();
    hillclimb(&mut route, 100000, &mut rng);
    println!("hillclimb {:?}", total_distance(&route));
    plot::plot_route(&route, "hillclimb.png", "hillclimb")?;
    two_opt(&mut nr_route, 100000);
    println!("two_opt {:?}", total_distance(&nr_route));
    plot::plot_route(&nr_route, "two_opt.png", "two opt")?;
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
fn hillclimb(route: &mut Vec<Node>, max_iterations: i32, rng: &mut rand::rngs::ThreadRng) {
    let node_count = route.len();
    for _current_iterations in 0..max_iterations {
        let current_total_distance = total_distance(&route);
        let rand_index1: usize = rng.gen::<usize>() % node_count;
        let rand_index2: usize = rng.gen::<usize>() % node_count;
        route.swap(rand_index1, rand_index2);
        let new_total_distance = total_distance(&route);
        if new_total_distance >= current_total_distance {
            route.swap(rand_index1, rand_index2);
        }
    }
}

fn two_opt(route: &mut Vec<Node>, max_iterations: i32) {
    let count = route.len();
    'outer: for _current_iterations in 0..max_iterations {
        for i in 0..count {
            for j in (i + 2)..count {
                let ni = i + 1;
                let nj = j + 1;
                let original_dist = distance(&route[i], &route[ni % count])
                    + distance(&route[j], &route[nj % count]);
                let changed_dist = distance(&route[i], &route[j])
                    + distance(&route[ni % count], &route[nj % count]);
                if changed_dist < original_dist {
                    // let org_total = total_distance(&route);
                    let swap_count = ((j as f32 - ni as f32) / 2.0 + 0.5) as usize;
                    for offset in 0..swap_count {
                        route.swap((ni + offset) % count, (j - offset) % count);
                    }
                    // let new_total = total_distance(&route);
                    // if org_total <= new_total {
                    //     panic!("okasi");
                    // }
                    continue 'outer;
                }
            }
        }
        break;
    }
}

fn nearest_neighbor(route: &mut Vec<Node>, rng: &mut rand::rngs::ThreadRng) {
    let count = route.len();
    let first_index = (rng.next_u32() as usize) % count;
    route.swap(0, first_index);
    for i in 1..count {
        let mut min_distance: f64 = std::f64::MAX;
        let mut min_index: usize = i;
        for j in i..count {
            let temp_distance = distance(&route[i - 1], &route[j]);
            if temp_distance < min_distance {
                min_distance = temp_distance;
                min_index = j;
            }
        }
        route.swap(i, min_index);
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
