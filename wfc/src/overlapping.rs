use coord_2d::{Coord, Size};
use direction::{CardinalDirection, CardinalDirectionTable, CardinalDirections};
use grid_2d::coord_system::XThenYIter;
use grid_2d::Grid;
use hashbrown::HashMap;
use orientation::{self, Orientation};
use std::hash::Hash;
use std::num::NonZeroU32;
use tiled_grid_slice::TiledGridSlice;
use wfc::{GlobalStats, PatternDescription, PatternId, PatternTable};

fn are_patterns_compatible<T: PartialEq>(
    a: &TiledGridSlice<T>,
    b: &TiledGridSlice<T>,
    b_offset_direction: CardinalDirection,
) -> bool {
    let size = a.size();
    assert!(size == b.size());
    if size.x() == 1 {
        // patterns don't overlap, so everything is compatible
        return true;
    }
    let axis = b_offset_direction.axis();
    let compare_size = size.with_axis(axis, |d| d - 1);
    let (a_offset, b_offset) = match b_offset_direction {
        CardinalDirection::North => (Coord::new(0, 0), Coord::new(0, 1)),
        CardinalDirection::South => (Coord::new(0, 1), Coord::new(0, 0)),
        CardinalDirection::East => (Coord::new(1, 0), Coord::new(0, 0)),
        CardinalDirection::West => (Coord::new(0, 0), Coord::new(1, 0)),
    };
    let coords = || XThenYIter::new(compare_size);
    let a_iter = coords().map(|c| a.get_checked(c + a_offset));
    let b_iter = coords().map(|c| b.get_checked(c + b_offset));
    a_iter.zip(b_iter).all(|(a, b)| a == b)
}

#[derive(Debug)]
pub struct Pattern {
    id: PatternId,
    count: u32,
    samples: Vec<(Coord, Orientation)>,
}

impl Pattern {
    fn tiled_grid_slice<'a, T>(
        &self,
        grid: &'a Grid<T>,
        size: Size,
    ) -> TiledGridSlice<'a, T> {
        let (coord, orientation) = self.samples[0];
        TiledGridSlice::new(grid, coord, size, orientation)
    }
    pub fn coord(&self) -> Coord {
        self.samples[0].0
    }
    pub fn clear_count(&mut self) {
        self.count = 0;
    }
}

pub struct OverlappingPatterns<T: Eq + Clone + Hash> {
    pattern_table: PatternTable<Pattern>,
    pattern_size: Size,
    grid: Grid<T>,
    id_grid: Grid<(PatternId, Orientation)>,
}

