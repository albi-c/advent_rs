use advent::prelude::*;

#[derive(Debug)]
pub enum Segment {
    Br(String),
    NoBr(String),
}

parse!(Vec<Vec<Segment>>, lines({
    '[' s:string(alpha+) ']' => Segment::Br(s),
    s:string(alpha+) => Segment::NoBr(s),
}+));

fn has_abba(string: &str) -> bool {
    string.chars().map_windows(|window: &[char; 4]| window[0] == window[3] && window[1] == window[2] && window[0] != window[1]).any(|m| m)
}

fn check_segments(segments: &[Segment]) -> bool {
    if segments.iter().any(|seg| match seg {
        Segment::Br(_) => false,
        Segment::NoBr(s) => has_abba(s),
    }) {
        segments.iter().all(|seg| match seg {
            Segment::Br(s) => !has_abba(s),
            Segment::NoBr(_) => true,
        })
    } else {
        false
    }
}

pub fn part1(input: Input) -> impl Display {
    input
        .into_iter()
        .filter(|seg| check_segments(seg))
        .count()
}

fn check_segments_2(segments: &[Segment]) -> bool {
    for seg in segments {
        if let Segment::NoBr(s) = seg {
            let m = s.chars()
                .map_windows(|window: &[char; 3]| (*window, window[0] == window[2] && window[0] != window[1]))
                .filter(|(_, m)| *m)
                .find(|(w, _)| segments.iter()
                    .find(|seg| match seg {
                        Segment::NoBr(_) => false,
                        Segment::Br(s) => s.contains(&format!("{0}{1}{0}", w[1], w[0])),
                    })
                    .is_some());
            if m.is_some() {
                return true;
            }
        }
    }
    false
}

pub fn part2(input: Input) -> impl Display {
    input
        .into_iter()
        .filter(|seg| check_segments_2(seg))
        .count()
}
