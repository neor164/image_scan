use image::error::UnsupportedErrorKind;

use crate::sobel::{Kernel, KernelDir, Sobel};
use crate::filter::{apply_filter, Filter};

pub struct Harris{
}


pub fn calculate_harris_matrix(image:&[u8], width:usize, height:usize) -> Vec<u8>{

    let filter = Filter::new(vec![
            1.0/273.0,4.0/273.0,7.0/273.0,4.0/273.0,1.0/273.0,
            4.0/273.0,16.0/273.0,26.0/273.0,16.0/273.0,4.0/273.0,
            7.0/273.0,26.0/273.0,41.0/273.0,26.0/273.0,7.0/273.0,
            4.0/273.0,16.0/273.0,26.0/273.0,16.0/273.0,4.0/273.0,
            1.0/273.0,4.0/273.0,7.0/273.0,4.0/273.0,1.0/273.0,
                
        ],5,5);   
    let filtered_image = apply_filter(&image, width, height, &filter);
    let response =  calculate_response_matrix(&filtered_image, width, height);
    non_maximal_suppression(&response, width,height,300, 32,32)


}
 
fn calculate_response_matrix(filtered_image: &[f64], width: usize, height: usize) -> Vec<f64>{

    let ixx = Sobel::new().kernel(Kernel::X3).apply(filtered_image, width, height, KernelDir::XX);
    let ixy = Sobel::new().kernel(Kernel::X3).apply(filtered_image, width, height, KernelDir::XY);
    let iyy = Sobel::new().kernel(Kernel::X3).apply(filtered_image, width, height, KernelDir::YY);
    let filter: Filter = Filter::new(vec![1.0/25.0; 25],5,5);

    let ixx = apply_filter(&ixx, width, height, &filter);
    let ixy = apply_filter(&ixy, width, height, &filter);
    let iyy = apply_filter(&iyy, width, height, &filter);
    let k = 0.05;

    let mut buf = vec![0.0; filtered_image.len()];
    for r in 0..height {
        for c in 0..width {
                let index =  c + r * width;
                let det = ixx[index] * iyy[index] - ixy[index].powf(2.0);
                let trace = ixx[index] + iyy[index];
                buf[index] = det + k * trace.powf(2.0);     
        }
    }
    buf
}


pub fn non_maximal_suppression(harris_response:&[f64], width:usize, height:usize, n_corners:usize, gx:usize, gy:usize) -> Vec<u8> {

    let n_rows: usize = height / gx;
    let n_cols: usize = width / gy;
    let max_value = harris_response.iter().max_by(|a, b: &&f64| a.total_cmp(b)).unwrap();


    let filtered_harris = harris_response.iter().map(|&x| if x > max_value / 6.0 { 255 } else { 0 }).collect();
    filtered_harris
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::ImageReader;
    use image::GrayImage;
    use rstest::rstest;

    #[rstest]
    #[case( "menorah.jpg")]
    #[case( "checkers.jpg")]
    #[case( "checkers.jpg")]
    #[case( "house_corners.png")]
    #[case( "PXL_20240824_143132519.RAW-02.ORIGINAL.png")]
    #[case( "PXL_20240824_143133946.RAW-02.ORIGINAL.png")]
   
    fn test_corner_response(#[case] img_name: &str) {
        let src_path = format!("./data/png/{img_name}");
        let img = ImageReader::open(src_path).unwrap().decode().unwrap();
        let img = img.grayscale().into_luma8();
        let result = calculate_harris_matrix(img.as_raw(), img.width() as usize, img.height() as usize);
        let width = img.width();
        let height = img.height();
        // let result = result.iter().map(|&x| (x) as u8).collect();
        let result = GrayImage::from_raw(width as u32, height as u32, result).unwrap();
        let fname = format!("./data/out/{img_name}_corners.png");
        result.save(fname).unwrap();
    }
}
