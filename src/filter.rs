use std::cmp::{max, min};

pub trait ImagePixel: Copy {
    fn to_i32(self) -> i32;
    fn to_f64(self) -> f64;

}

impl ImagePixel for f64 {
    fn to_i32(self) -> i32
     {
        self as i32
    }
    fn to_f64(self) -> f64
    {
       self 
   }
}

impl ImagePixel for u8 {
    fn to_i32(self) -> i32 {
        self as i32
    }
    fn to_f64(self) -> f64 {
        self as f64
    }
}
fn are_vectors_equal(v1: &Vec<f64>, v2: &Vec<f64>, threshold: f64) -> bool {
    if v1.len() != v2.len() {
        return false;
    }

    v1.iter().zip(v2.iter()).all(|(&a, &b)| (a - b).abs() <= threshold)
}
pub struct Filter{
    kernel:Vec<f64>,
    width:usize,
    height:usize
}

impl Filter {
    pub fn new(kernel:Vec<f64> , width:usize, height:usize) -> Self {
        Self { kernel, width, height}
    }}
pub fn apply_filter<T:ImagePixel>(image: &[T], width: usize, height: usize, filter:&Filter) ->Vec<f64> {


    let mut buf = vec![0.0; image.len()];
    let bx = (filter.width / 2) as isize;
    let by = (filter.height / 2) as isize;
    for r in 0..height {
        for c in 0..width {
            let start_x = max(c as isize - bx, 0) as usize;
            let stop_x = min(c as isize + bx, width as isize - 1) as usize;
            let start_y = max(r as isize - by, 0) as usize;
            let stop_y = min(r as isize + by, height as isize - 1) as usize;
            let mut val = 0.0;

            for ix in start_x..=stop_x {
                for iy in start_y..=stop_y {
                    let kx = ix + bx as usize - c;
                    let ky = iy + by as usize - r;
                    let val_i = image[ix + iy * width].to_f64();
                    val += val_i * filter.kernel[kx + ky * filter.width];
     
                }
            }
                buf[c + r * width] = val;
   

            
        }
    }
    buf

}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_uniform(){
        let mat:Vec<f64> = vec![
            0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,
            0,0,255,255,255,0,0,
            0,0,255,255,255,0,0,
            0,0,255,255,255,0,0,
            0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,
        ].into_iter().map(|x| x as f64).collect();
        let w = 7;
        let h = 7;
        let expected: Vec<f64> = vec![
        10.2, 20.4, 30.6, 30.6, 30.6, 20.4, 10.2,
       20.4, 40.8, 61.2, 61.2, 61.2, 40.8, 20.4,
       30.6, 61.2, 91.8, 91.8, 91.8, 61.2, 30.6,
       30.6, 61.2, 91.8, 91.8, 91.8, 61.2, 30.6,
       30.6, 61.2, 91.8, 91.8, 91.8,61.2, 30.6,
       20.4, 40.8, 61.2, 61.2, 61.2,40.8, 20.4,
       10.2, 20.4, 30.6, 30.6, 30.6, 20.4, 10.2
        ];
        
        let filter = Filter::new(vec![1.0/25.0; 25],5,5);
        let response = apply_filter(&mat, w, h, &filter);
        let threshold = 0.001;
        println!("{:?}", response);
        assert!(are_vectors_equal(&response, &expected, threshold)); 

    }

}

