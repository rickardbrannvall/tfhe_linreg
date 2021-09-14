#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

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

    println!("loading LWE key... \n");
    let path = "keys";
    let sk0_LWE_path = format!("{}/sk0_LWE.json",path);
    let sk0 = LWESecretKey::load(&sk0_LWE_path).unwrap();    
    
    // Read parameters of the linear regression
    
    let file = File::open("data/coeff.csv")?;
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);
    let coeff: Array2<f64> = reader.deserialize_array2_dynamic().unwrap();
    let coeff = coeff.column(0).to_vec();
    println!("{:?}",coeff);
    
    let file = File::open("data/intercept.csv")?;
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);
    let intercept: Array2<f64> = reader.deserialize_array2_dynamic().unwrap();
    let intercept = intercept.column(0).to_vec();
    println!("{:?}",intercept);
    
    let offset = vec![intercept[0]/coeff.len() as f64; coeff.len()];    
    
    let max_constant: f64 = 0.5;
    let nb_bit_padding = 8;

    let file = File::open("data/y_test.csv").expect("could not read data file");
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);
    let y_test: Array2<f64> = reader.deserialize_array2_dynamic().expect("conversion error");
    println!("{:?}",y_test.dim());
    
    let N = y_test.shape()[0];
    println!("Number of data rows: {}", N);
    
    for i in 0..N {
        if false && i==5 {
            break;
        };
        
        let encfile = format!("data/X_test/{}.enc",i);
        println!("\n{}", encfile);
        let mut terms = VectorLWE::load(&encfile).unwrap();
        terms.mul_constant_with_padding_inplace(&coeff, max_constant, nb_bit_padding)?;
        println!("terms {:?}", terms.decrypt_decode(&sk0).unwrap());

        terms.add_constant_static_encoder_inplace(&offset)?; 
        println!("terms {:?}", terms.decrypt_decode(&sk0).unwrap());
  
        let price = terms.sum_with_new_min(0.).unwrap();
        println!("pred price {:?}", price.decrypt_decode(&sk0).unwrap());
        //price.pp(); 

        println!("true price {:?}", y_test.row(i).to_vec());

        let encfile = format!("data/y_test0/{}.enc",i);
        price.save(&encfile).unwrap();
    };
    
    Ok(())
}