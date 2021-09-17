#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

//extern crate csv;
//extern crate ndarray;
//extern crate ndarray_csv;

use concrete::*;

use csv::{ReaderBuilder, WriterBuilder};
use ndarray::{Array, Array2};
use ndarray_csv::{Array2Reader, Array2Writer};
use std::error::Error;
use std::fs::File;

// replace vector format with scalar

fn main() -> Result<(), CryptoAPIError> {
    let path = "keys_80_2048_60_64";
    //let path = "keys";

    println!("Load LWE secret key ... ");
    let sk0_LWE_path = format!("{}/sk0_LWE.json",path);
    let sk0 = LWESecretKey::load(&sk0_LWE_path).unwrap();    

    println!("Load ground truth data ... ");
    let file = File::open("data/y_test.csv").expect("could not read data file");
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);
    let y_test: Array2<f64> = reader.deserialize_array2_dynamic().expect("conversion error");
    //println!("{:?}",y_test.dim());

    let N = y_test.shape()[0];
    println!("Number of data rows: {}", N);
    
    let mut y_pred = y_test.clone();
    
    println!("Load and decrypt predictions ...");
    for i in 0..N {
        if false && i==5 {
            break;
        };
        
        let encfile = format!("data/y_test1/{}.enc",i);
        let pred = VectorLWE::load(&encfile).unwrap();
        
        y_pred[[i,0]] = pred.decrypt_decode(&sk0).unwrap()[0];
    };
 
    //println!("{:?}",y_pred);
    
    let err = &y_pred - &y_test;
    println!("Mean Average Error: {}", err.mapv(|x| x.abs()).mean().unwrap());

    let file = File::create("data/y_test1/y_test.csv").expect("could not create file");
    let mut writer = WriterBuilder::new().has_headers(false).from_writer(file);
    writer.serialize_array2(&y_pred).expect("could not write file");    
    
    Ok(())
}
