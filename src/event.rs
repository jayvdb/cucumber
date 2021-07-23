//! Key occurrences in the lifecycle of a Cucumber execution.
//!
//! The top-level enum here is [`Cucumber`].
//!
//! Each event enum contains variants indicating what stage of execution
//! [`Runner`] is at and, variants with detailed content about the precise
//! sub-event.
//!
//! [`Runner`]: crate::Runner

#![allow(clippy::large_enum_variant)]

use std::any::Any;

/// Top-level cucumber run event.
#[derive(Debug)]
pub enum Cucumber<World> {
    /// Event for a `Cucumber` execution started.
    Started,

    /// [`Feature`] event.
    Feature(gherkin::Feature, Feature<World>),

    /// Event for a `Cucumber` execution finished.
    Finished,
}

/// Alias for a [`catch_unwind()`] error.
///
/// [`catch_unwind()`]: std::panic::catch_unwind()
pub type PanicInfo = Box<dyn Any + Send + 'static>;

impl<World> Cucumber<World> {
    /// Constructs event of a [`Feature`] being started.
    ///
    /// [`Feature`]: gherkin::Feature
    #[must_use]
    pub fn feature_started(feature: gherkin::Feature) -> Self {
        Cucumber::Feature(feature, Feature::Started)
    }

    /// Constructs event of a [`Rule`] being started.
    ///
    /// [`Rule`]: gherkin::Rule
    #[must_use]
    pub fn rule_started(
        feature: gherkin::Feature,
        rule: gherkin::Rule,
    ) -> Self {
        Cucumber::Feature(feature, Feature::Rule(rule, Rule::Started))
    }

    /// Constructs event of a finished [`Feature`].
    ///
    /// [`Feature`]: gherkin::Feature
    #[must_use]
    pub fn feature_finished(feature: gherkin::Feature) -> Self {
        Cucumber::Feature(feature, Feature::Finished)
    }

    /// Constructs event of a finished [`Rule`].
    ///
    /// [`Rule`]: gherkin::Rule
    #[must_use]
    pub fn rule_finished(
        feature: gherkin::Feature,
        rule: gherkin::Rule,
    ) -> Self {
        Cucumber::Feature(feature, Feature::Rule(rule, Rule::Finished))
    }

    /// Constructs [`Cucumber`] event from a [`Scenario`] and it's path.
    #[must_use]
    pub fn scenario(
        feature: gherkin::Feature,
        rule: Option<gherkin::Rule>,
        scenario: gherkin::Scenario,
        event: Scenario<World>,
    ) -> Self {
        #[allow(clippy::option_if_let_else)] // use of moved value: `ev`
        if let Some(r) = rule {
            Cucumber::Feature(
                feature,
                Feature::Rule(r, Rule::Scenario(scenario, event)),
            )
        } else {
            Cucumber::Feature(feature, Feature::Scenario(scenario, event))
        }
    }
}

/// Event specific to a particular [Feature]
///
/// [Feature]: (https://cucumber.io/docs/gherkin/reference/#feature)
#[derive(Debug)]
pub enum Feature<World> {
    /// Event for a [`Feature`] execution started.
    ///
    /// [`Feature`]: gherkin::Feature
    Started,

    /// [`Rule`] event.
    Rule(gherkin::Rule, Rule<World>),

    /// [`Scenario`] event.
    Scenario(gherkin::Scenario, Scenario<World>),

    /// Event for a [`Feature`] execution finished.
    ///
    /// [`Feature`]: gherkin::Feature
    Finished,
}

/// Event specific to a particular [Rule]
///
/// [Rule]: (https://cucumber.io/docs/gherkin/reference/#rule)
#[derive(Debug)]
pub enum Rule<World> {
    /// Event for a [`Rule`] execution started.
    ///
    /// [`Rule`]: gherkin::Rule
    Started,

    /// [`Scenario`] event.
    Scenario(gherkin::Scenario, Scenario<World>),

