//! Tests

use std::{
    fmt::{Debug, Display},
    ops::Sub,
};

use time::{Duration, Instant};

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

/// Test status
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
pub struct TestSuite {
    /// Test
    pub tests: Vec<Test>,
}

/// Statistics for a collection of tests
#[derive(Debug, Default)]
pub struct TestSuiteStats {
    /// Nb of tests ran
    pub total: usize,
    /// Nb of successful tests
    pub successful: usize,
    /// Nb of failed tests
    pub failed: usize,
    /// Average time
    pub average: Duration,
    /// Fastest time
    pub fastest: Duration,
    /// Slowest time
    pub slowest: Duration,
}

impl TestSuite {
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

    /// Returns the [TestCollStats]
    pub fn stats(&self) -> TestSuiteStats {
        let total = self.tests.len();
        let successful = self.tests.iter().filter(|t| t.status.is_some()).count();
        let failed = self.tests.iter().filter(|t| t.status.is_none()).count();
        let durations = self
            .tests
            .iter()
            .filter(|t| t.end.is_some())
            .map(|t| t.end.unwrap().sub(t.start))
            .collect::<Vec<_>>();
        let total_u32: u32 = total.try_into().unwrap();
        let average = durations.iter().sum::<Duration>() / total_u32;
        let fastest = *durations.iter().min().unwrap();
        let slowest = *durations.iter().max().unwrap();

        TestSuiteStats {
            total,
            successful,
            failed,
            average,
            fastest,
            slowest,
        }
    }
}

impl From<Vec<Test>> for TestSuite {
    fn from(value: Vec<Test>) -> Self {
        Self { tests: value }
    }
}

impl Display for TestSuiteStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Summary:       ")?;
        writeln!(f, "  Total:   {}", self.total)?;
        writeln!(f, "  Success:   {}", self.successful)?;
        writeln!(f, "  Failure:   {}", self.failed)?;
        writeln!(f, "  Average: {}ms", self.average.whole_milliseconds())?;
        writeln!(f, "  Slowest: {}ms", self.slowest.whole_milliseconds())?;
        writeln!(f, "  Fastest: {}ms", self.fastest.whole_milliseconds())?;
        Ok(())
    }
}
