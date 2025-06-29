use crate::internals::helpers::runtime_helper::Runtime;
use crate::internals::test_case::TestCase;

use super::*;
use foundation::prelude::*;
use orchestration::{
    api::{Orchestration, design::Design},
    common::DesignConfig,
};

pub struct SingleConcurrencyTest;

fn single_concurrency_design() -> Result<Design, CommonErrors> {
    let mut design = Design::new("SingleConcurrency".into(), DesignConfig::default());

    let t1_tag = design.register_invoke_fn("Function1".into(), function1)?;
    let t2_tag = design.register_invoke_fn("Function2".into(), function2)?;
    let t3_tag = design.register_invoke_fn("Function3".into(), function3)?;

    // Create a program with actions
    design.add_program(file!(), move |_design_instance, builder| {
        builder.with_body(
            SequenceBuilder::new()
                .with_step(
                    ConcurrencyBuilder::new()
                        .with_branch(Invoke::from_tag(&t1_tag))
                        .with_branch(Invoke::from_tag(&t2_tag))
                        .with_branch(Invoke::from_tag(&t3_tag))
                        .build(),
                )
                .with_step(JustLogAction::new("FinishAction"))
                .build(),
        );

        Ok(())
    });

    Ok(design)
}

/// Checks Concurrency Functions
impl TestCase for SingleConcurrencyTest {
    fn get_name(&self) -> &'static str {
        "single_concurrency"
    }

    fn run(&self, input: Option<String>) -> Result<(), String> {
        let mut rt = Runtime::new(&input).build();

        // Build Orchestration
        let orch = Orchestration::new()
            .add_design(single_concurrency_design().expect("Failed to create design"))
            .design_done();

        // Create programs
        let mut programs = orch.create_programs().unwrap();

        // Put programs into runtime and run them
        let _ = rt.block_on(async move {
            let _ = programs.programs.pop().unwrap().run_n(1).await;
            info!("Program finished running.");
            Ok(0)
        });

        Ok(())
    }
}

pub struct MultipleConcurrencyTest;

fn multiple_concurrency_design() -> Result<Design, CommonErrors> {
    let mut design = Design::new("MultipleConcurrency".into(), DesignConfig::default());

    let t1_tag = design.register_invoke_fn("Function1".into(), function1)?;
    let t2_tag = design.register_invoke_fn("Function2".into(), function2)?;
    let t3_tag = design.register_invoke_fn("Function3".into(), function3)?;
    let t4_tag = design.register_invoke_fn("Function4".into(), function4)?;
    let t5_tag = design.register_invoke_fn("Function5".into(), function5)?;
    let t6_tag = design.register_invoke_fn("Function6".into(), function6)?;
    // Create a program with actions
    design.add_program(file!(), move |_design_instance, builder| {
        builder.with_body(
            SequenceBuilder::new()
                .with_step(
                    ConcurrencyBuilder::new()
                        .with_branch(Invoke::from_tag(&t1_tag))
                        .with_branch(Invoke::from_tag(&t2_tag))
                        .with_branch(Invoke::from_tag(&t3_tag))
                        .build(),
                )
                .with_step(JustLogAction::new("IntermediateAction"))
                .with_step(
                    ConcurrencyBuilder::new()
                        .with_branch(Invoke::from_tag(&t4_tag))
                        .with_branch(Invoke::from_tag(&t5_tag))
                        .with_branch(Invoke::from_tag(&t6_tag))
                        .build(),
                )
                .with_step(JustLogAction::new("FinishAction"))
                .build(),
        );

        Ok(())
    });

    Ok(design)
}

/// Checks Concurrency Functions
impl TestCase for MultipleConcurrencyTest {
    fn get_name(&self) -> &'static str {
        "multiple_concurrency"
    }

    fn run(&self, input: Option<String>) -> Result<(), String> {
        let mut rt = Runtime::new(&input).build();

        // Build Orchestration
        let orch = Orchestration::new()
            .add_design(multiple_concurrency_design().expect("Failed to create design"))
            .design_done();

        // Create programs
        let mut programs = orch.create_programs().unwrap();

        // Put programs into runtime and run them
        let _ = rt.block_on(async move {
            let _ = programs.programs.pop().unwrap().run_n(1).await;
            info!("Program finished running.");
            Ok(0)
        });

        Ok(())
    }
}

pub struct NestedConcurrencyTest;

fn nested_concurrency_design() -> Result<Design, CommonErrors> {
    let mut design = Design::new("NestedConcurrency".into(), DesignConfig::default());

    let t1_tag = design.register_invoke_fn("OuterFunction1".into(), outer_function1)?;
    let t2_tag = design.register_invoke_fn("InnerFunction1".into(), inner_function1)?;
    let t3_tag = design.register_invoke_fn("InnerFunction2".into(), inner_function2)?;
    let t4_tag = design.register_invoke_fn("OuterFunction2".into(), outer_function2)?;

    // Create a program with actions
    design.add_program(file!(), move |_design_instance, builder| {
        builder.with_body(
            SequenceBuilder::new()
                .with_step(
                    ConcurrencyBuilder::new()
                        .with_branch(Invoke::from_tag(&t1_tag))
                        .with_branch(
                            ConcurrencyBuilder::new()
                                .with_branch(Invoke::from_tag(&t2_tag))
                                .with_branch(Invoke::from_tag(&t3_tag))
                                .build(),
                        )
                        .with_branch(Invoke::from_tag(&t4_tag))
                        .build(),
                )
                .with_step(JustLogAction::new("FinishAction"))
                .build(),
        );

        Ok(())
    });

    Ok(design)
}

/// Checks Concurrency Functions
impl TestCase for NestedConcurrencyTest {
    fn get_name(&self) -> &'static str {
        "nested_concurrency"
    }

    fn run(&self, input: Option<String>) -> Result<(), String> {
        let mut rt = Runtime::new(&input).build();

        // Build Orchestration
        let orch = Orchestration::new()
            .add_design(nested_concurrency_design().expect("Failed to create design"))
            .design_done();

        // Create programs
        let mut programs = orch.create_programs().unwrap();

        // Put programs into runtime and run them
        let _ = rt.block_on(async move {
            let _ = programs.programs.pop().unwrap().run_n(1).await;
            info!("Program finished running.");
            Ok(0)
        });

        Ok(())
    }
}
