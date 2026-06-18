mod vectors;
mod classification_metrics;


const SIZE: usize = 5000;
const THRESHOLD: f32 = 0.8;

fn main() {
    let y_class = vectors::create_label_vector(SIZE);
    let y_class_hat = vectors::create_predict_label_vector(SIZE);
    let y_class_hat_binary = vectors::threshold_fn(&y_class_hat, THRESHOLD);
    println!("Label: {:?}", &y_class[..5]);
    println!("Predict: {:?}", &y_class_hat_binary[..5]);
    println!("Proba: {:?}", &y_class_hat[..5]);

    let acc = classification_metrics::accuracy_fn(&y_class, &y_class_hat_binary);
    let recall = classification_metrics::recall_fn(&y_class, &y_class_hat_binary);
    let precision = classification_metrics::precision_fn(&y_class, &y_class_hat_binary);
    let f1_score = classification_metrics::f1_score_fn(&y_class, &y_class_hat_binary);
    println!("Accuracy: {}", acc);
    println!("Recall {}", recall);
    println!("Precision {}", precision);
    println!("F1 Score {}", f1_score);
    

    // let y_reg = vectors::create_target_vector(SIZE);
    // let y_reg_hat = vectors::create_predict_target_vector(&y_reg);

}
