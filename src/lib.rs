//! Geometric types for 2D lattice-shaped puzzles.

use std::ops::{Add, Index, IndexMut, Mul, Neg, Range, Sub};

/// A two-dimensional lattice point.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Point(pub i32, pub i32);

/// A size of a rectangle.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Size(pub i32, pub i32);

/// A difference between two `Point`s.
///
/// `Point(y0, x0)` - `Point(y1, x1) == `Move(y0 - y1, x0 - x1)`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Move(pub i32, pub i32);

/// A 2x2 rotation matrix.
///
/// `Rotation(yy, yx, xy, xx) * Move(y, x) == Move(yy*y + yx*x, xy*y + xx*x)`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Rotation(i32, i32, i32, i32);

/// An upward `Move` vector.
pub const MOVE_UP: Move = Move(-1, 0);

/// A rightward `Move` vector.
pub const MOVE_RIGHT: Move = Move(0, 1);

/// A downward `Move` vector.
pub const MOVE_DOWN: Move = Move(1, 0);

/// A leftward `Move` vector.
pub const MOVE_LEFT: Move = Move(0, -1);

/// `Move` vectors that is toward four adjacent points.
pub const MOVE_ALL_DIRECTIONS: [Move; 4] = [MOVE_UP, MOVE_RIGHT, MOVE_DOWN, MOVE_LEFT];

/// `Move` vectors that is toward eight adjacent points.
pub const MOVE_ALL_ADJACENTS: [Move; 8] = [
    MOVE_UP,
    Move(-1, 1),
    MOVE_RIGHT,
    Move(1, 1),
    MOVE_DOWN,
    Move(1, -1),
    MOVE_LEFT,
    Move(-1, -1),
];

impl Add<Move> for Point {
    type Output = Point;

    #[inline]
    fn add(self, other: Move) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub<Point> for Point {
    type Output = Move;

    #[inline]
    fn sub(self, other: Point) -> Move {
        Move(self.0 - other.0, self.1 - other.1)
    }
}

impl Add<Move> for Move {
    type Output = Move;

