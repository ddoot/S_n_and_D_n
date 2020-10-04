use std::convert::TryFrom;

enum Transformation {
    D_n_generator(D_n_Generators),
    S_n_generator(u16), /* 0 is ``id,'', i is (1, i + 1) */
}

enum D_n_Generators {
    p, /* proper rotation */
    x, /* improper rotation, or ``reflection'' about vertex slot 0 */
}

struct Point {
    color : (u8, u8, u8),
    /* Perhaps including fields to reach other
     * points isn't a terrible future idea */
}


struct Shape {
    vertices : Vec<Point>,
    rot_index : u16,
    inverted : bool,
}

impl Shape {
    pub fn new(number_points : u16) -> Shape {
        let capacity : usize = usize::from(number_points);
        let mut v : Vec<Point> = Vec::with_capacity(capacity);
        for i in 0..capacity {
            v.push(Point {color : (u8::try_from(i).unwrap(), 0, 0)});
        }
        Shape {
            vertices : v,
            rot_index : 0,
            inverted : false,
        }
    }

    /* Applies the transformation requested by the user to the 
     * shape; returns a boolean indicating whether the shape
     * was changed (i.e., a non-trivial transformation) 
     * upon sucess, or an error upon invalid input.
     * */
    pub fn apply_transformation 
        (&mut self, requested : &String) -> Result<bool, &str> {
            let mut tv : Vec<Transformation> = Shape::tokenize(requested).unwrap();
            Shape::simplify(&mut tv);
            let non_trivial : bool = tv.len() > 0;
            Ok(non_trivial)
    }

    pub fn display (& self) -> String {
        let mut display_str = String::new();
        display_str
    }

    /* WIP 
     * (1, i) = (i, 1) = Transformation::S_n_generator(i - 1) 
     * p^{}
     * x^m
     * */
    fn tokenize(requested : &String) -> Result<Vec<Transformation>, &str> {
        let mut tv : Vec<Transformation> = Vec::new();
        let mut erroneous = false;
        if !erroneous {Ok(tv)} else {Err("Invalid user input")}
    }
    /* WIP */
    fn simplify(tv : &mut Vec<Transformation>) {
    }


    fn transform(&mut self, tv : &Vec<Transformation>) {
        for transformation in tv.iter() {
            match transformation {
                Transformation::D_n_generator(rotation) => self.apply_rotation(&rotation),
                Transformation::S_n_generator(which_position) =>
                    self.vertices.swap(0, usize::from(*which_position)),
            }
        }
    }

    fn apply_rotation(&mut self, rotation : &D_n_Generators) {
        match rotation {
            D_n_Generators::x => self.inverted = !self.inverted,
            D_n_Generators::p => {
                let base : u16 = u16::try_from(self.vertices.len()).unwrap();
                self.rot_index = (if !self.inverted {self.rot_index + 1} else {self.rot_index - 1}) % base;
            },
        }
    }
}

fn main() {
    println!("Hello, world!");
}
