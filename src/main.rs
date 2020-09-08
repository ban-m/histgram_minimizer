extern crate rayon;
extern crate csv;
mod histogram_minimizer;

type Record = (usize,f64,i64);


fn setup_csv(record:&str,refsize:usize)->csv::Result<Vec<f64>>{
    use std::path::Path;
    let mut result = vec![];
    let mut rdr = csv::Reader::from_path(&Path::new(record))?;
    for record in rdr.deserialize(){
        let record :Record = record?;
        if record.0 == refsize {
            result.push(record.1);
        }
    }
    Ok(result)
}

fn main(){
    let args :Vec<_> = std::env::args().collect();
    eprintln!("{:?}",args);
    let refsize :usize = args[1].parse().unwrap();
    let refsize = refsize * 1_000;
    let correct = setup_csv(&args[2],refsize).unwrap();
    let wrong = setup_csv(&args[3],refsize).unwrap();
    eprintln!("setup done");
    let (min,argmin) = histogram_minimizer::minimize(wrong,correct);
    println!("{},{},{}",refsize,min,argmin);
}
