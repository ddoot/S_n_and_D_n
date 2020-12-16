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


     /*      0
      *  1       4
      *
      *    2   3
      *
      *      p
      *      4
      *  0       3
      *    
      *    1   2
      *
      *
      *
      * */


struct Shape {
    vertices : Vec<Point>,
    rot_index : u16,
    inverted : bool,
}

impl Shape {
    pub fn new(number_points : u16) -> Shape {

        /* From recollection, this was about u16? */
        let capacity : usize = usize::from(number_points);

        let mut v : Vec<Point> = Vec::with_capacity(capacity);

        let mut red : u8;
        let mut green : u8;
        let mut blue : u8;
        /* Addiing the points */
        for i in 0..capacity {

            /* This is definitely fallible */
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
        (&mut self, requested : &str) -> Result<bool, &str> {
            let mut tv : Vec<Transformation> = Shape::tokenize(requested).unwrap();
            Shape::simplify(&mut tv);
            self.transform(&tv);
            Ok(tv.len() > 0)
    }

    pub fn display (&self) -> String {
        let mut display_str = String::new();
        
        let base : usize = self.vertices.len();
        /* We need compatibility because of logic involving negative multiplications */
        let comp_base : i32 = i32::try_from(base).unwrap();
        let comp_rot_index : i32 = i32::try_from(self.rot_index).unwrap();
        let inv_multiplier : i32 = if !self.inverted {1} else {-1};
        
        for i in (0..base).map(|x| usize::try_from(
            (comp_base + comp_rot_index + i32::try_from(x).unwrap() * inv_multiplier) % comp_base
        ).unwrap()) {
            display_str.push_str(format!("rot_index is {} with color {} ", i, self.vertices[i].color.0).as_str());
            display_str.push_str(format!("rot_index is {} with color {} ", i, self.vertices[i].color.1).as_str());
            display_str.push_str(format!("rot_index is {} with color {} \n", i, self.vertices[i].color.2).as_str());
        }

        display_str
    }

    /* (1, i) = (i, 1) = Transformation::S_n_generator(i - 1) 
     * p^{}
     * x^m
     * */
    fn tokenize(requested : &str) -> Result<Vec<Transformation>, &str> {
        let mut tv : Vec<Transformation> = Vec::new();
        let mut erroneous = false;
        let split = requested.split_whitespace();
        for token in split {
            match token.chars().nth(0).unwrap() {
               'x' => tv.push(Transformation::D_n_generator(D_n_Generators::x)),
               'p' => tv.push(Transformation::D_n_generator(D_n_Generators::p)),
               '(' => {
                   /* atoi() like thing needed here */
               },
               _   => {
                   erroneous = true;
                   break;
               },
           }
        }
        if !erroneous {Ok(tv)} else {Err("Invalid user input")}
    }


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
                /* SO MANY ASSUMPTIONS ABOUT ARITHMETIC: GET HELP */
                self.rot_index = ((if self.inverted {self.rot_index + base + 1} else {self.rot_index + base - 1})) % base;
            },
        }
    }
}

fn main() {
    let mut s : Shape = Shape::new(3);
    println!("{}", s.display().as_str());
    s.apply_transformation("x");
    println!("{}", s.display().as_str());

    s.apply_transformation("x");
    println!("{}", s.display().as_str());

    s.apply_transformation("x");
    println!("{}", s.display().as_str());

    s.apply_transformation("p");
    println!("{}", s.display().as_str());

    s.apply_transformation("x");
    println!("{}", s.display().as_str());

    s.apply_transformation("p");
    println!("{}", s.display().as_str());
}
