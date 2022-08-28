use std::path::Path;
use ndarray::Array;
use gdal::raster::{Buffer, RasterBand};
use gdal::{Dataset, Driver};


fn main() {

    // To get GDAL bindings to compile...
    // apt-get update
    // apt-get install -y gdal-bin
    // apt-get install -y libgdal-dev
    // export CPLUS_INCLUDE_PATH=/usr/include/gdal
    // export C_INCLUDE_PATH=/usr/include/gdal

    let input_filepath: String = std::env::args().nth(1).expect("No input filepath given");
    let output_filepath: String = std::env::args().nth(2).expect("No input filepath given");
    let red_band_idx: String = std::env::args().nth(3).expect("No Red band index given");
    let nir_band_idx: String = std::env::args().nth(4).expect("No NIR band index given");

    let red_band_id: u8 = red_band_idx.trim().parse().expect("Invalid input for integer.");
    let nir_band_id: u8 = nir_band_idx.trim().parse().expect("Invalid input for integer.");

    let input_path = Path::new(&input_filepath);
    let input_dataset = Dataset::open(input_path).unwrap();

    let red_band: RasterBand = input_dataset.rasterband(red_band_id.into()).unwrap();
    let nir_band: RasterBand = input_dataset.rasterband(nir_band_id.into()).unwrap();

    let rv = if let Ok(rv) = red_band.read_as::<f64>(
        (0, 0), red_band.size(), red_band.size(), None
    ) {rv} else {todo!()};
    let red_arr = Array::from(rv.data);
    let rv = if let Ok(rv) = nir_band.read_as::<f64>(
        (0, 0), nir_band.size(), nir_band.size(), None
    ) {rv} else {todo!()};
    let nir_arr = Array::from(rv.data);

    let ratio_arr = red_arr / nir_arr;

    let output_path = Path::new(&output_filepath);

    let output_driver = Driver::get("GTIFF").unwrap();
    let mut output_dataset = output_driver.create_with_band_type::<f64, _>(
        output_path, 
        nir_band.size().0.try_into().unwrap(), 
        nir_band.size().1.try_into().unwrap(), 
        1
    ).unwrap();

    let srs = input_dataset.spatial_ref().unwrap();
    let transform = input_dataset.geo_transform().unwrap();

    output_dataset.set_projection(&input_dataset.projection());
    output_dataset.set_spatial_ref(&srs);
    output_dataset.set_geo_transform(&transform);

    let ratio_vec = ratio_arr.into_raw_vec();

    let output_raster = Buffer::<f64> {
        size: nir_band.size(),
        data: ratio_vec,
    };

    let mut output_rb = output_dataset.rasterband(1).unwrap();

    let res = output_rb.write((0, 0), nir_band.size(), &output_raster);

    assert!(res.is_ok());

}
