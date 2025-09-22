use plotiron::figure;

fn main() {
    let graph_input = r#"digraph Tree {
       "tearRate_0" [label = "tearRate"];
       "astigmatic_1" [label = "astigmatic"];
       "tearRate_0" -> "astigmatic_1" [label = "normal"];
       "astigmatic_1" [label = "astigmatic"];
       "age_2" [label = "age"];
       "astigmatic_1" -> "age_2" [label = "no"];
       "age_2" [label = "age"];
       "soft_292" [label = "soft"];
       "age_2" -> "soft_292" [label = "young"];
       "soft_368" [label = "soft"];
       "age_2" -> "soft_368" [label = "pre"];
       "prescript_3" [label = "prescript"];
       "age_2" -> "prescript_3" [label = "presbyopic"];
       "prescript_3" [label = "prescript"];
       "soft_574" [label = "soft"];
       "prescript_3" -> "soft_574" [label = "hyper"];
       "no lenses_656" [label = "no lenses"];
       "prescript_3" -> "no lenses_656" [label = "myope"];
       "prescript_2" [label = "prescript"];
       "astigmatic_1" -> "prescript_2" [label = "yes"];
       "prescript_2" [label = "prescript"];
       "age_3" [label = "age"];
       "prescript_2" -> "age_3" [label = "hyper"];
       "age_3" [label = "age"];
       "hard_988" [label = "hard"];
       "age_3" -> "hard_988" [label = "young"];
       "no lenses_1064" [label = "no lenses"];
       "age_3" -> "no lenses_1064" [label = "pre"];
       "no lenses_1155" [label = "no lenses"];
       "age_3" -> "no lenses_1155" [label = "presbyopic"];
       "hard_1253" [label = "hard"];
       "prescript_2" -> "hard_1253" [label = "myope"];
       "no lenses_1337" [label = "no lenses"];
       "tearRate_0" -> "no lenses_1337" [label = "reduced"];
    }"#;

    let gi = r#"digraph {
        a -> b;
        }"#;

    let mut fig = figure();
    fig.add_dot_subplot(gi).unwrap();
    fig.show();
}
