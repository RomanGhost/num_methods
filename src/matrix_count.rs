pub fn lower_matrix(a: Vec<Vec<f32>>, b: Vec<f32>) ->Vec<f32> {
    let N:usize = b.len();

    let mut x=vec![0.0; N];
    x[0] = b[0]/a[0][0];
    let mut summ:f32;

    for i in (1..N){
        summ = 0.0;
        for j in (0..i){
            summ += x[j] * a[i][j];
        }
        x[i] = (b[i]-summ)/a[i][i];
    }

    return x;
}

pub fn upper_matrix(a: Vec<Vec<f32>>, b: Vec<f32>) ->Vec<f32> {
    let N:usize = b.len();

    let mut x=vec![0.0; N];
    x[N-1] = b[N-1]/a[N-1][N-1];
    let mut summ:f32;

    for i in (0..N-1).rev(){
        summ = 0.0;
        for j in (i+1..N){
            summ += x[j] * a[i][j];
        }
        x[i] = (b[i]-summ)/a[i][i];
    }

    return x;
}