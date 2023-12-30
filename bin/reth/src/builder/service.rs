//! Customizable node builder service

use crate::{
    builder::{NodeConfig, NodeHandle},
    cli::ext::{RethCliExt, RethNodeCommandExt},
};
use reth_db::{
    database::Database,
    database_metrics::{DatabaseMetadata, DatabaseMetrics},
};
use reth_tasks::TaskExecutor;
use crate::builder::traits::PoolBuilder;

/// Declaratively construct a node.
///
/// [`NodeBuilder`] provides a [builder-like interface][builder] for composing
/// components of a node.
///
/// [builder]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct NodeBuilder<DB, Pool, Ext, State> {
    /// All settings for how the node should be configured.
    config: NodeConfig,
    /// State of the node builder process.
    state: State,
    /// Context for the node builder process.
    ctx: NodeBuilderContext<DB, Pool, Ext>,
}

impl<DB, Pool, Ext, State> NodeBuilder<DB, Pool, Ext, State> {

    /// Returns a reference to the node builder's config.
    pub fn config(&self) -> &NodeConfig {
        &self.config
    }

}

impl NodeBuilder<(), (), (), InitState> {
    /// Create a new [`NodeBuilder`].
    pub fn new(config: NodeConfig) -> Self {
        Self { config, state: InitState::default(), ctx: Default::default() }
    }
}

impl<DB, Pool, Ext> NodeBuilder<DB, Pool, Ext, InitState> {
    /// Configures the additional external context, e.g. additional context captured via CLI args.
    pub fn with_ext<E>(self, ext: E) -> NodeBuilder<DB, Pool, E, InitState> {
        NodeBuilder { config: self.config, state: self.state, ctx: NodeBuilderContext { database: self.ctx.database,pool: self.ctx.pool, ext } }
    }

    /// Configures the additional external context, e.g. additional context captured via CLI args.
    pub fn with_database<D>(self, database: D) -> NodeBuilder<D, Pool, Ext, InitState> {
        NodeBuilder { config: self.config, state: self.state,  ctx: NodeBuilderContext { database, pool: self.ctx.pool, ext: self.ctx.ext } }
    }
}

impl<DB, Pool, Ext> NodeBuilder<DB, Pool, Ext, InitState>
where
    DB: Database + DatabaseMetrics + DatabaseMetadata + Clone + 'static,
    Ext: RethNodeCommandExt,
    Pool: PoolBuilder,
{
    /// Launches the node.
    pub async fn launch(mut self, executor: TaskExecutor) -> eyre::Result<NodeHandle> {

        todo!()
    }
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct InitState;


/// Captures configurable context for the node builder process.
///
/// This is responsible for creating the node's components.
struct NodeBuilderContext<DB, Pool, Ext> {
    /// Holds the database
    database: DB,
    /// Holds additional external context, e.g. additional context captured via CLI args.
    ext: Ext,
    /// The transaction pool
    pool: Pool
}

impl Default for NodeBuilderContext<(), (), ()> {
    fn default() -> Self {
        Self { database: (), ext: (), pool: () }
    }
}
