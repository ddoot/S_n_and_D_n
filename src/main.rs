struct Point {
    color : (u8, u8, u8),
    /* Perhaps including fields to reach other
     * points isn't a terrible future idea */
}


struct Shape {
    vertices : Vec<Point>,
    rot_index : u32,
    inverted : bool,
}

impl Shape {
    pub fn new(number_points : usize) -> Shape {
        let mut v : Vec<Point> = Vec::with_capacity(number_points);
        for i in 0..number_points {
            v.push(Point {color : (0, 0, 0)});
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
            let mut non_trivial : bool = false;
            /* tokenize (requested); */
            Ok(non_trivial)
    }
    fn tokenize(requested : &String) /* -> Something */{
    }
}

fn main() {
    println!("Hello, world!");
}
