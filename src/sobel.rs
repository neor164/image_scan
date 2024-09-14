use std::cmp::{max, min};

use crate::filter::ImagePixel;

#[derive(Debug, Clone, Copy)]
pub enum Kernel {
    X5,
    X3,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KernelDir
{
    X,
    Y,
    XX,
    YY,
    XY

}

impl Kernel {
    #[inline]
    fn size(self) -> usize {
        match self {
            Self::X5 => 5,
            Self::X3 => 3,
        }
    }

    fn y(self) -> Vec<i32> {
        match self {
            Self::X5 => vec![
                2, 2, 4, 2, 2, 1, 1, 2, 1, 1, 0, 0, 0, 0, 0, -1, -1, -2, -1, -1, -2, -2, -4, -2, -2,
            ],
            Self::X3 => vec![-1, -2, -1, 0, 0, 0, 1, 2, 1],
        }
    }

    fn x(self) -> Vec<i32> {
        match self {
            Self::X5 => vec![
                2, 1, 0, -1, -2, 2, 1, 0, -1, -2, 4, 2, 0, -2, -4, 2, 1, 0, -1, -2, 2, 1, 0, -1, -2,
            ],

            Self::X3 => vec![-1, 0, 1, -2, 0, 2, -1, 0, 1],
        }

    }
}

#[derive(Debug)]
pub struct Sobel {
    kernel: Kernel,
}




impl Sobel {
    pub fn new() -> Self {
        Self { kernel: Kernel::X3 }
    }

    pub fn kernel(mut self, kernel: Kernel) -> Self {
        self.kernel = kernel;
        self
    }

    pub fn apply<T: ImagePixel>(&self, image: &[T], width: usize, height: usize, kernel_dir:KernelDir) -> Vec<f64> {
        let kernel_a = match kernel_dir {
            KernelDir::X => self.kernel.x(),
            KernelDir::Y => self.kernel.y(),
            KernelDir::XX => self.kernel.x(),
            KernelDir::YY => self.kernel.y(),
            KernelDir::XY => self.kernel.x(),
        };
        let kernel_b = match kernel_dir {
            KernelDir::X => self.kernel.x(),
            KernelDir::Y => self.kernel.y(),
            KernelDir::XX => self.kernel.x(),
            KernelDir::YY => self.kernel.y(),
            KernelDir::XY => self.kernel.y(),
        };

        //let kernel_x = self.kernel.x();
        //let kernel_y = self.kernel.y();
        let mut buf = vec![0.0; image.len()];
        let ksize = self.kernel.size();
        let b = (ksize / 2) as isize;
        for r in 0..height {
            for c in 0..width {
                let start_x = max(c as isize - b, 0) as usize;
                let stop_x = min(c as isize + b, width as isize - 1) as usize;
                let start_y = max(r as isize - b, 0) as usize;
                let stop_y = min(r as isize + b, height as isize - 1) as usize;
                let mut val_a = 0.0;
                let mut val_b = 0.0;

                for ix in start_x..=stop_x {
                    for iy in start_y..=stop_y {
                        let kx = ix + b as usize - c;
                        let ky = iy + b as usize - r;
                        let val_i = image[ix + iy * width].to_i32();
                        val_a += (val_i * kernel_a[kx + ky * ksize]) as f64;
                        if kernel_dir != KernelDir::X && kernel_dir != KernelDir::Y{
                            val_b += (val_i * kernel_b[kx + ky * ksize]) as f64
                        }
                    }
                }
                if kernel_dir == KernelDir::X || kernel_dir == KernelDir::Y{
                    buf[c + r * width] = val_a;}
                else{

                    buf[c + r * width] = val_a * val_b;

                }
            }
        }
        buf
    }
}

impl Default for Sobel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sobel_x_3(){
        let mat:Vec<u8> = vec![
            0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,
            0,0,1,1,1,0,0,
            0,0,1,1,1,0,0,
            0,0,1,1,1,0,0,
            0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,
        ];
        let w = 7;
        let h = 7;
        let expected: Vec<f64> = vec![ 
         0.,  0.,  0.,  0.,  0.,  0.,  0.,
         0.,  1.,  1.,  0., -1., -1.,  0.,
         0.,  3.,  3.,  0., -3., -3.,  0.,
         0.,  4.,  4.,  0., -4., -4.,  0.,
         0.,  3.,  3.,  0., -3., -3.,  0.,
         0.,  1.,  1.,  0., -1., -1.,  0.,
         0.,  0.,  0.,  0.,  0.,  0.,  0.];

