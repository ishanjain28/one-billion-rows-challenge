use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("./measurements.txt").expect("error in opening file");

    let mut map = HashMap::new();

    let mut buf = Vec::new();

    file.read_to_end(&mut buf)
        .expect("error in reading file to memory");

    for line in buf.split(|&x| x == b'\n') {
        let l = line.len();
        if l == 0 {
            continue;
        }

        let loc = line.iter().position(|&x| x == b';').unwrap();

        let (city, val) = line.split_at(loc);
        let val = &val[1..];

        let val = String::from_utf8_lossy(val);

        let val: f32 = val.parse().unwrap();

        let (min, sum, max, counter) =
            map.entry(city)
                .or_insert((std::f32::MAX, 0.0, std::f32::MIN, 0));

        *min = val.min(*min);
        *max = val.max(*max);
        *counter += 1;
        *sum += val;
    }

    let mut list: Vec<(&[u8], (f32, f32, f32, usize))> = map.into_iter().collect();
    list.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));

    let mut output = String::new();

    let l = list.len();

    for (i, (city, (min, mean, max, counter))) in list.into_iter().enumerate() {
        output.push_str(&format!(
            "{}={:.1}/{:.1}/{:.1}{}",
            String::from_utf8_lossy(city),
            min,
            mean / counter as f32,
            max,
            if i == l - 1 { "" } else { ", " }
        ));
    }

    println!("{{{output}}}");
}
