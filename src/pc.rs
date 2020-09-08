use histogram_minimizer::minimize;

#[derive(Debug)]
#[repr(C)]
pub struct PC{
    threshold:f64,
}

impl PC {
    pub fn new(h1:Vec<f64>,h2:Vec<f64>) -> PC{
        let (_,threshold) = minimize(h1,h2);
        let threshold = threshold;
        //eprintln!("hisgogram_minimizer set threshold:{}",threshold);
        PC{threshold}
    }
    pub fn classify(&self,score:f64)-> bool{
        score <= self.threshold //in under threshold, take it.
    }
    pub fn get_threshold(&self)->f64{
        self.threshold
    }
    pub fn predict_test(&self,testset:&Vec<(f64,bool)>) -> u64{
        // predict each data in testset, then return how good the predict was.
        testset.iter().map(|&(query,answer)| self.classify(query) == answer).
            map(|result| if result {1}else{0}).sum()
    }
}
