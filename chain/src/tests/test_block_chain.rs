use super::random_block;
use crate::{AsyncChain, BlockChain, ChainActor, ChainActorRef, ChainAsyncService};
use actix::Addr;
use anyhow::Result;
use bus::BusActor;
use config::NodeConfig;
use consensus::{dummy::DummyConsensus, Consensus};
use crypto::{hash::CryptoHash, HashValue};
use executor::{mock_executor::MockExecutor, TransactionExecutor};
use logger::prelude::*;
use std::sync::Arc;
use storage::{memory_storage::MemoryStorage, StarcoinStorage};
use traits::ChainReader;
use traits::ChainWriter;
use txpool::{CachedSeqNumberClient, SubscribeTxns, TxPool, TxPoolRef};
use types::block::Block;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

fn gen_block_chain_actor(conf: Arc<NodeConfig>) -> ChainActorRef<ChainActor> {
    let repo = Arc::new(MemoryStorage::new());
    let storage = Arc::new(StarcoinStorage::new(repo).unwrap());
    let bus = BusActor::launch();
    let seq_number_client = CachedSeqNumberClient::new(storage.clone());
    let txpool = TxPool::start(seq_number_client);
    ChainActor::launch(conf, storage.clone(), None, bus, txpool).unwrap()
}

async fn gen_head_chain(times: u64) -> ChainActorRef<ChainActor> {
    let node_config = NodeConfig::default();
    let (state_root, chain_state_set) = MockExecutor::init_genesis(&node_config.vm).unwrap();
    let conf = Arc::new(node_config);
    let chain = gen_block_chain_actor(conf);

    let genesis_block = Block::genesis_block(HashValue::zero(), state_root, chain_state_set);

    let times = 5;
    let mut parent_hash = genesis_block.header().id();
    if times > 0 {
        for i in 0..times {
            println!("{}", i);
            let block = random_block(Some((parent_hash, i)));
            parent_hash = block.header().id();
            chain.clone().try_connect(block).await.unwrap();
        }
    }

    chain
}

#[actix_rt::test]
async fn test_block_chain_head() {
    let times = 5;
    let chain = gen_head_chain(times).await;
    assert_eq!(chain.current_header().await.unwrap().number(), times)
}

#[actix_rt::test]
async fn test_block_chain_forks() {
    let times = 5;
    let chain = gen_head_chain(times).await;
    let node_config = NodeConfig::default();
    let (state_root, chain_state_set) = MockExecutor::init_genesis(&node_config.vm).unwrap();
    let genesis_block = Block::genesis_block(HashValue::zero(), state_root, chain_state_set);
    let mut parent_hash = genesis_block.header().id();
    if times > 0 {
        for i in 0..(times + 1) {
            println!("{}", i);
            let block = random_block(Some((parent_hash, i)));
            parent_hash = block.header().id();
            chain.clone().try_connect(block).await.unwrap();
        }
    }

    assert_eq!(chain.current_header().await.unwrap().number(), (times + 1))
}

#[actix_rt::test]
async fn test_block_chain_rollback() {
    //todo
}

#[stest::test]
fn test_chain_apply() -> Result<()> {
    let node_config = NodeConfig::default();
    let config = Arc::new(node_config);
    let repo = Arc::new(MemoryStorage::new());
    let storage = Arc::new(StarcoinStorage::new(repo)?);
    let seq_number_client = CachedSeqNumberClient::new(storage.clone());
    let txpool = TxPool::start(seq_number_client);

    let mut block_chain =
        BlockChain::<MockExecutor, DummyConsensus, StarcoinStorage, TxPoolRef>::new(
            config, storage, None, txpool,
        )?;
    let header = block_chain.current_header();
    debug!("genesis header: {:?}", header);
    let block_template = block_chain.create_block_template(vec![])?;
    let (sender, receiver) = futures::channel::oneshot::channel();
    let new_block = DummyConsensus::create_block(&block_chain, block_template, receiver)?;
    block_chain.apply(new_block);
    let header1 = block_chain.current_header();
    debug!("block 1 header: {:?}", header1);
    assert_ne!(header.state_root(), header1.state_root());
    Ok(())
}