impl<T: Eq + Clone + Hash> OverlappingPatterns<T> {
    pub fn new(
        grid: Grid<T>,
        pattern_size: NonZeroU32,
        orientations: &[Orientation],
    ) -> Self {
        let pattern_size = Size::new(pattern_size.get(), pattern_size.get());
        let (id_grid, pattern_table) = {
            let mut next_id = 0;
            let mut pattern_map: HashMap<TiledGridSlice<T>, Pattern> = HashMap::new();
            let id_grid = Grid::new_fn(grid.size(), |coord| {
                let mut id_and_orientation = None;
                for &orientation in orientations.iter() {
                    let pattern_slice =
                        TiledGridSlice::new(&grid, coord, pattern_size, orientation);
                    if let Some(pattern) = pattern_map.get_mut(&pattern_slice) {
                        pattern.count += 1;
                        pattern.samples.push((coord, orientation));
                        id_and_orientation = Some((pattern.id, orientation));
                        break;
                    }
                }
                if let Some(id_and_orientation) = id_and_orientation {
                    id_and_orientation
                } else {
                    let orientation = Orientation::Original;
                    let pattern_slice =
                        TiledGridSlice::new(&grid, coord, pattern_size, orientation);
                    let id = next_id;
                    next_id += 1;
                    pattern_map.insert(
                        pattern_slice,
                        Pattern {
                            id,
                            count: 1,
                            samples: vec![(coord, orientation)],
                        },
                    );
                    (id, orientation)
                }
            });
            let mut patterns = pattern_map
                .drain()
                .map(|(_, pattern)| pattern)
                .collect::<Vec<_>>();
            patterns.sort_by_key(|pattern| pattern.id);
            (id_grid, PatternTable::from_vec(patterns))
        };
        Self {
            grid,
            id_grid,
            pattern_size,
            pattern_table,
        }
    }
    pub fn new_all_orientations(grid: Grid<T>, pattern_size: NonZeroU32) -> Self {
        Self::new(grid, pattern_size, &orientation::ALL)
    }
    pub fn new_original_orientation(grid: Grid<T>, pattern_size: NonZeroU32) -> Self {
        Self::new(grid, pattern_size, &[Orientation::Original])
    }
    pub fn grid(&self) -> &Grid<T> {
        &self.grid
    }
    pub fn pattern(&self, pattern_id: PatternId) -> &Pattern {
        &self.pattern_table[pattern_id]
    }
    pub fn pattern_mut(&mut self, pattern_id: PatternId) -> &mut Pattern {
        &mut self.pattern_table[pattern_id]
    }
    pub fn pattern_top_left_value(&self, pattern_id: PatternId) -> &T {
        let pattern = self.pattern(pattern_id);
        let tiled_grid_slice = pattern.tiled_grid_slice(&self.grid, self.pattern_size);
        tiled_grid_slice.get_checked(Coord::new(0, 0))
    }
    fn compatible_patterns<'b>(
        &'b self,
        pattern: &'b Pattern,
        direction: CardinalDirection,
    ) -> impl 'b + Iterator<Item = PatternId> {
        let tiled_grid_slice = pattern.tiled_grid_slice(&self.grid, self.pattern_size);
        self.pattern_table
            .enumerate()
            .filter(move |(_id, other)| {
                let other_tiled_grid_slice =
                    other.tiled_grid_slice(&self.grid, self.pattern_size);
                are_patterns_compatible(
                    &tiled_grid_slice,
                    &other_tiled_grid_slice,
                    direction,
                )
            })
            .map(|(id, _other)| id)
    }
    pub fn pattern_descriptions(&self) -> PatternTable<PatternDescription> {
        self.pattern_table
            .iter()
            .map(|pattern| {
                let weight = NonZeroU32::new(pattern.count);
                let mut allowed_neighbours = CardinalDirectionTable::default();
                for direction in CardinalDirections {
                    allowed_neighbours[direction] = self
                        .compatible_patterns(pattern, direction)
                        .collect::<Vec<_>>();
                }
                PatternDescription::new(weight, allowed_neighbours)
            })
            .collect::<PatternTable<_>>()
    }
    pub fn global_stats(&self) -> GlobalStats {
        GlobalStats::new(self.pattern_descriptions())
    }
    pub fn id_grid(&self) -> &Grid<(PatternId, Orientation)> {
        &self.id_grid
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use coord_2d::{Coord, Size};
    use direction::CardinalDirection;
    use grid_2d::Grid;
    use orientation::Orientation;

    fn pattern_with_coord(coord: Coord) -> Pattern {
        Pattern {
            id: 0,
            count: 0,
            samples: vec![(coord, Orientation::Original)],
        }
    }

    #[test]
    fn compatibile_patterns() {
        let r = 0;
        let b = 1;
        let array = [[r, b, b], [b, r, b]];
        let grid = Grid::new_fn(Size::new(3, 2), |coord| {
            array[coord.y as usize][coord.x as usize]
        });
        let pattern_size = Size::new(2, 2);
        assert!(are_patterns_compatible(
            &pattern_with_coord(Coord::new(0, 0)).tiled_grid_slice(&grid, pattern_size),
            &pattern_with_coord(Coord::new(1, 0)).tiled_grid_slice(&grid, pattern_size),
            CardinalDirection::East,
        ));
        assert!(are_patterns_compatible(
            &pattern_with_coord(Coord::new(0, 0)).tiled_grid_slice(&grid, pattern_size),
            &pattern_with_coord(Coord::new(1, 0)).tiled_grid_slice(&grid, pattern_size),
            CardinalDirection::North,
        ));
        assert!(!are_patterns_compatible(
            &pattern_with_coord(Coord::new(0, 0)).tiled_grid_slice(&grid, pattern_size),
            &pattern_with_coord(Coord::new(1, 0)).tiled_grid_slice(&grid, pattern_size),
            CardinalDirection::South,
        ));
        assert!(!are_patterns_compatible(
            &pattern_with_coord(Coord::new(0, 0)).tiled_grid_slice(&grid, pattern_size),
            &pattern_with_coord(Coord::new(1, 0)).tiled_grid_slice(&grid, pattern_size),
            CardinalDirection::West,
        ));
    }
}
