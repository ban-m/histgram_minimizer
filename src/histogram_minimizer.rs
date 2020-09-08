pub fn minimize(mut h1:Vec<f64>,mut h2:Vec<f64>) -> (f64,f64){
    // compute argmin(P(h)/C(h)),where P(h) = #(h1:<h),C=#(h2:<h)
    // return (min(P(h)/C(h)),argmin(--))
    // usually, P(h) is negative data,C(h) is positive data
    h1.sort_by(|a,b| a.partial_cmp(b).unwrap());
    h2.sort_by(|a,b| a.partial_cmp(b).unwrap());
    let mut h1_cdf= 0;
    let mut h2_cdf= 0;
    let mut result = vec![];
    let mut j = 0;//coursor pointing h1 index.
    for i in 0..h2.len(){
        // reult[i] := P(h)/C(h),where h = h2[i]
        //h2_cdf += 1./h2.len() as f64;
        h2_cdf += 1;
        while j < h1.len() && h1[j] <= h2[i] {
            //h1_cdf += 1./h1.len() as f64;
            h1_cdf += 1;
            j += 1;
        }
        result.push((h1_cdf * h2.len()) as f64 /(h2_cdf *h1.len()) as f64);
    }
    // for &x in result.iter(){
    //     eprint!("{} ",x);
    // }
    // eprintln!("");
    result.into_iter().enumerate().fold((1.,0.),|(min,argmin),(index,cdfratio)|{
        if min >= cdfratio && cdfratio != 0.{
            (cdfratio,h2[index])
        }else{
            (min,argmin)
        }})
}
