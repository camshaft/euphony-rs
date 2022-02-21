#[cfg(test)]
macro_rules! generator_test {
    ($test_name:ident, $fill:expr) => {
        #[test]
        fn $test_name() {
            #[allow(unused_imports)]
            use crate::{
                buffer::{Batch, TestBatch},
                signal::{generator::*, Signal, SignalExt as _},
            };

            let mut result = None;
            TestBatch::buffer::<f32, _, _>($fill, |out| {
                result = Some(out.to_vec());
            });
            let result = result.unwrap();

            if !crate::signal::generator::testing::plot(stringify!($test_name), &result) {
                insta::assert_debug_snapshot!(result);
            }
        }
    };
}

#[cfg(test)]
mod testing {
    use crate::buffer::{Batch, TestBatch};
    use plotters::prelude::*;
    const CHART_DIMENSIONS: (u32, u32) = (1024, 768);

    pub fn plot(name: &str, results: &[f32]) -> bool {
        let dir = if let Ok(dir) = std::env::var("GENERATOR_CHART_DIR") {
            dir
        } else {
            return false;
        };

        let mut path = std::path::PathBuf::new();
        path.push(dir);
        std::fs::create_dir_all(&path).unwrap();

        path.push(name);
        path.set_extension("svg");

        let root_area = SVGBackend::new(&path, CHART_DIMENSIONS).into_drawing_area();
        root_area.fill(&WHITE).expect("Could not fill chart");
        root_area
            .titled(name, ("sans-serif", 40))
            .expect("Could not add title");

        let mut ctx = ChartBuilder::on(&root_area)
            .set_label_area_size(LabelAreaPosition::Left, 10)
            .set_label_area_size(LabelAreaPosition::Bottom, 10)
            .margin(20)
            .margin_top(40)
            .build_cartesian_2d(0..TestBatch::LEN, -1.1f32..1.1f32)
            .expect("Could not build chart");

        ctx.configure_mesh()
            .draw()
            .expect("Could not configure mesh");

        ctx.draw_series(PointSeries::of_element(
            results.iter().copied().enumerate(),
            2,
            &RED,
            &|c, s, st| Circle::new(c, s, st.filled()),
        ))
        .unwrap();

        true
    }
}

#[macro_use]
mod phase;
use phase::Phase;

mod sine;
pub use sine::*;

mod pulse;
pub use pulse::{pulse as square, *};

mod sawtooth;
pub use sawtooth::*;

mod triangle;
pub use triangle::*;

mod silence;
pub use silence::*;
