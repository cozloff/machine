use clap::{Args, Subcommand};

#[derive(Args)]
pub struct TestArgs {
    #[command(subcommand)]
    pub command: Option<TestCommand>,
}

#[derive(Clone, Copy, Subcommand)]
pub enum TestCommand {
    /// Print the gumball machine test screen
    Art,
    /// Run test command one
    One,
    /// Run test command two
    Two,
    /// Run test command three
    Three,
    /// Run test command four
    Four,
    /// Run test command five
    Five,
    /// Run test command six
    Six,
    /// Run test command seven
    Seven,
    /// Run test command eight
    Eight,
    /// Run test command nine
    Nine,
    /// Run test command ten
    Ten,
}