#[derive(Default, Debug)]
struct RicciFlow {
    pub g_0: f64,
    pub g_ij: f64,
    pub ricci_ij: f64,
}

impl RicciFlow {
    /// Initialize
    pub const fn new() -> Self {
        Self {
            g_0: 1.0,
            g_ij: 10.0,
            ricci_ij: 0.1,
        }
    }

    /// compute metric flow gij
    pub fn flow(&mut self, time: f64) -> f64 {
        let c = 0.1;
        self.ricci_ij = c * self.g_0;
        self.g_0 - self.ricci_ij * 2.0 * time
    }

    #[allow(unused)]
    pub fn evolute_time(&mut self, max_step: usize) -> f64 {
        let dt = 0.001;
        let mut time = 0.0;

        for n in 0..max_step {
            self.g_ij = self.flow(time);
            time += dt;

            dbg!(self.g_ij);
        }
        self.g_ij
    }
}

#[test]
fn should_call_flow_for_einstein_manifold() {
    let mut ein = RicciFlow::new();

    let max_step = 100;
    ein.evolute_time(max_step);
}
