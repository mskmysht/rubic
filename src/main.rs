use std::fmt::Display;

#[derive(Clone, Copy)]
enum Axe {
    X,
    Y,
    Z,
}

struct Rot<const G: usize> {
    a: Axe,
    n: i32,
    s: [bool; G],
}

impl<const G: usize> Rot<G> {
    pub fn new(a: Axe, n: i32, s: [bool; G]) -> Self {
        Rot { a, n, s }
    }
}

#[derive(Clone, Copy, Debug)]
enum Color {
    G,
    Y,
    R,
    B,
    W,
    O,
}

enum Part {
    Edge(Color, Color),
    Corner(Color, Color, Color),
    Center(Color),
    Core,
}

type Cube<A, const G: usize> = [[[A; G]; G]; G];

fn print_cube<A: Display, const G: usize>(b: &Cube<A, G>) {
    for y in 0..G {
        print!("|");
        for x in 0..G {
            for z in 0..G {
                print!("{}", b[x][y][z]);
            }
            print!("|");
        }
        println!();
    }
    println!();
}

fn test() {
    let mut cube = [[[[Color::B; 3]; 3]; 2]; 3];

    for (k, &c) in [Color::G, Color::Y, Color::R, Color::B, Color::W, Color::O]
        .iter()
        .enumerate()
    {
        for i in 0..3 {
            for j in 0..3 {
                cube[k % 3][k / 3][i][j] = c;
            }
        }
    }

    for i in 0..3 {
        print!("|");
        for a in 0..3 {
            for d in 0..2 {
                for j in 0..3 {
                    print!("{:?}", cube[a][d][i][j]);
                }
                print!("|");
            }
        }
        println!();
    }
}

fn rotate<A: Clone, const GRID: usize>(b: &Cube<A, GRID>, rs: Vec<Rot<GRID>>) -> Cube<A, GRID> {
    let mut b2 = b.clone();
    let mut temp;

    for r in rs {
        temp = b2.clone();
        let i = r.a as usize;
        for x in 0..=2 {
            for y in 0..=2 {
                for z in 0..=2 {
                    let c = [x, y, z];
                    let d = if r.s[c[i]] {
                        rotate_vector(c, r.a, r.n)
                    } else {
                        c
                    };
                    b2[d[0]][d[1]][d[2]] = temp[x][y][z].clone();
                }
            }
        }
    }

    b2
}

type V3<A> = [A; 3];

fn rotate_vector(c: V3<usize>, a: Axe, n: i32) -> V3<usize> {
    if n == 0 {
        return c;
    }

    let i = a as usize;
    let j = (i + 1) % 3;
    let k = (i + 2) % 3;
    let m = n.abs();
    let d = n.signum();

    let mut temp: V3<i32> = {
        let mut x = V3::default();
        for i in 0..3 {
            x[i] = c[i] as i32 - 1;
        }
        x
    };

    let mut v_ = [0, 0, 0];

    for _ in 0..m {
        v_[i] = temp[i];
        v_[k] = temp[j] * d;
        v_[j] = -temp[k] * d;
        temp = v_;
    }

    let v = {
        let mut x = V3::default();
        for i in 0..3 {
            x[i] = (v_[i] + 1) as usize;
        }
        x
    };
    v
}

fn main() {
    test();

    let mut b = [[[' '; 3]; 3]; 3];
    b[0][0][1] = 'a';
    b[0][1][2] = 'b';
    b[0][2][1] = 'c';
    b[0][1][0] = 'd';

    b[1][0][1] = 'e';
    b[1][1][2] = 'f';
    b[1][2][1] = 'g';
    b[1][1][0] = 'h';

    b[2][0][1] = 'i';
    b[2][1][2] = 'j';
    b[2][2][1] = 'k';
    b[2][1][0] = 'l';

    print_cube(&b);
    print_cube(&rotate(
        &b,
        vec![
            Rot::new(Axe::X, 1, [true, true, true]),
            Rot::new(Axe::Z, 3, [true, true, true]),
        ],
    ));
    print_cube(&rotate(&b, vec![Rot::new(Axe::Z, -1, [true, true, true])]));
    print_cube(&rotate(&b, vec![Rot::new(Axe::X, 1, [true, false, false])]));
}
