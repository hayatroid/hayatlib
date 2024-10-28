use proconio::source::Readable;

pub enum Query {
    // ここを書き換える
    Q1(String),
    Q2,
    Q3,
}

impl Readable for Query {
    type Output = Query;
    fn read<R: std::io::BufRead, S: proconio::source::Source<R>>(source: &mut S) -> Self::Output {
        match u8::read(source) {
            // ここを書き換える
            1 => Query::Q1(String::read(source)),
            2 => Query::Q2,
            3 => Query::Q3,
            _ => unreachable!(),
        }
    }
}
