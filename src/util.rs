use std::{error, fs, io::{self, BufRead}};

use triangulate::Vertex;

#[derive(Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct VTest {
    x: f32,
    y: f32,
}

impl VTest {
    pub fn new(x: f32, y: f32) -> Self { VTest {x, y} }
}

impl std::fmt::Debug for VTest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::fmt::Display for VTest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Vertex for VTest {
    type Coordinate = f32;

    #[inline(always)]
    fn x(&self) -> Self::Coordinate { self.x }

    #[inline(always)]
    fn y(&self) -> Self::Coordinate { self.y }
}

impl Into<VTest> for (f32, f32) {
    fn into(self) -> VTest {
        VTest::new(self.0, self.1)
    }
}

pub fn load_polygon_list(path: &str) -> Result<Vec<Vec<VTest>>, Box<dyn error::Error>> {
    let mut output = Vec::new();
    let mut current = Vec::new();
    let f = fs::File::open(path)?;
    for line in io::BufReader::new(f).lines() {
        let line = line?;
        let mut chunks = line.split_ascii_whitespace();
        if let Some(x) = chunks.next() {
            let x = x.parse::<f32>()?;
            let y = chunks.next().ok_or(Box::new(io::Error::new(io::ErrorKind::InvalidData, "Invalid input file")))?.parse::<f32>()?;
            current.push((x, y).into());
        } else {
            let mut next = Vec::new();
            std::mem::swap(&mut current, &mut next);
            if next.len() > 0 {
                output.push(next);
            }
        }
    }

    Ok(output)
}