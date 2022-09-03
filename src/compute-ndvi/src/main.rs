use std::path::Path;
use ndarray::{Array, Array1};
use gdal::raster::{Buffer, RasterBand};
use gdal::{Dataset, Driver};


// Compute NDVI as a float64 Array
fn compute_ndvi(red_array: Array1<f64>, nir_array: Array1<f64>) -> Array1<f64> {
    let top: Array1<f64> = &nir_array - &red_array; 
    let bottom: Array1<f64> = &nir_array + &red_array;
    let ndvi: Array1<f64> = top / bottom;
    ndvi
}


// Takes an input GeoTIFF and reads the Redand NIR bands, computes an NDVI array and writes
// it to a user-specified output GeoTIFF.
fn main() {

    // To get GDAL bindings to compile...
    // apt-get update
    // apt-get install -y gdal-bin
    // apt-get install -y libgdal-dev
    // export CPLUS_INCLUDE_PATH=/usr/include/gdal
    // export C_INCLUDE_PATH=/usr/include/gdal

    // Get the inputs from the command line
    let input_filepath: String = std::env::args().nth(1).expect("No input filepath given");
    let output_filepath: String = std::env::args().nth(2).expect("No input filepath given");
    let red_band_idx: String = std::env::args().nth(3).expect("No Red band index given");
    let nir_band_idx: String = std::env::args().nth(4).expect("No NIR band index given");

    // Convert the inputs for band index from string to byte
    let red_band_id: u8 = red_band_idx.trim().parse().expect("Invalid input for integer.");
    let nir_band_id: u8 = nir_band_idx.trim().parse().expect("Invalid input for integer.");

    // Open in the input dataset
    let input_path = Path::new(&input_filepath);
    let input_dataset = Dataset::open(input_path).unwrap();

    // Get the Red and NIR bands from the dataset
    let red_band: RasterBand = input_dataset.rasterband(red_band_id.into()).unwrap();
    let nir_band: RasterBand = input_dataset.rasterband(nir_band_id.into()).unwrap();

    // Read the data into arrays from the Red and NIR bands
    let rv: Buffer<f64> = if let Ok(rv) = red_band.read_as::<f64>(
        (0, 0), red_band.size(), red_band.size(), None
    ) {rv} else {todo!()};
    let red_arr: Array1<f64> = Array::from_vec(rv.data);
    let rv: Buffer<f64> = if let Ok(rv) = nir_band.read_as::<f64>(
        (0, 0), nir_band.size(), nir_band.size(), None
    ) {rv} else {todo!()};
    let nir_arr: Array1<f64> = Array::from_vec(rv.data);

    // Compute NDVI array
    let ndvi_arr = compute_ndvi(nir_arr, red_arr);

    let output_path = Path::new(&output_filepath);

    // Create the new output dataset to write the NDVI data to
    let output_driver = Driver::get("GTIFF").unwrap();
    let mut output_dataset = output_driver.create_with_band_type::<f64, _>(
        output_path, 
        nir_band.size().0.try_into().unwrap(), 
        nir_band.size().1.try_into().unwrap(), 
        1
    ).unwrap();

    // Set the spatial reference attributes of the output dataset
    let srs = input_dataset.spatial_ref().unwrap();
    let transform = input_dataset.geo_transform().unwrap();

    output_dataset.set_projection(&input_dataset.projection()).expect("Error adding projection to output dataset!");
    output_dataset.set_spatial_ref(&srs).expect("Error adding spatial ref to output dataset!");
    output_dataset.set_geo_transform(&transform).expect("Error adding transform to output dataset!");

    // Write the NDVI data to the output dataset
    let ndvi_vec = ndvi_arr.into_raw_vec();

    let output_raster = Buffer::<f64> {
        size: nir_band.size(),
        data: ndvi_vec,
    };

    let mut output_rb = output_dataset.rasterband(1).unwrap();

    let res = output_rb.write((0, 0), nir_band.size(), &output_raster);

    assert!(res.is_ok());

}
