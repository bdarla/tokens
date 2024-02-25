use graph::prelude::*;

fn main() {
    // https://en.wikipedia.org/wiki/PageRank#/media/File:PageRanks-Example.svg
    let graph: DirectedCsrGraph<usize> = GraphBuilder::new()
        .edges(vec![
            (1, 2),  // B->C
            (2, 1),  // C->B
            (4, 0),  // D->A
            (4, 1),  // D->B
            (5, 4),  // E->D
            (5, 1),  // E->B
            (5, 6),  // E->F
            (6, 1),  // F->B
            (6, 5),  // F->E
            (7, 1),  // G->B
            (7, 5),  // F->E
            (8, 1),  // G->B
            (8, 5),  // G->E
            (9, 1),  // H->B
            (9, 5),  // H->E
            (10, 1), // I->B
            (10, 5), // I->E
            (11, 5), // J->B
            (12, 5), // K->B
        ])
        .build();

    let (ranks, iterations, _) = page_rank(&graph, PageRankConfig::new(10, 1E-4, 0.85));

    assert_eq!(iterations, 10);

    let expected = vec![
        0.024064068,
        0.3145448,
        0.278901528,
        0.01153846,
        0.029471997,
        0.06329483,
        0.029471997,
        0.01153846,
        0.01153846,
        0.01153846,
        0.01153846,
        0.01153846,
        0.01153846,
    ];

    assert_eq!(ranks, expected);
    println!("{:?}", ranks);
}
