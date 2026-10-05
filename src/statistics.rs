use crate::measurer::Sample;

pub struct Statistics {
    pub samples: Vec<Sample>
}

impl Statistics {
    pub fn new(samples: Vec<Sample>) -> Statistics {
        Statistics { samples }
    }

    pub fn write_file(self) {
        let mut wtr = csv::Writer::from_path("result.csv").unwrap();

        for sample in self.samples.into_iter() {
            wtr.serialize(sample).expect("TODO: panic message");
        }

        wtr.flush().unwrap();
    }
}