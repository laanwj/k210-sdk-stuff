/** Moving filters for touch screen noise filtering */
use core::ops::{AddAssign, SubAssign, Div};
use core::cmp::PartialOrd;

/* TODO: really want const_generics here */
/* must be smaller than 256 */
const S: usize = 5;

/** Moving average filter */
#[derive(Clone)]
pub struct Mean<T> {
    buffer: [T; S],
    index: usize,
    count: usize,
    sum: T,
}

impl<T: Copy + From<u8> + AddAssign + SubAssign + Div> Mean<T> 
    where T: Div<Output = T> {
    pub fn new() -> Self {
        Self {
            buffer: [T::from(0); S],
            index: 0,
            count: 0,
            sum: T::from(0),
        }
    }

    pub fn update(&mut self, value: T) -> T {
        self.sum -= self.buffer[self.index];
        self.sum += value;
        self.buffer[self.index] = value;
        self.index = if self.index == S - 1 { 0 } else { self.index + 1 };
        if self.count < S {
            self.count += 1;
        }
        return self.sum / T::from(self.count as u8);
    }

    pub fn clear(&mut self) {
        self.buffer = [T::from(0); S];
        self.index = 0;
        self.count = 0;
        self.sum = T::from(0);
    }
}

/** Moving median filter */
#[derive(Clone)]
pub struct Median<T> {
    buffer: [T; S],
    index: [usize; S],
    position: usize,
    count: usize,
}

impl<T: Copy + From<u8> + PartialOrd> Median<T> 
    where T: Div<Output = T> {
    pub fn new() -> Self {
        Self {
            buffer: [T::from(0); S],
            index: [0; S],
            position: 0,
            count: 0,
        }
    }

    pub fn update(&mut self, value: T) -> T {
        let pos = self.position;
        let cnt = self.count;
        let result: T;

        if cnt > 0 {
            let oval;
            let mut oidx;
            let dummy_oval;
            if cnt == S {
                oidx = 0;
                while self.index[oidx] != pos {
                    oidx += 1;
                }
                oval = self.buffer[pos];
                dummy_oval = false;
            }
            else
            {
                self.index[pos] = pos;
                oidx = pos;
                oval = T::from(0);
                dummy_oval = true;
            }
            self.buffer[pos] = value;

            if !dummy_oval && oval < value {
                while (oidx + 1) != cnt
                {
                    oidx += 1;
                    let cidx = self.index[oidx];
                    if self.buffer[cidx] < value {
                        self.index[oidx] = self.index[oidx - 1];
                        self.index[oidx - 1] = cidx;
                    } else {
                        break;
                    }
                }
            }
            else if dummy_oval || oval > value {
                while oidx != 0
                {
                    oidx -= 1;
                    let cidx = self.index[oidx];
                    if self.buffer[cidx] > value {
                        self.index[oidx] = self.index[oidx + 1];
                        self.index[oidx + 1] = cidx;
                    } else {
                        break;
                    }
                }
            }

            result = self.buffer[self.index[cnt / 2]];
        }
        else
        {
            self.buffer[0] = value;
            self.index[0] = 0;
            self.position = 0;
            self.count = 0;
            result = value;
        }

        self.position = if pos == S - 1 { 0 } else { pos + 1 };
        if cnt < S {
            self.count += 1;
        }

        result
    }

    pub fn clear(&mut self) {
        self.buffer = [T::from(0); S];
        self.index = [0; S];
        self.position = 0;
        self.count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean() {
        let mut t = Mean::<i32>::new();
        assert_eq!(t.update(10), 10);
        assert_eq!(t.update(20), 15);
        assert_eq!(t.update(0), 10);
        assert_eq!(t.update(10), 10);
        assert_eq!(t.update(60), 20);
        assert_eq!(t.update(10), 20);
    }

    #[test]
    fn test_median() {
        let mut t = Median::<i32>::new();
        assert_eq!(t.update(10), 10); // 10
        assert_eq!(t.update(20), 10); // 10 20
        assert_eq!(t.update(30), 20); // 10 20 30
        assert_eq!(t.update(40), 20); // 10 20 30 40
        assert_eq!(t.update(50), 30); // 10 20 30 40 50
        assert_eq!(t.update(0), 30);  // 0  20 30 40 50
        assert_eq!(t.update(60), 40); // 0  30 40 50 60
        assert_eq!(t.update(10), 40); // 0  10 40 50 60
        assert_eq!(t.update(10), 10); // 0  10 10 50 60
        assert_eq!(t.update(15), 10); // 0  10 10 15 60
        assert_eq!(t.update(70), 15); // 10 10 15 60 70
    }
}
