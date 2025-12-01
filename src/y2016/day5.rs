use advent::prelude::*;

parse!();

fn is_zero(input: &str, index: usize) -> Option<[u8; 32]> {
    let mut result = [0; 32];
    hex::encode_to_slice(md5::compute(format!("{input}{index}")).0, &mut result).unwrap();
    (result[..5] == [b'0'; 5]).then_some(result)
}

pub fn part1(input: Input) -> impl Display {
    let mut result = String::new();
    for i in 0usize.. {
        if let Some(res) = is_zero(input, i) {
            result.push(res[5] as char);
            if result.len() == 8 {
                break;
            }
        }
    }
    result
}

pub fn part2(input: Input) -> impl Display {
    let mut result = [None; 8];
    for i in 0usize.. {
        if let Some(res) = is_zero(input, i) {
            let pos = res[5];
            if pos < b'0' || pos > b'7' {
                continue;
            }
            let pos = pos - b'0';
            if result[pos as usize].is_none() {
                result[pos as usize] = Some(res[6]);
            }
            if result.iter().all(|x| x.is_some()) {
                break;
            }
        }
    }
    result.into_iter().map(|x| x.unwrap() as char).collect::<String>()
}
