#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_variables)]

//extern crate csv;
//extern crate ndarray;
//extern crate ndarray_csv;

use concrete::*;

use csv::{ReaderBuilder};
use ndarray::{Array, Array2};
use ndarray_csv::{Array2Reader};
use std::error::Error;
use std::fs::File;

//use glob::glob;

fn main() -> Result<(), Box<dyn Error>> {
    let path = "keys_80_2048_60_64";
    //let path = "keys";

    println!("Load parameters for linear regression ... ");
    let file = File::open("data/coeff.csv")?;
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);
    let coeff: Array2<f64> = reader.deserialize_array2_dynamic().unwrap();
    let coeff = coeff.column(0).to_vec();
    println!("Number of coefficients {}",coeff.len());
    
    let file = File::open("data/intercept.csv")?;
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);
    let intercept: Array2<f64> = reader.deserialize_array2_dynamic().unwrap();
    let intercept = intercept.column(0).to_vec();
    println!("Intercept {:?}",intercept.len());
    
    //let offset = vec![intercept[0]/coeff.len() as f64; coeff.len()];    
    
    //let max_constant: f64 = 0.5;
    //let nb_bit_padding = 8;
    let mut N = 102;

    // This is only for debugging
    //println!("DEBUG: Load LWE secret key 0 ... ");
    //let sk0_LWE_path = format!("{}/sk0_LWE.json",path);
    //let sk0 = LWESecretKey::load(&sk0_LWE_path).unwrap();    
    
    println!("DEBUG: Load LWE secret key 1 ... ");
    let sk1_LWE_path = format!("{}/sk1_LWE.json",path);
    let sk1 = LWESecretKey::load(&sk1_LWE_path).unwrap();    
    
    println!("DEBUG: Load ground truth data ... ");
    let file = File::open("data/y_test.csv").expect("could not read data file");
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);
    let y_test: Array2<f64> = reader.deserialize_array2_dynamic().expect("conversion error");
    // N = y_test.shape()[0];
    // **************************

    
    let enc = Encoder::new(-27., 23., 8, 1)?;

    println!("Load Bootstrapping Key 01 ... \n");
    let bsk01_path = format!("{}/bsk01_LWE.json", path);
    let bsk01 = LWEBSK::load(&bsk01_path);
    
    println!("Number of data rows: {}", N);
    //N = 1;
    
    for i in 0..N {        
        let encfile = format!("data/X_test1/{}.enc",i);
        println!("{}", encfile);
        
        let features = VectorLWE::load(&encfile).unwrap();
                
        let mut temp = features.bootstrap_nth_with_function(&bsk01, |x| coeff[0] * x, &enc,0)?;
        println!("DEBUG: term {:?}", temp.decrypt_decode(&sk1).unwrap());
                
        for j in 1..coeff.len() {
            let term = features.bootstrap_nth_with_function(&bsk01, |x| coeff[j] * x, &enc, j)?;
            println!("DEBUG: term {:?}", term.decrypt_decode(&sk1).unwrap());
            temp.add_with_new_min_inplace(&term, &vec![-32.])?;
            println!("DEBUG: temp {:?}", temp.decrypt_decode(&sk1).unwrap());
        }
        
        temp.add_constant_dynamic_encoder_inplace(&intercept)?; 
        println!("DEBUG: temp {:?}", temp.decrypt_decode(&sk1).unwrap());
        
        let encfile = format!("data/y_test1/{}.enc",i);
        temp.save(&encfile).unwrap();
    };
    
    Ok(())
}