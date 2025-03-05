//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::sync::Arc;

use jsonrpsee::RpcModule;
use sc_rpc::SubscriptionTaskExecutor;
use sc_transaction_pool_api::TransactionPool;
use solochain_template_runtime::{ AccountId, Balance, Nonce};
use sp_api::{CallApiAt, ProvideRuntimeApi};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sp_inherents::CreateInherentDataProviders;
use sp_runtime::traits::Block as BlockT;
use sc_transaction_pool::ChainApi;
use sc_client_api::{
	backend::{Backend, StorageProvider},
	client::BlockchainEvents,
	AuxStore, UsageProvider,
};

mod eth;
pub use self::eth::{create_eth, EthDeps};

/// Full client dependencies.
pub struct FullDeps<B: BlockT, C, P, A: ChainApi, CT, CIDP> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// Ethereum-compatibility specific dependencies.
	pub eth: EthDeps<B, C, P, A, CT, CIDP>,
}

pub struct DefaultEthConfig<C, BE>(std::marker::PhantomData<(C, BE)>);

impl<B, C, BE> fc_rpc::EthConfig<B, C> for DefaultEthConfig<C, BE>
where
	B: BlockT,
	C: StorageProvider<B, BE> + Sync + Send + 'static,
	BE: Backend<B> + 'static,
{
	type EstimateGasAdapter = ();
	type RuntimeStorageOverride =
		fc_rpc::frontier_backend_client::SystemAccountId20StorageOverride<B, C, BE>;
}

/// Instantiate all full RPC extensions.
pub fn create_full<B, C, P, BE, A, CT, CIDP>(
	deps: FullDeps<B, C, P, A, CT, CIDP>,
	subscription_task_executor: SubscriptionTaskExecutor,
	pubsub_notification_sinks: Arc<
		fc_mapping_sync::EthereumBlockNotificationSinks<
			fc_mapping_sync::EthereumBlockNotification<B>,
		>,
	>,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
	B: BlockT,
	C: CallApiAt<B> + ProvideRuntimeApi<B>,
	C::Api: sp_block_builder::BlockBuilder<B>,
	C::Api: sp_consensus_aura::AuraApi<B, AuraId>,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<B, AccountId, Nonce>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<B, Balance>,
	C::Api: fp_rpc::ConvertTransactionRuntimeApi<B>,
	C::Api: fp_rpc::EthereumRuntimeRPCApi<B>,
	C: HeaderBackend<B> + HeaderMetadata<B, Error = BlockChainError> + 'static,
	C: BlockchainEvents<B> + AuxStore + UsageProvider<B> + StorageProvider<B, BE>,
	BE: Backend<B> + 'static,
	P: TransactionPool<Block = B> + 'static,
	A: ChainApi<Block = B> + 'static,
	CIDP: CreateInherentDataProviders<B, ()> + Send + 'static,
	CT: fp_rpc::ConvertTransaction<<B as BlockT>::Extrinsic> + Send + Sync + 'static,
{
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
	use substrate_frame_rpc_system::{System, SystemApiServer};

	let mut module = RpcModule::new(());
	let FullDeps {
		client,
		pool,
		eth,
	} = deps;

	module.merge(System::new(client.clone(), pool).into_rpc())?;
	module.merge(TransactionPayment::new(client).into_rpc())?;

	// Extend this RPC with a custom API by using the following syntax.
	// `YourRpcStruct` should have a reference to a client, which is needed
	// to call into the runtime.
	// `module.merge(YourRpcTrait::into_rpc(YourRpcStruct::new(ReferenceToClient, ...)))?;`

	// You probably want to enable the `rpc v2 chainSpec` API as well
	//
	// let chain_name = chain_spec.name().to_string();
	// let genesis_hash = client.block_hash(0).ok().flatten().expect("Genesis block exists; qed");
	// let properties = chain_spec.properties();
	// module.merge(ChainSpec::new(chain_name, genesis_hash, properties).into_rpc())?;
	let module = create_eth::<_, _, _, _, _, _, _, DefaultEthConfig<C, BE>>(
		module,
		eth,
		subscription_task_executor,
		pubsub_notification_sinks,
	)?;

	Ok(module)
}
