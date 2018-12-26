use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Error;
use std::fmt::Display;
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Default, Ord, Eq)]
pub struct Pos {
    pub x: usize,
    pub y: usize
}


#[derive(Clone, Default)]
pub struct Mat2<T> {
    h: usize,
    w: usize,
    data: Vec<T>
}

fn diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Pos {
        Pos {x, y}
    }

    pub fn add(&self, other: Pos) -> Pos {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }

    pub fn sub(&self, other: Pos) -> Option<Pos> {
        if self.x >= other.x && self.y >= other.y {
            Some(Pos{
                x: self.x - other.x,
                y: self.y - other.y
            })
        } else {
            None
        }
    }

    pub fn offset(&self, off: (isize, isize)) -> Option<Pos> {
        let x = (self.x as isize) + off.0;
        let y = (self.y as isize) + off.1;

        if x < 0 || y < 0 {
            return None
        } else {
            Some(Pos::new(x as usize, y as usize))
        }
    }

    pub fn euclid_dist(&self, other: Pos) -> usize {
        let a = diff(self.x, other.x).pow(2) +
            diff(self.y, other.y).pow(2);
        (a as f64).sqrt() as usize
    }
}



impl<T> Mat2<T> {
    pub fn from_vec(data: Vec<T>, h: usize, w: usize) -> Self {
        if (h*w) != data.len() {
            panic!("Mat2 size");
        }

        Mat2{
            data,
            h,
            w
        }
    }


    pub fn height(&self) -> usize {
        self.h
    }

    pub fn width(&self) -> usize {
        self.w
    }

    fn coords_to_ix(&self, p: Pos) -> Option<usize> {
        if p.x < self.w && p.y < self.h {
            Some(self.w * p.y + p.x)
        } else {
            None
        }
    }

    pub fn has_pos(&self, p: Pos) -> bool {
        self.coords_to_ix(p).is_some()
    }

    pub fn get(&self, p: Pos) -> Option<&T> {
        self.coords_to_ix(p)
            .map(|ix| &self.data[ix])
    }

    pub fn get_mut(&mut self, p: Pos) -> Option<&mut T> {
        self.coords_to_ix(p)
            .map(move |ix| &mut self.data[ix])
    }

    pub fn set(&mut self, p: Pos, val: T) {
        let ix = self.coords_to_ix(p).unwrap();
        self.data[ix] = val;
    }

    pub fn coords(&self) -> impl Iterator<Item=Pos> {
        let w = self.w;
        let n = self.data.len();
        (0..n)
            .map(move |ix| Pos::new(ix%w, ix/w))
    }

    pub fn grids<'a>(&'a self) -> impl Iterator<Item=&'a T> + '_ {
        self.coords()
            .map(move |p| self.get(p).unwrap())
    }



}

impl<T: Default + Clone> Mat2<T> {
    pub fn new(h: usize, w: usize) -> Self {
        Mat2 {
            h,
            w,
            data: vec![T::default(); h*w]
        }
    }
}

impl<T: Clone> Mat2<T> {
    pub fn new_with(h: usize, w: usize, with: T) -> Self {
        Mat2 {
            h,
            w,
            data: vec![with; h*w]
        }
    }

    pub fn clear(&mut self, with: T) {
        for d in self.data.iter_mut() {
            *d = with.clone();
        }
    }
}

impl<T: Debug> Debug for Mat2<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for y in 0..self.h {
            if y != 0 {
                writeln!(f)?;
            }

            for x in 0..self.w {
                if x != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}", self.get(Pos::new(x, y)).unwrap())?;
            }
        }

        Ok(())
    }
}

impl<T: Display> Display for Mat2<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for y in 0..self.h {
            if y != 0 {
                writeln!(f)?;
            }

            for x in 0..self.w {
                if x != 0 {
                    write!(f, "")?;
                }
                write!(f, "{}", self.get(Pos::new(x, y)).unwrap())?;
            }
        }

        Ok(())
    }
}

impl<T> Index<Pos> for Mat2<T> {
    type Output = T;

    fn index(&self, index: Pos) -> &<Self as Index<Pos>>::Output {
        self.get(index).unwrap()
    }
}

impl<T> IndexMut<Pos> for Mat2<T> {
    fn index_mut(&mut self, index: Pos) -> &mut <Self as Index<Pos>>::Output {
        self.get_mut(index).unwrap()
    }
}