    /// Event for a [`Rule`] execution finished.
    ///
    /// [`Rule`]: gherkin::Rule
    Finished,
}

/// Event specific to a particular [Scenario]
///
/// [Scenario]: https://cucumber.io/docs/gherkin/reference/#example
#[derive(Debug)]
pub enum Scenario<World> {
    /// Event for a [`Scenario`] execution started.
    ///
    /// [`Scenario`]: gherkin::Scenario
    Started,

    /// [`Background`] [`Step`] event.
    ///
    /// [`Background`]: gherkin::Background
    Background(gherkin::Step, Step<World>),

    /// [`Step`] event.
    Step(gherkin::Step, Step<World>),

    /// Event for a [`Scenario`] execution finished.
    ///
    /// [`Scenario`]: gherkin::Scenario
    Finished,
}

impl<World> Scenario<World> {
    /// Event of a [`Step`] being started.
    ///
    /// [`Step`]: gherkin::Step
    pub fn step_started(step: gherkin::Step) -> Self {
        Scenario::Step(step, Step::Started)
    }

    /// Event of a [`Background`] [`Step`] being started.
    ///
    /// [`Background`]: gherkin::Background
    /// [`Step`]: gherkin::Step
    pub fn background_step_started(step: gherkin::Step) -> Self {
        Scenario::Background(step, Step::Started)
    }

    /// Event of a passed [`Step`].
    ///
    /// [`Step`]: gherkin::Step
    pub fn step_passed(step: gherkin::Step) -> Self {
        Scenario::Step(step, Step::Passed)
    }

    /// Event of a passed [`Background`] [`Step`].
    ///
    /// [`Background`]: gherkin::Background
    /// [`Step`]: gherkin::Step
    pub fn background_step_passed(step: gherkin::Step) -> Self {
        Scenario::Background(step, Step::Passed)
    }

    /// Event of a skipped [`Step`].
    ///
    /// [`Step`]: gherkin::Step
    pub fn step_skipped(step: gherkin::Step) -> Self {
        Scenario::Step(step, Step::Skipped)
    }
    /// Event of a skipped [`Background`] [`Step`].
    ///
    /// [`Background`]: gherkin::Background
    /// [`Step`]: gherkin::Step
    pub fn background_step_skipped(step: gherkin::Step) -> Self {
        Scenario::Background(step, Step::Skipped)
    }

    /// Event of a failed [`Step`].
    ///
    /// [`Step`]: gherkin::Step
    pub fn step_failed(
        step: gherkin::Step,
        world: World,
        info: PanicInfo,
    ) -> Self {
        Scenario::Step(step, Step::Failed(world, info))
    }

    /// Event of a failed [`Background`] [`Step`].
    ///
    /// [`Background`]: gherkin::Background
    /// [`Step`]: gherkin::Step
    pub fn background_step_failed(
        step: gherkin::Step,
        world: World,
        info: PanicInfo,
    ) -> Self {
        Scenario::Background(step, Step::Failed(world, info))
    }
}

/// Event specific to a particular [Step]
///
/// [Step]: (https://cucumber.io/docs/gherkin/reference/#step)
#[derive(Debug)]
pub enum Step<World> {
    /// Event for a [`Step`] execution started.
    ///
    /// [`Step`]: gherkin::Step
    Started,

    /// Event for a [`Step`] being skipped.
    ///
    /// That means there is no [`Regex`] matching [`Step`] in
    /// [`step::Collection`].
    ///
    /// [`Regex`]: regex::Regex
    /// [`step::Collection`]: crate::step::Collection
    /// [`Step`]: crate::Step
    /// [`Step`]: gherkin::Step
    Skipped,

    /// Event for a passed [`Step`].
    ///
    /// [`Step`]: gherkin::Step
    Passed,

    /// Event for a failed [`Step`].
    ///
    /// [`Step`]: gherkin::Step
    Failed(World, PanicInfo),
}
