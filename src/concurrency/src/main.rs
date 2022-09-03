use std::path::Path;
use std::marker::Copy;
use ndarray::{Array, Array1};
use gdal::raster::{Buffer, RasterBand, GdalType};
use gdal::{Dataset, Driver, GeoTransform};
use gdal::spatial_ref::SpatialRef;


fn main() {
    let input_filepath: String = std::env::args().nth(1).expect(
        "No input filepath given"
    );

    let input_path = Path::new(&input_filepath);
    let input_dataset = Dataset::open(input_path).unwrap();

    let band_1: RasterBand = input_dataset.rasterband(1).unwrap();
    let band_2: RasterBand = input_dataset.rasterband(1).unwrap();
    let band_3: RasterBand = input_dataset.rasterband(1).unwrap();
    let band_4: RasterBand = input_dataset.rasterband(1).unwrap();

    let output_file_1 = Path::new(&"test1.tif");
    let output_file_2 = Path::new(&"test2.tif");
    let output_file_3 = Path::new(&"test3.tif");
    let output_file_4 = Path::new(&"test4.tif");

    println!("{:?}", output_file_1);

    let rows: isize = band_1.size().0.try_into().unwrap();
    let cols: isize = band_1.size().1.try_into().unwrap();
    let srs = input_dataset.spatial_ref().unwrap();
    let transform = input_dataset.geo_transform().unwrap();
    let projection = input_dataset.projection();

    let creation_options = RasterWriteOptions {
        rows: &rows,
        cols: &cols,
        srs: &srs,
        transform: &transform,
        proj: &projection,
    };

    let output_files = [output_file_1, output_file_2, output_file_3, output_file_4];
    let rasterbands = [band_1, band_2, band_3, band_4];

    let n_processes = output_files.len();

    for i in 0..n_processes {
        let band = rasterbands[i];
        let output_file = output_files[i];
        write_single_band_raster(
            band, 
            output_file, 
            &creation_options
        );
    };

}


fn write_single_band_raster<P: AsRef<Path>>(
    input_data_band: RasterBand, 
    output_path: P, 
    creation_options: &RasterWriteOptions
) {
    
    let output_driver = Driver::get("GTIFF").unwrap();
    let mut output_dataset = output_driver.create_with_band_type::<f64, P>(
        output_path, 
        *creation_options.rows, 
        *creation_options.cols, 
        1
    ).unwrap();

    output_dataset.set_projection(
        &creation_options.proj
    ).expect("Error adding projection to output dataset!");
    output_dataset.set_spatial_ref(
        &creation_options.srs
    ).expect("Error adding spatial ref to output dataset!");
    output_dataset.set_geo_transform(
        &creation_options.transform
    ).expect("Error adding transform to output dataset!");

    let band_size = input_data_band.size();

    let array = read_array_from_rasterband::<f64>(input_data_band);
    let vector_data = array.into_raw_vec();

    let output_raster = Buffer::<f64> {
        size: band_size,
        data: vector_data,
    };

    let mut output_rb = output_dataset.rasterband(1).unwrap();
    let res = output_rb.write((0, 0), band_size, &output_raster);

    assert!(res.is_ok());

}


pub struct RasterWriteOptions<'a> {
    pub rows: &'a isize,
    pub cols: &'a isize,
    pub srs: &'a SpatialRef,
    pub transform: &'a GeoTransform,
    pub proj: &'a str,
}


fn read_array_from_rasterband<T: Copy + GdalType>(
    band: RasterBand
) -> Array1<T> {

    let rv: Buffer<T> = if let Ok(rv) = band.read_as::<T>(
        (0, 0), band.size(), band.size(), None
    ) {rv} else {todo!()};

    let arr: Array1<T> = Array::from_vec(rv.data);

    arr

}