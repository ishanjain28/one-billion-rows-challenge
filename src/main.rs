use fxhash::FxHashMap;
use rayon::prelude::*;
use std::{fs::File, io::Read, time::Instant};

fn main() {
    let mut buf = Vec::new();

    let t1 = Instant::now();

    let mut file = File::open("./measurements.txt").expect("error in opening file");

    file.read_to_end(&mut buf)
        .expect("error in reading file to memory");

    let t1_elapsed = t1.elapsed();

    let t2 = Instant::now();

    let map: FxHashMap<&[u8], (i16, i32, i16, usize)> = buf
        .par_split(|&x| x == b'\n')
        .filter_map(|line| {
            let l = line.len();
            if l == 0 {
                return None;
            }

            let loc = if line[l - 6] == b';' {
                l - 6
            } else if line[l - 5] == b';' {
                l - 5
            } else if line[l - 4] == b';' {
                l - 4
            } else {
                unreachable!();
            };

            let (city, val) = line.split_at(loc);
            let val = &val[1..];

            let val = parse_float(val);

            Some((city, val))
        })
        .fold(FxHashMap::default, |mut a, (city, val)| {
            let (min, sum, max, counter) = a
                .entry(city)
                .or_insert_with(|| (std::i16::MAX, 0, std::i16::MIN, 0usize));

            *min = val.min(*min);
            *max = val.max(*max);
            *counter += 1;
            *sum += val as i32;

            a
        })
        .reduce_with(|mut m1, m2| {
            for (k, (min1, sum1, max1, counter1)) in m2 {
                let (min, sum, max, counter) = m1
                    .entry(k)
                    .or_insert_with(|| (std::i16::MAX, 0, std::i16::MIN, 0usize));

                *min = min1.min(*min);
                *max = max1.max(*max);
                *counter += counter1;
                *sum += sum1;
            }

            m1
        })
        .unwrap();

    let t2_elapsed = t2.elapsed();

    let t3 = Instant::now();

    let mut list: Vec<(&[u8], (i16, i32, i16, usize))> = map.into_iter().collect();
    list.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));

    let mut output = String::new();

    let l = list.len();

    for (i, (city, (min, sum, max, counter))) in list.into_iter().enumerate() {
        output.push_str(&format!(
            "{}={:.1}/{:.1}/{:.1}{}",
            String::from_utf8_lossy(city),
            (min as f32 / 10.0),
            (sum as f32 / 10.0) / counter as f32,
            (max as f32 / 10.0),
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
fn parse_float(b: &[u8]) -> i16 {
    let l = b.len();
    let mut num: i16 = 0;
    let mut is_negative = false;

    let mut i = 0;
    while unsafe { *b.get_unchecked(i) } != b'.' {
        match unsafe { *b.get_unchecked(i) } {
            v @ b'0'..=b'9' => {
                num *= 10;
                num += (v - b'0') as i16;
            }
            b'-' => is_negative = true,
            b'.' => break,
            _ => (),
        }
        i += 1;
    }

    num *= 10;
    num += (b[l - 1] - b'0') as i16;

    if is_negative {
        num *= -1;
    }

    num
}
