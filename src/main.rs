use fxhash::FxHashMap;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

fn main() {
    let mut map = FxHashMap::default();
    let mut buf = Vec::new();

    let t1 = Instant::now();

    let mut file = File::open("./measurements.txt").expect("error in opening file");

    file.read_to_end(&mut buf)
        .expect("error in reading file to memory");

    let t1_elapsed = t1.elapsed();

    let t2 = Instant::now();

    for line in buf.split(|&x| x == b'\n') {
        let l = line.len();
        if l == 0 {
            continue;
        }

        let loc = if line[l - 4] == b';' {
            l - 4
        } else if line[l - 5] == b';' {
            l - 5
        } else if line[l - 6] == b';' {
            l - 6
        } else {
            unreachable!();
        };

        let (city, val) = line.split_at(loc);
        let val = &val[1..];

        let val = parse_float(val);

        let (min, sum, max, counter) =
            map.entry(city)
                .or_insert((std::f32::MAX, 0.0, std::f32::MIN, 0));

        *min = val.min(*min);
        *max = val.max(*max);
        *counter += 1;
        *sum += val;
    }

    let t2_elapsed = t2.elapsed();

    let t3 = Instant::now();

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

    let t3_elapsed = t3.elapsed();

    println!("{{{output}}}");

    eprintln!(
        "read = {:?} processed = {:?} output_gen = {:?}",
        t1_elapsed, t2_elapsed, t3_elapsed
    );
}

#[inline]
fn parse_float(b: &[u8]) -> f32 {
    let l = b.len();
    let mut num: i32 = 0;
    let mut is_negative = false;

    for &c in b.iter().take_while(|&&x| x != b'.') {
        match c {
            v @ b'0'..=b'9' => {
                num *= 10;
                num += (v - b'0') as i32;
            }
            b'-' => is_negative = true,
            b'.' => continue,

            _ => unreachable!(),
        }
    }

    let mut num = num as f32;

    num += 0.1 * (b[l - 1] - b'0') as f32;
    if is_negative {
        num *= -1.0;
    }

    num * 10.0 / 10.0
}