        let response = Sobel::new().kernel(Kernel::X3).apply(&mat, w, h, KernelDir::X);
        println!("{:?}", response);
        assert_eq!(response, expected); 

    }
    #[test]
    fn test_sobel_xx_3(){
        let w = 7;
        let h = 7;
        let mat:Vec<u8> = vec![
            0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,
            0,0,1,1,1,0,0,
            0,0,1,1,1,0,0,
            0,0,1,1,1,0,0,
            0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,
        ];
        let expected = vec![ 
        0.,  0.,  0.,  0.,  0.,  0.,  0.0,
        0.,  1.,  1.,  0.,  1.,  1.,  0.0,
        0.,  9.,  9.,  0.,  9.,  9.,  0.0,
        0., 16., 16.,  0., 16., 16.,  0.0,
        0.,  9.,  9.,  0.,  9.,  9.,  0.0,
        0.,  1.,  1.,  0.,  1.,  1.,  0.0,
        0.,  0.,  0.,  0.,  0.,  0.,  0.0];

        let response = Sobel::new().kernel(Kernel::X3).apply(&mat, w, h, KernelDir::XX);
        println!("{:?}", response);
        assert_eq!(response, expected); 
    }
    #[test]
    fn test_sobel_y_3(){
        let mat:Vec<u8> = vec![
            0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,
            0,0,1,1,1,0,0,
            0,0,1,1,1,0,0,
            0,0,1,1,1,0,0,
            0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,
        ];
        let w = 7;
        let h = 7;
        let expected: Vec<f64> = vec![ 
             0.,  0.,  0.,  0.,  0.,  0.,  0.,
             0.,  1.,  3.,  4.,  3.,  1.,  0.,
             0.,  1.,  3.,  4.,  3.,  1.,  0.,
             0.,  0.,  0.,  0.,  0.,  0.,  0.,
             0., -1., -3., -4., -3., -1.,  0.,
             0., -1., -3., -4., -3., -1.,  0.,
             0.,  0.,  0.,  0.,  0.,  0.,  0.];

        let response = Sobel::new().kernel(Kernel::X3).apply(&mat, w, h, KernelDir::Y);
        println!("{:?}", response);
        assert_eq!(response, expected); 

    }
    #[test]
    fn test_sobel_xy_3(){
        let w = 7;
        let h = 7;
        let mat:Vec<u8> = vec![
            0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,
            0,0,1,1,1,0,0,
            0,0,1,1,1,0,0,
            0,0,1,1,1,0,0,
            0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,
        ];
        let expected = vec![ 
             0.,  0.,  0.,  0.,  0.,  0.,  0.,
             0.,  1.,  3.,  0., -3., -1.,  0.,
             0.,  3.,  9.,  0., -9., -3.,  0.,
             0.,  0.,  0.,  0., -0., -0.,  0.,
             0., -3., -9., -0.,  9.,  3.,  0.,
             0., -1., -3., -0.,  3.,  1.,  0.,
             0.,  0.,  0.,  0.,  0.,  0.,  0.];

        let response = Sobel::new().kernel(Kernel::X3).apply(&mat, w, h, KernelDir::XY);
        println!("{:?}", response);
        assert_eq!(response, expected); 
    }


}
