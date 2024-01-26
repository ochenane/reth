//! Wrapper for [`discv5::Discv5`] that supports downgrade to [`Discv4`].

use std::{
    fmt,
    net::IpAddr,
    pin::Pin,
    sync::Arc,
    task::{ready, Context, Poll},
};

use derive_more::From;
use futures::{
    stream::{select, Select},
    Stream, StreamExt,
};
use parking_lot::RwLock;
use reth_discv4::{DiscoveryUpdate, Discv4, HandleDiscovery, NodeRecord, PublicKey};
use reth_primitives::{bytes::Bytes, PeerId};
use tokio::sync::{mpsc, watch};
use tokio_stream::wrappers::ReceiverStream;

/// Reth Discv5 type, wraps [`discv5::Discv5`] supporting downgrade to [`Discv4`].
pub struct Discv5 {
    discv5: Arc<RwLock<discv5::Discv5>>,
    discv4: Discv4,
    discv5_kbuckets_change_tx: watch::Sender<()>,
}

impl Discv5 {
    /// Returns a new [`Discv5`] handle.
    pub fn new(
        discv5: Arc<RwLock<discv5::Discv5>>,
        discv4: Discv4,
        discv5_kbuckets_change_tx: watch::Sender<()>,
    ) -> Self {
        Self { discv5, discv4, discv5_kbuckets_change_tx }
    }

    /// Notifies [`Discv4`] that [discv5::Discv5]'s kbucktes have been updated. This brings
    /// [`Discv4`] to update its mirror of the [discv5::Discv5] kbucktes upon next
    /// [`reth_discv4::proto::Neighbours`] message.
    pub fn notify_discv4_of_kbuckets_update(&self) -> Result<(), watch::error::SendError<()>> {
        self.discv5_kbuckets_change_tx.send(())
    }
}

impl HandleDiscovery for Discv5 {
    fn add_node(&self, node_record: NodeRecord) {
        // todo: convert node record to an enr. this is an upstream change since it requires being
        // able to assemble the enr type without access to the secret key, as `EnrBuilder`
        // requires.

        self.discv4.add_node(node_record)
    }

    fn set_eip868_rlp_pair(&self, key: Vec<u8>, rlp: Bytes) {
        self.discv4.set_eip868_rlp_pair(key, rlp)
    }

    fn set_eip868_rlp(&self, key: Vec<u8>, value: impl alloy_rlp::Encodable) {
        self.discv4.set_eip868_rlp(key, value)
    }

    fn ban(&self, node_id: PeerId, _ip: IpAddr) {
        if let Ok(node_id_compressed) = uncompressed_to_compressed_id(node_id) {
            self.discv5.read().ban_node(&node_id_compressed, None);
        }
        self.discv4.ban(node_id, _ip);
    }

    fn ban_ip(&self, ip: IpAddr) {
        self.discv5.read().ban_ip(ip, None);
        self.discv4.ban_ip(ip);
    }
}

impl fmt::Debug for Discv5 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("Discv5");

        debug_struct.field("discv5", &"{ .. }");
        debug_struct.field("discv4", &self.discv4);
        debug_struct.field("discv5_kbuckets_change_tx", &self.discv5_kbuckets_change_tx);

        debug_struct.finish()
    }
}

/// Wrapper around update type used in [`discv5::Discv5`] and [`Discv4`].
#[derive(Debug, From)]
pub enum DiscoveryUpdateV5 {
    /// A [`discv5::Discv5`] update.
    V5(discv5::Discv5Event),
    /// A [`Discv4`] update.
    V4(DiscoveryUpdate),
}

/// Stream wrapper for streams producing types that can convert to [`DiscoveryUpdateV5`].
#[derive(Debug)]
pub struct UpdateStream<S>(S);

impl<S, I> Stream for UpdateStream<S>
where
    S: Stream<Item = I> + Unpin,
    DiscoveryUpdateV5: From<I>,
{
    type Item = DiscoveryUpdateV5;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(ready!(self.0.poll_next_unpin(cx)).map(DiscoveryUpdateV5::from))
    }
}

/// A stream that polls update streams from [`discv5::Discv5`] and [`Discv4`] in round-robin
/// fashion.
pub type MergedUpdateStream = Select<
    UpdateStream<ReceiverStream<discv5::Discv5Event>>,
    UpdateStream<ReceiverStream<DiscoveryUpdate>>,
>;

/// Returns a merged stream of [`discv5::Discv5Event`]s and [`DiscoveryUpdate`]s, that supports
/// downgrading to discv4.
pub fn merge_discovery_streams(
    discv5_event_stream: mpsc::Receiver<discv5::Discv5Event>,
    discv4_update_stream: ReceiverStream<DiscoveryUpdate>,
) -> Select<
    UpdateStream<ReceiverStream<discv5::Discv5Event>>,
    UpdateStream<ReceiverStream<DiscoveryUpdate>>,
> {
    let discv5_event_stream = UpdateStream(ReceiverStream::new(discv5_event_stream));
    let discv4_update_stream = UpdateStream(discv4_update_stream);

    select(discv5_event_stream, discv4_update_stream)
}

/// Converts a [`discv5::enr::NodeId`] to a [`PeerId`]. [`discv5::enr::NodeId`] is essentially a
/// compressed [`PeerId`].
///
/// Trait `discv5::enr::EnrPublicKey` is implemented for reth_discv4 re-exported key
/// type`secp256k1::PublicKey` from secp256k1-0.27.0.
pub fn compressed_to_uncompressed_id(
    node_id: discv5::enr::NodeId,
) -> Result<PeerId, secp256k1::Error> {
    let pk_compressed_bytes = node_id.raw();
    let pk = PublicKey::from_slice(&pk_compressed_bytes)?;

    Ok(PeerId::from_slice(&pk.serialize_uncompressed()[1..]))
}

/// Converts a [`PeerId`] to a [`discv5::enr::NodeId`]. [`PeerId`] is essentially an uncompressed
/// [`discv5::enr::NodeId`].
///
/// Trait `discv5::enr::EnrPublicKey` is implemented for reth_discv4 re-exported key
/// type`secp256k1::PublicKey` from secp256k1-0.27.0.
pub fn uncompressed_to_compressed_id(
    peer_id: PeerId,
) -> Result<discv5::enr::NodeId, secp256k1::Error> {
    let pk = PublicKey::from_slice(peer_id.as_ref())?;

    Ok(pk.into())
}
