use std::io::Read;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Free,
    Occupied(usize),
}

#[derive(Debug, Clone, Copy)]
struct FreeSegment {
    start_index: usize,
    length: usize,
}

#[derive(Debug, Copy, Clone)]
struct FileSegment {
    start_index: usize,
    length: usize,
    id: usize,
}

#[derive(Debug, Clone)]
struct FreeMap {
    segments: Vec<FreeSegment>,
}

impl FreeMap {
    fn new() -> Self {
        Self { segments: vec![] }
    }

    fn push(&mut self, segment: FreeSegment) {
        self.segments.push(segment);
    }
}

#[derive(Debug, Clone)]
struct Tape {
    cells: Vec<Cell>,
    free_map: FreeMap,
    files: Vec<FileSegment>,
}

impl Tape {
    fn from_str(s: &str) -> Self {
        let mut id = 0;
        let mut idx = 0;
        let mut free_map = FreeMap::new();
        let mut files = vec![];
        let cells: Vec<_> = s
            .chars()
            .enumerate()
            .flat_map(|(i, c)| {
                let len: usize = c.to_digit(10).unwrap() as usize;
                let cell = if i % 2 == 0 {
                    assert!(len > 0);
                    let cell = Cell::Occupied(id);
                    let file = FileSegment {
                        start_index: idx,
                        length: len,
                        id,
                    };
                    files.push(file);
                    id += 1;
                    cell
                } else {
                    if len > 0 {
                        let free_segment = FreeSegment {
                            start_index: idx,
                            length: len,
                        };
                        free_map.push(free_segment);
                    }
                    Cell::Free
                };
                idx += len;
                vec![cell; len]
            })
            .collect();
        Tape {
            cells,
            free_map,
            files,
        }
    }

    fn defragment_blocks(&self) -> Self {
        let mut new = self.clone();
        let mut free = new
            .cells
            .iter()
            .position(|f| matches!(f, Cell::Free))
            .unwrap();
        let mut filled = new
            .cells
            .iter()
            .rposition(|f| matches!(f, Cell::Occupied(_)))
            .unwrap();
        while free < filled {
            new.cells[free] = new.cells[filled].clone();
            new.cells[filled] = Cell::Free;
            while matches!(new.cells[free], Cell::Occupied(_)) {
                free += 1
            }
            while matches!(new.cells[filled], Cell::Free) {
                filled -= 1;
            }
        }
        new
    }

    fn defragment_files(&self) -> Self {
        let mut new = self.clone();
        new.files.reverse();
        for segment in new.files.iter() {
            if let Some(target) = new
                .free_map
                .segments
                .iter_mut()
                .find(|s| s.start_index <= segment.start_index && s.length >= segment.length)
            {
                for i in 0..segment.length {
                    assert_eq!(self.cells[target.start_index + i], Cell::Free);
                    assert_eq!(
                        self.cells[segment.start_index + i],
                        Cell::Occupied(segment.id)
                    );
                    new.cells[target.start_index + i] = Cell::Occupied(segment.id);
                    new.cells[segment.start_index + i] = Cell::Free;
                }
                target.length -= segment.length;
                target.start_index += segment.length;
            }
        }
        new
    }

    fn checksum(&self) -> u64 {
        self.cells
            .iter()
            .enumerate()
            .map(|(i, c)| {
                if let Cell::Occupied(v) = c {
                    (i * v) as u64
                } else {
                    0
                }
            })
            .sum()
    }
}

fn main() -> anyhow::Result<()> {
    let mut stdin = std::io::stdin().lock();
    let mut s = String::new();
    stdin.read_to_string(&mut s)?;
    let tape = Tape::from_str(s.trim());
    println!("part 1: {}", tape.defragment_blocks().checksum());
    println!("part 2: {}", tape.defragment_files().checksum());
    Ok(())
}
