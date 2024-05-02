use std::fmt;

// also going to be defining my own BigFloat

struct BigFloat {
    sign: bool,
    int: Vec<u8>,
    float: Vec<u8>
}

impl fmt::Display for BigFloat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out_str: String = match self.sign {
            true => String::from("-"), // yes sign = negative
            false => String::from(""), //  no sign = positive
        };

        out_str.push_str(&self.int.clone().iter() // convert int vector to iterable 
                          .map(|&i| i.to_string()) // map all elements from ints to strs
                          .collect::<String>()); // collect and copmile all strings into one
        out_str.push_str(&".");
        out_str.push_str(&self.float.clone().iter() // convert int vector to iterable 
                          .map(|&i| i.to_string()) // map all elements from ints to strs
                          .collect::<String>()); // collect and copmile all strings into one
        write!(f, "{out_str}")
    }
}

fn main() {
    println!("Hello, world!");
    let test_BigFloat: BigFloat = BigFloat{sign: false, 
                                           int: vec![1, 9, 6], 
                                           float: vec![8, 8, 3]};
    // this should print 196.883
    println!("{}", test_BigFloat);

    // TODO: implement creating BigFloat from String(s)
    // TODO: implement basic arithmetic operations for BigFloat
}
