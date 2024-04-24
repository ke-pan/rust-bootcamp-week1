use crate::Executor;
use clap::Args;
use std::path::PathBuf;

#[derive(Debug, Args)]
pub struct HttpArgs {
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
    #[arg(short, long, default_value = ".")]
    pub dir: PathBuf,
}

impl Executor for HttpArgs {
    async fn execute(&self) -> anyhow::Result<()> {
        crate::serve(self.dir.clone(), self.port).await
    }
}
