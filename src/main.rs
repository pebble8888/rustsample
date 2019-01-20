extern crate crossbeam;

fn main() {
    let mut sum0: u64 = 0;
    let mut sum1: u64 = 0;
    let mut sum2: u64 = 0;
    let mut sum3: u64 = 0;

    let v0: Vec<u64> = (0..4).collect();
    let v1: Vec<u64> = (4..8).collect();
    let v2: Vec<u64> = (8..12).collect();
    let v3: Vec<u64> = (12..16).collect();

    let mut units = Vec::new();
    units.push((v0, &mut sum0));
    units.push((v1, &mut sum1));
    units.push((v2, &mut sum2));
    units.push((v3, &mut sum3));

    crossbeam::scope(|spawner| {
        for unit in units.into_iter() {
            spawner.spawn(move || {
                let val: u64 = unit.0[0] + unit.0[1] + unit.0[2] + unit.0[3];
                *unit.1 = val; 
                println!("unit.0: {} {} {} {}", unit.0[0], unit.0[1], unit.0[2], unit.0[3]);
                println!("unit.1: {}", unit.1);
            });
        }
    });
    println!("sum0: {}", sum0);
    println!("sum1: {}", sum1);
    println!("sum2: {}", sum2);
    println!("sum3: {}", sum3);
    let sum = sum0 + sum1 + sum2 + sum3;
    println!("sum: {}", sum);
}

