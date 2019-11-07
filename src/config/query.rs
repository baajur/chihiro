use super::TestConfig;
use rand::Rng;
use serde::Deserialize;
use std::{collections::HashMap, convert::TryFrom, time::Duration};

#[derive(Debug)]
pub struct QueryConfig {
    pub(super) queries: Vec<Query>,
    pub(super) duration: Duration,
    pub(super) identifier: String,
    pub(super) elastic_endpoint: String,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct QueryVariable {
    pub(super) minimum: u64,
    pub(super) maximum: u64,
}

#[derive(Debug)]
pub struct Query {
    pub(super) name: String,
    pub(super) query: String,
    pub(super) rps: &'static [u64],
    pub(super) variables: HashMap<String, QueryVariable>,
}

impl Query {
    pub fn query(&self) -> String {
        let mut rng = rand::thread_rng();

        self.variables
            .iter()
            .fold(self.query.clone(), |acc, (name, var)| {
                let x = rng.gen_range(var.minimum, var.maximum);
                acc.replace(&format!("${}", name), &format!("{}", x))
            })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn rps(&self) -> &'static [u64] {
        self.rps
    }
}

impl QueryConfig {
    pub fn new(test_file: &str) -> crate::Result<Self> {
        let mut config = TestConfig::try_from(test_file)?;

        Ok(Self {
            queries: config.take_queries()?,
            duration: Duration::from_secs(config.duration_per_test),
            identifier: config.identifier,
            elastic_endpoint: config.elastic_endpoint,
        })
    }

    pub fn duration(&self) -> Duration {
        self.duration
    }

    pub fn elastic_endpoint(&self) -> &str {
        self.elastic_endpoint.as_str()
    }

    pub fn query_count(&self) -> usize {
        self.queries.len()
    }

    pub fn test_count(&self) -> usize {
        self.queries.iter().fold(0, |acc, q| acc + q.rps.len())
    }

    pub fn queries(&self) -> impl Iterator<Item = &Query> {
        self.queries.iter()
    }

    pub fn runs(&self) -> impl Iterator<Item = (&Query, u64)> {
        self.queries()
            .flat_map(move |q| q.rps().iter().map(move |r| (q, *r)))
    }
}