    #[inline]
    fn add(self, other: Move) -> Move {
        Move(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub<Move> for Move {
    type Output = Move;

    #[inline]
    fn sub(self, other: Move) -> Move {
        Move(self.0 - other.0, self.1 - other.1)
    }
}

impl Neg for Move {
    type Output = Move;

    #[inline]
    fn neg(self) -> Move {
        Move(-self.0, -self.1)
    }
}

impl Mul<i32> for Move {
    type Output = Move;

    #[inline]
    fn mul(self, other: i32) -> Move {
        Move(self.0 * other, self.1 * other)
    }
}

/// A 0-degree `Rotation` to the left (counterclockwise).
pub const ROT_CCW0: Rotation = Rotation(1, 0, 0, 1);

/// A 90-degree `Rotation` to the left (counterclockwise).
pub const ROT_CCW90: Rotation = Rotation(0, -1, 1, 0);

/// A 180-degree `Rotation` to the left (counterclockwise).
pub const ROT_CCW180: Rotation = Rotation(-1, 0, 0, -1);

/// A 270-degree `Rotation` to the left (counterclockwise).
pub const ROT_CCW270: Rotation = Rotation(0, 1, -1, 0);

/// Flip horizontal.
pub const ROT_H_FLIP: Rotation = Rotation(1, 0, 0, -1);

/// Flip vertical.
pub const ROT_V_FLIP: Rotation = Rotation(-1, 0, 0, 1);

impl Mul<Rotation> for Rotation {
    type Output = Rotation;

    #[inline]
    fn mul(self, other: Rotation) -> Rotation {
        Rotation(
            self.0 * other.0 + self.1 * other.2,
            self.0 * other.1 + self.1 * other.3,
            self.2 * other.0 + self.3 * other.2,
            self.2 * other.1 + self.3 * other.3,
        )
    }
}

impl Mul<Move> for Rotation {
    type Output = Move;

    #[inline]
    fn mul(self, other: Move) -> Move {
        Move(
            self.0 * other.0 + self.1 * other.1,
            self.2 * other.0 + self.3 * other.1,
        )
    }
}

/// An ID identifying a cell in lattice rectangle.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CellId(usize);

/// An ID being given to cells on outside the rectangle.
pub const CELL_ID_OUTSIDE: CellId = CellId(0);

impl CellId {
    /// Creates a new `CellId` from an ID.
    #[inline]
    pub fn new(id: usize) -> CellId {
        CellId(id)
    }

    /// Gets an ID of the cell.
    #[inline]
    pub fn id(self) -> usize {
        self.0
    }

    /// Returns if the cell is on outside of the rectangle.
    #[inline]
    pub fn is_outside(self) -> bool {
        self == CELL_ID_OUTSIDE
    }
}

/// A representative `Point` of points on outside the rectangle.
const OUTSIDE_POINT: Point = Point(-1, -1);

/// A rectangle area.
pub trait Geom {
    /// Returns the rectangle's size.
    #[inline]
    fn size(&self) -> Size;

    /// Returns the number of the rectangle's rows.
    #[inline]
    fn row(&self) -> i32 {
        self.size().0
    }

    /// Returns the number of the rectangle's columns.
    #[inline]
    fn column(&self) -> i32 {
        self.size().1
    }

    /// Returns the cell length which is contained in the rectangle.
    #[inline]
    fn cell_len(&self) -> usize {
        (self.row() * self.column() + 1) as usize
    }

    /// Returns true if the point is contained in the rectangle.
    #[inline]
    fn contains(&self, p: Point) -> bool {
        let size = self.size();
        0 <= p.0 && p.0 < size.0 && 0 <= p.1 && p.1 < size.1
    }

    /// Convert a point to a corresponding cell ID.
    #[inline]
    fn point_to_cellid(&self, p: Point) -> CellId {
        if self.contains(p) {
            CellId::new((p.0 * self.column() + p.1 + 1) as usize)
        } else {
            CELL_ID_OUTSIDE
        }
    }

    /// Convert a cell ID to a corresponding point.
    #[inline]
    fn cellid_to_point(&self, id: CellId) -> Point {
        if id.is_outside() {
            OUTSIDE_POINT
        } else {
            let idx = id.id() - 1;
            Point((idx as i32) / self.column(), (idx as i32) % self.column())
        }
    }

    /// Returns an iterator iterating all points.
    #[inline]
    fn points(&self) -> Points {
        if self.row() > 0 && self.column() > 0 {
            Points {
                point: Some(Point(0, 0)),
                size: self.size(),
            }
        } else {
            Points {
                point: None,
                size: self.size(),
            }
        }
    }

    /// Returns an iterator iterating all points in the row.
    #[inline]
    fn points_in_row(&self, row: i32) -> PointsInRow {
        PointsInRow {
            row: row,
            columns: 0..self.column(),
        }
    }

    /// Returns an iterator iterating all points in the column.
    #[inline]
    fn points_in_column(&self, column: i32) -> PointsInColumn {
        PointsInColumn {
            column: column,
            rows: 0..self.row(),
        }
    }
}

/// An iterator iterating all points in the rectangle.
#[derive(Copy, Clone, Debug)]
pub struct Points {
    point: Option<Point>,
    size: Size,
}

impl Iterator for Points {
    type Item = Point;

    #[inline]
    fn next(&mut self) -> Option<Point> {
        if let Some(cur) = self.point {
            let mut next = cur;
            let mut end = false;
            next.1 += 1;
            if next.1 >= self.size.1 {
                next.0 += 1;
                next.1 = 0;
                if next.0 >= self.size.0 {
                    end = true;
                }
            }
            if !end {
                self.point = Some(next);
            } else {
                self.point = None;
            }
            return Some(cur);
        }
        None
    }
}

/// An iterator iterating all points in a row of the rectangle.
#[derive(Clone, Debug)]
pub struct PointsInRow {
    row: i32,
    columns: Range<i32>,
}

impl Iterator for PointsInRow {
    type Item = Point;

    #[inline]
    fn next(&mut self) -> Option<Point> {
        if let Some(c) = self.columns.next() {
            Some(Point(self.row, c))
        } else {
            None
        }
    }
}

/// An iterator iterating all points in a column of the rectangle.
#[derive(Clone, Debug)]
pub struct PointsInColumn {
    rows: Range<i32>,
    column: i32,
}

impl Iterator for PointsInColumn {
    type Item = Point;

    #[inline]
    fn next(&mut self) -> Option<Point> {
        if let Some(r) = self.rows.next() {
            Some(Point(r, self.column))
        } else {
            None
        }
    }
}

/// A table that stores values for each cells.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Table<T> {
    size: Size,
    data: Vec<T>,
}

impl<T> Table<T> {
    /// Creates a new table with data.
    #[inline]
    pub fn new(size: Size, outside: T, mut data: Vec<T>) -> Table<T> {
        assert_eq!((size.0 * size.1) as usize, data.len());
        data.insert(0, outside);
        Table {
            size: size,
            data: data,
        }
    }

    /// Creates a new empty table.
    #[inline]
    pub fn new_empty(size: Size, outside: T, init: T) -> Table<T>
    where
        T: Clone,
    {
        let data = vec![init; (size.0 * size.1) as usize];
        Table::new(size, outside, data)
    }
}

impl<T> Geom for Table<T> {
    #[inline]
    fn size(&self) -> Size {
        self.size
    }
}

impl<T> Index<Point> for Table<T> {
    type Output = T;

    #[inline]
    fn index(&self, p: Point) -> &T {
        let idx = self.point_to_cellid(p).id();
        &self.data[idx]
    }
}

impl<T> IndexMut<Point> for Table<T> {
    #[inline]
    fn index_mut(&mut self, p: Point) -> &mut T {
        let idx = self.point_to_cellid(p).id();
        &mut self.data[idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Rect(Size);
    impl Geom for Rect {
        fn size(&self) -> Size {
            self.0
        }
    }

    #[test]
    fn points() {
        let pts = [
            Point(0, 0),
            Point(0, 1),
            Point(0, 2),
            Point(1, 0),
            Point(1, 1),
            Point(1, 2),
            Point(2, 0),
            Point(2, 1),
            Point(2, 2),
            Point(3, 0),
            Point(3, 1),
            Point(3, 2),
        ];
        let rect = Rect(Size(4, 3));
        assert_eq!(&pts[..], &rect.points().collect::<Vec<_>>()[..]);
    }

    #[test]
    fn rotate_mat() {
        let mat = [ROT_CCW0, ROT_CCW90, ROT_CCW180, ROT_CCW270];
        for i in 0..mat.len() {
            for j in 0..mat.len() {
                assert_eq!(mat[(i + j) % mat.len()], mat[i] * mat[j]);
            }
        }
    }

    #[test]
    fn rotate_point() {
        let mat = [ROT_CCW0, ROT_CCW90, ROT_CCW180, ROT_CCW270];
        let vec = [
            [MOVE_UP, MOVE_LEFT, MOVE_DOWN, MOVE_RIGHT],
            [
                MOVE_UP + MOVE_RIGHT,
                MOVE_LEFT + MOVE_UP,
                MOVE_DOWN + MOVE_LEFT,
                MOVE_RIGHT + MOVE_DOWN,
            ],
        ];
        for i in 0..mat.len() {
            for v in &vec {
                for j in 0..v.len() {
                    assert_eq!(v[(i + j) % v.len()], mat[i] * v[j]);
                }
            }
        }
    }
}
