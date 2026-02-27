use eyre::Result;
use futures_util::TryStreamExt;
use rtnetlink::{
    Handle,
    packet_route::{
        link::LinkMessage,
        tc::{TcAttribute, TcMessage, TcStats2},
    },
};

#[derive(Debug)]
pub struct QdiscInterfaceStatistics {
    pub index: usize,
    /// number of seen bytes
    pub bytes: Option<u64>,
    /// number of seen packets
    pub packets: Option<u32>,
    /// queue length
    pub qlen: Option<u32>,
    /// backlog size of queue
    pub backlog: Option<u32>,
    /// number of dropped packets
    pub drops: Option<u32>,
    /// number of requeues
    pub requeues: Option<u32>,
    /// number of enqueues over the limit
    pub overlimits: Option<u32>,
}

const TC_H_ROOT: u32 = 0xFFFF_FFFF;

pub struct QdiscStatistics {
    handle: Handle,
}

impl QdiscStatistics {
    pub async fn new() -> Self {
        let (connection, handle, _messages) = rtnetlink::new_connection().unwrap();
        tokio::spawn(connection);

        Self { handle }
    }

    fn process_qdisc_message(&self, message: TcMessage) -> Option<QdiscInterfaceStatistics> {
        let (header, attributes) = message.into_parts();

        // node_exporter typically reports *root* qdiscs; keep this if you want the same effect.
        if header.parent != TC_H_ROOT.into() {
            return None;
        }

        let mut basic = None;
        let mut queue = None;

        for attribute in attributes {
            if let TcAttribute::Stats2(stats) = attribute {
                for stat in stats {
                    match stat {
                        TcStats2::Basic(raw) => {
                            basic = Some(raw);
                        }
                        TcStats2::Queue(raw) => {
                            queue = Some(raw);
                        }
                        _ => {}
                    }
                }
            }
        }

        Some(QdiscInterfaceStatistics {
            index: header.index as usize,
            bytes: basic.and_then(|basic| Some(basic.bytes)),
            packets: basic.and_then(|basic| Some(basic.packets)),
            qlen: queue.and_then(|queue| Some(queue.qlen)),
            backlog: queue.and_then(|queue| Some(queue.backlog)),
            drops: queue.and_then(|queue| Some(queue.drops)),
            requeues: queue.and_then(|queue| Some(queue.requeues)),
            overlimits: queue.and_then(|queue| Some(queue.overlimits)),
        })
    }

    pub async fn poll(&self) -> Result<Vec<QdiscInterfaceStatistics>> {
        let qreq = self.handle.qdisc().get().index(0); // FIXME: index = 0 ok ?

        let stream = qreq.execute();

        let messages: Vec<TcMessage> = stream.try_collect().await?;

        Ok(messages
            .into_iter()
            .map(|message| self.process_qdisc_message(message))
            .flatten()
            .collect())
    }

    pub async fn get_interface_names(&self) -> Result<Vec<String>> {
        let links = self.handle.link().get().execute();

        let messages: Vec<LinkMessage> = links.try_collect().await?;

        let interfaces: Vec<(String, usize)> = messages
            .into_iter()
            .map(|message| {
                let index = message.header.index as usize;
                let name =
                    message
                        .attributes
                        .into_iter()
                        .find_map(|attribute| match attribute {
                            rtnetlink::packet_route::link::LinkAttribute::IfName(name) => {
                                Some(name)
                            }
                            _ => None,
                        })?;

                Some((name, index))
            })
            .flatten()
            .collect();

        let max_index = *interfaces
            .iter()
            .map(|(_, index)| index)
            .max()
            .unwrap_or_else(|| &0) as usize;

        let mut interface_names = vec!["?".into(); max_index + 1];
        for (name, index) in interfaces {
            interface_names[index] = name;
        }

        Ok(interface_names)
    }
}
