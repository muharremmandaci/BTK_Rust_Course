use std::collections::HashSet;

fn main() {
    let mut data = HashSet::new();

    data.insert("gamma");
    data.insert("delta");

    println!("{:?}", data);

    data.insert("delta");

    println!("{:?}", data);

    let is_added = data.insert("alpha");

    if is_added {
        println!("alpha eklendi");
        println!("{:?}", data);
    }

    if !data.contains("phi") {
        println!("phi yok");
    }

    let is_removed_data = data.remove("alpha");

    if is_removed_data {
        println!("alpha kaldırıldı");
        println!("{:?}", data);

    }


    /*******/

    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();

    println!("{:?} kümesi, {:?} kümesinin alt kümesi mi: {}", _2_8, _1_10, _2_8.is_subset(&_1_10));
    println!("{:?} kümesi, {:?} kümesi ile ayrık mı: {}", _1_5, _6_10, _1_5.is_disjoint(&_6_10));
    println!("{:?} kümesi, {:?} kümesi ile ayrık mı: {}", _1_5, _1_10, _1_5.is_disjoint(&_1_10));
    println!("{:?} kümesi, {:?} kümesinin birleşimi: {:?}", _2_8, _6_10, _2_8.union(&_6_10));
}
