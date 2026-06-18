pub struct ConfusionMatrix {
    pub tp: f32,
    pub tn: f32,
    pub fp: f32,
    pub fn_: f32,
}

pub fn confusion_matrix(y: &Vec<f32>, y_hat: &Vec<f32>) -> ConfusionMatrix {
        let (
        mut tp,
        mut tn,
        mut fp,
        mut fn_
    ) = (0.0, 0.0, 0.0, 0.0);
    y.iter().zip(y_hat.iter()).for_each(|(&real, &predito)| match (real as u8, predito as u8) {
        (1, 1) => tp += 1.0,
        (0, 0) => tn += 1.0,
        (0, 1) => fp += 1.0,
        (1, 0) => fn_ += 1.0,
        _      => {}
    });
    ConfusionMatrix { tp, tn, fp, fn_ }
}

// ACC
pub fn accuracy_fn(
    y: &Vec<f32>,
    y_hat: &Vec<f32>,
) -> f32 {
    let cm = confusion_matrix(y, y_hat);
    (cm.tp + cm.tn) / (cm.tp + cm.tn + cm.fp + cm.fn_)
}

// RECALL
pub fn recall_fn(
    y: &Vec<f32>,
    y_hat: &Vec<f32>,
) -> f32 {
    let cm = confusion_matrix(y, y_hat);
    cm.tp / (cm.tp + cm.fn_)  // de todos que são 1, quantos eu estou encontrando e falando que são 1
}

// PRECISION
pub fn precision_fn(
    y: &Vec<f32>,
    y_hat: &Vec<f32>,
) -> f32 {
    let cm = confusion_matrix(y, y_hat);
    cm.tp / (cm.tp + cm.fp)  // de todos que o modelo diz ser 1 quem realmente é 1
}

pub fn f1_score_fn(
    y: &Vec<f32>,
    y_hat: &Vec<f32>,
) -> f32 {
    let precision: f32 = precision_fn(y, y_hat);
    let recall: f32 = recall_fn(y, y_hat);

    (2.0 * precision * recall) / (precision + recall)  // é uma média harmonica
}