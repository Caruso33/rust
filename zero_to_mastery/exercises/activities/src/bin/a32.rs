// Topic: Lifetimes & Structures
//
// Requirements:
// * Display just the names and titles of persons from the mock-data.csv file
// * The names & titles must be stored in a struct separately from the mock
//   data for potential later usage
// * None of the mock data may be duplicated in memory
//
// Notes:
// * The mock data has already been loaded with the include_str! macro, so all functionality
//   must be implemented using references/borrows
use std::iter::Zip;
const MOCK_DATA: &'static str = include_str!("mock-data.csv");

// FIXME
// TODO
// how to pass here a generic struct type?? is not possible?
// fn display_data<'a>(data: &(Titles + Names)) {
//     for n in data.inner.iter().take(5) {
//         println!("{:?}", n);
//     }
// }

fn display_names_titles(data: Zip<std::slice::Iter<'_, &str>, std::slice::Iter<'_, &str>>) {
    for (name, title) in data.take(5) {
        println!("{} {}", name, title);
    }
}

struct Names<'a> {
    inner: Vec<&'a str>,
}

struct Titles<'a> {
    inner: Vec<&'a str>,
}

fn main() {
    let data: Vec<_> = MOCK_DATA.split('\n').skip(1).collect();

    let names = Names {
        inner: data
            .iter()
            .filter_map(|line| line.split(',').nth(1))
            .collect(),
    };

    let titles = Titles {
        inner: data
            .iter()
            .filter_map(|line| line.split(',').nth(4))
            .collect(),
    };

    // display_data(&names);
    // display_data(&titles);

    let names_titles = names.inner.iter().zip(titles.inner.iter());

    display_names_titles(names_titles);
}
