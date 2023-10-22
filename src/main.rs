extern crate linfa;
extern crate ndarray;

use linfa::prelude::*;
use linfa::{traits::Fit, DatasetBase};
use linfa_linear::LinearRegression;
use ndarray::prelude::*;

fn main() {
    // Example data
    let x = arr2(&[[1.0], [2.0], [3.0], [4.0], [5.0]]);
    let y = arr1(&[2.0, 4.0, 5.5, 7.0, 8.5]);

    // Create a DatasetBase from the data
    let dataset = DatasetBase::new(x, y);

    // Create a linear regression model
    let model = LinearRegression::default();

    // Fit the model to the data
    let trained_model = model.fit(&dataset).expect("Error fitting the model");

    // Make predictions using the trained model
    let x_pred = arr2(&[[6.0]]);
    let y_pred = trained_model.predict(&x_pred);

    println!("Forecast: {:.2}", y_pred[0]);
}
