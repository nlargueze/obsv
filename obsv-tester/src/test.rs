//! Tests

use std::{fmt::Debug, ops::Sub};

use time::Instant;

/// A single test
#[derive(Debug)]
pub struct Test {
    // Test index
    pub idx: usize,
    // Start time
    pub start: Instant,
    // End time
    pub end: Option<Instant>,
    // Status
    pub status: Option<TestStatus>,
}

/// Test result
#[derive(Debug)]
pub enum TestStatus {
    // OK
    Ok,
    // Err
    Err,
}

impl Test {
    /// Instantiates a new [Test]
    pub fn start(idx: usize) -> Self {
        let start = Instant::now();

        Self {
            idx,
            start,
            end: None,
            status: None,
        }
    }

    /// Sets the test result
    pub fn set_result(&mut self, result: TestStatus) {
        self.end = Some(Instant::now());
        self.status = Some(result);
    }

    /// Sets the test result as OK
    pub fn set_ok(&mut self) {
        self.set_result(TestStatus::ok());
    }

    /// Sets the test result as ERR
    pub fn set_err(&mut self) {
        self.set_result(TestStatus::err());
    }
}

impl TestStatus {
    /// Creates a new OK result
    pub fn ok() -> Self {
        Self::Ok
    }

    /// Creates a new Err result
    pub fn err() -> Self {
        Self::Err
    }
}

/// A collection of tests
#[derive(Debug, Default)]
pub struct TestCollection {
    /// Test
    pub tests: Vec<Test>,
}

impl TestCollection {
    /// Instantiates a new [TestCollection]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a new test
    pub fn add_test(&mut self, test: Test) {
        self.tests.push(test);
    }

    /// Adds a new test with its index
    pub fn start_test(&mut self, idx: usize) {
        let test = Test::start(idx);
        self.add_test(test)
    }

    /// Returns the test at index i
    pub fn get(&self, i: usize) -> Option<&Test> {
        self.tests.get(i)
    }

    /// Prints the results to stdout
    pub fn print(&self) {
        let total = self.tests.len();
        let success = self.tests.iter().filter(|t| t.status.is_some()).count();
        let failure = self.tests.iter().filter(|t| t.status.is_none()).count();
        let durations = self
            .tests
            .iter()
            .filter(|t| t.end.is_some())
            .map(|t| t.end.unwrap().sub(t.start))
            .collect::<Vec<_>>();
        let slowest = durations.iter().min().unwrap();
        let fastest = durations.iter().max().unwrap();

        println!("Summary:       ");
        println!("  Total:   {}", total);
        println!("  Success:   {}", success);
        println!("  Failure:   {}", failure);
        println!("  Slowest: {}ms", slowest.whole_milliseconds());
        println!("  Fastest: {}ms", fastest.whole_milliseconds());
    }
}

impl From<Vec<Test>> for TestCollection {
    fn from(value: Vec<Test>) -> Self {
        Self { tests: value }
    }
}
