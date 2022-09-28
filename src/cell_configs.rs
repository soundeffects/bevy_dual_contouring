use array_macro::array;
use crate::dc::Edge;

// CELL_CONFIGS stores different cell configurations as suggested by Nielson.
// Each cell has 8 corners. Each corner can be either inside (true) or outside (false).
// This results in 256 different configurations. Due to symmetry those can be reduced to 23
// distinct configs.

// For each corner config, return all edges, that are connected to a single point. These egdes are
// stored in a BitSet. Since there might be more than 1 point, store  a slice of BitSets.

/*
 *
 */
pub const CELL_CONFIGS: [&[BitSet]; 256] = array![
    cell_corners => get_edges_for_cell_config(cell_corners as u8);
    256
];


// Following is the code used to generate this table.

//  Corner indexes
//
//      6---------------7
//     /|              /|
//    / |             / |
//   /  |            /  |
//  4---------------5   |
//  |   |           |   |
//  |   2-----------|---3
//  |  /            |  /
//  | /             | /
//  |/              |/
//  0---------------1
#[derive(Clone, Copy)]
pub enum Corner {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}
// // Corner connections
pub const CORNER_CONNS: [[Corner; 3]; 8] = [
    [Corner::B, Corner::C, Corner::E],
    [Corner::A, Corner::D, Corner::F],
    [Corner::A, Corner::D, Corner::G],
    [Corner::B, Corner::C, Corner::H],
    [Corner::A, Corner::F, Corner::G],
    [Corner::B, Corner::E, Corner::H],
    [Corner::C, Corner::E, Corner::H],
    [Corner::D, Corner::F, Corner::G],
];

// Which corners does a edge connect:
pub const EDGE_DEF: [(Corner, Corner); 12] = [
    (Corner::A, Corner::B),
    (Corner::A, Corner::C),
    (Corner::A, Corner::E),
    (Corner::C, Corner::D),
    (Corner::B, Corner::D),
    (Corner::B, Corner::F),
    (Corner::E, Corner::F),
    (Corner::E, Corner::G),
    (Corner::C, Corner::G),
    (Corner::G, Corner::H),
    (Corner::F, Corner::H),
    (Corner::D, Corner::H),
];

// Return a list of a set of edges for a cell config. E.g. which edges are connected to
// each other for that cell config.
fn get_edges_for_cell_config(corners: u8) -> Vec<BitSet> {
    let cell = BitSet::from_u32(u32::from(corners));
    // Handle special case
    if let Some(special) = get_connected_edges_for_diagonal_case(cell) {
        return special;
    }
    let mut result = Vec::new();
    let mut visited_corners = BitSet::zero();
    for corner in cell {
        let connected_corners = visit_all_corners(corner, cell, &mut visited_corners);
        if !connected_corners.empty() {
            result.push(connected_corners);
        }
    }
    result
}

fn get_edge_from_corners(a: usize, b: usize) -> Edge {
    for (edge, &(x, y)) in EDGE_DEF.iter().enumerate() {
        if (a == x as usize && b == y as usize) || (a == y as usize && b == x as usize) {
            return Edge::from_usize(edge);
        }
    }
    panic!("could not find edge for {:?} - {:?}", a, b);
}

fn visit_all_corners(corner: usize, cell: BitSet, visited_corners: &mut BitSet) -> BitSet {
    if visited_corners.get(corner) {
        // We already visited the current corner
        return BitSet::zero();
    }
    // Mark the current corner as visited.
    visited_corners.set(corner);
    let mut result = BitSet::zero();
    for adjacent_corner_ref in &CORNER_CONNS[corner] {
        let adjacent_corner = *adjacent_corner_ref as usize;
        if cell.get(adjacent_corner) {
            result = result.merge(visit_all_corners(adjacent_corner, cell, visited_corners));
        } else {
            let edge = get_edge_from_corners(corner, adjacent_corner);
            result.set(edge as usize)
        }
    }
    result
}

fn bitset_from_edges(edges: [Edge; 3]) -> BitSet {
    let mut bs = BitSet::zero();
    for edge in &edges {
        bs.set(*edge as usize);
    }
    bs
}

fn get_connected_edges_for_diagonal_case(cell: BitSet) -> Option<Vec<BitSet>> {
    if cell.count() == 6 {
        let inv = cell.invert();
        let lowest = inv.lowest().unwrap();
        if inv.get(7 - lowest) {
            return Some(match lowest {
                0 => vec![
                    bitset_from_edges([Edge::A, Edge::B, Edge::C]),
                    bitset_from_edges([Edge::J, Edge::K, Edge::L]),
                ],
                1 => vec![
                    bitset_from_edges([Edge::A, Edge::E, Edge::F]),
                    bitset_from_edges([Edge::H, Edge::I, Edge::J]),
                ],
                2 => vec![
                    bitset_from_edges([Edge::B, Edge::D, Edge::I]),
                    bitset_from_edges([Edge::F, Edge::G, Edge::K]),
                ],
                3 => vec![
                    bitset_from_edges([Edge::D, Edge::E, Edge::L]),
                    bitset_from_edges([Edge::C, Edge::G, Edge::H]),
                ],
                x => panic!("diagonal case {:?} with lowest corner {:?}", cell, x),
            });
        }
    }
    None
}
#[cfg(test)]
mod test {
    #[test]
    fn correct_number_of_cell_configs() {
    }

    #[test]
    fn correct_cell_configs_when_sampled() {
    }
}
