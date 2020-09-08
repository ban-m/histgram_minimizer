.PHONY=clean

histogram_minimizer:
	cargo build --release 
	qsub histogram_minimizer_chiba.job
	qsub histogram_minimizer_sub.job
	qsub histogram_minimizer_mock.job

setup:
	cp ~/work/knn_predictor/result/Chiba* ./data/
	cp ~/work/knn_predictor/result/Sub* ./data/

test:
	cargo build --release 
	qsub test.job
