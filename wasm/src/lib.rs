use aho_corasick::AhoCorasick;
use icu::segmenter::SentenceSegmenter;
use js_sys::Array;
use lazy_static::lazy_static;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

lazy_static! {
    // https://github.com/unicode-org/cldr/blob/90aaf561d1827ce06b1e9c4173a70c018665c7d2/common/segments/en.xml
    static ref ABBREVIATIONS: AhoCorasick = AhoCorasick::new([
        "L.P.", "Alt.", "Approx.", "E.G.", "O.", "Maj.", "Misc.", "P.O.", "J.D.", "Jam.", "Card.",
        "Dec.", "Sept.", "MR.", "Long.", "Hat.", "G.", "Link.", "DC.", "D.C.", "M.T.", "Hz.",
        "Mrs.", "By.", "Act.", "Var.", "N.V.", "Aug.", "B.", "S.A.", "Up.", "Job.", "Num.",
        "M.I.T.", "Ok.", "Org.", "Ex.", "Cont.", "U.", "Mart.", "Fn.", "Abs.", "Lt.", "OK.", "Z.",
        "E.", "Kb.", "Est.", "A.M.", "L.A.", "Prof.", "U.S.", "Nov.", "Ph.D.", "Mar.", "I.T.",
        "exec.", "Jan.", "N.Y.", "X.", "Md.", "Op.", "vs.", "D.A.", "A.D.", "R.L.", "P.M.", "Or.",
        "M.R.", "Cap.", "PC.", "Feb.", "Exec.", "I.e.", "Sep.", "Gb.", "K.", "U.S.C.", "Mt.", "S.",
        "A.S.", "C.O.D.", "Capt.", "Col.", "In.", "C.F.", "Adj.", "AD.", "I.D.", "Mgr.", "R.T.",
        "B.V.", "M.", "Conn.", "Yr.", "Rev.", "Phys.", "pp.", "Ms.", "To.", "Sgt.", "J.K.", "Nr.",
        "Jun.", "Fri.", "S.A.R.", "Lev.", "Lt.Cdr.", "Def.", "F.", "Do.", "Joe.", "Id.", "Mr.",
        "Dept.", "Is.", "Pvt.", "Diff.", "Hon.B.A.", "Q.", "Mb.", "On.", "Min.", "J.B.", "Ed.",
        "AB.", "A.", "S.p.A.", "I.", "a.ahc.", "Comm.", "Go.", "VS.", "L.", "All.", "PP.", "P.V.",
        "T.", "K.R.", "Etc.", "D.", "Adv.", "Lib.", "E.g.", "Pro.", "U.S.A.", "S.E.", "AA.",
        "Rep.", "Sq.", "As.", "Dr.",
    ]);
}

#[derive(Debug, Clone, Default)]
pub struct Segment<'a> {
    pub start: usize,
    pub end: usize,
    pub text: &'a str,
}

fn get_segments(input: &str, breakpoints: Vec<usize>) -> Vec<Segment> {
    breakpoints
        .windows(2)
        .map(|range| Segment {
            start: range[0],
            end: range[1],
            text: &input[range[0]..range[1]],
        })
        .collect()
}

fn sentence_breakpoints(input: &str) -> Vec<usize> {
    let segmenter =
        SentenceSegmenter::try_new_unstable(&icu_testdata::unstable()).expect("Data exists");
    segmenter.segment_str(input).collect()
}

#[wasm_bindgen]
pub fn sentences(input: &str) -> Array {
    if input.is_empty() {
        return Default::default();
    }
    let end_preserved: HashSet<usize> = ABBREVIATIONS
        .find_overlapping_iter(input)
        .map(|m| (m.end()))
        .collect();
    let icu_points = sentence_breakpoints(input);
    let mut breakpoints = vec![0];
    let mut breakpoint;

    for i in 0..icu_points.len() - 1 {
        let start = icu_points[i];
        let mut end = icu_points[i + 1];
        breakpoint = end;
        while end > start && input.as_bytes()[end - 1].is_ascii_whitespace() {
            end -= 1;
        }
        if !end_preserved.contains(&end) {
            breakpoints.push(breakpoint);
            continue;
        }
        if i == icu_points.len() - 2 {
            breakpoints.push(breakpoint);
            break;
        }
    }
    let segments = get_segments(input, breakpoints);
    let mut output = Vec::new();
    for segment in segments {
        if !output.is_empty() && segment.text == ".\n" {
            let last: &mut Segment = output.last_mut().unwrap();
            *last = Segment {
                start: last.start,
                end: segment.end,
                text: &input[last.start..segment.end],
            }
        } else {
            output.push(segment)
        }
    }
    let segments = output;
    let arr = Array::new_with_length(segments.len() as u32);
    for i in 0..arr.length() {
        let s = JsValue::from_str(segments[i as usize].text);
        arr.set(i, s);
    }
    arr
}