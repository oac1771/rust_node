#[cfg(test)]
mod tests {

    use crate::clients::ipfs::client::IpfsClient;
    use crate::clients::reqwest::client::MockReqwestClient;
    use crate::clients::reqwest::models::Response;

    fn create_mocked_response(status_code: String, body: String) -> Response{
        return Response {
            status_code,
            body
        }
    }

    #[tokio::test]
    async fn test_get_id_should_return_id() {
        let body = "{\"ID\":\"12D3KooWPoZPm5khvdtczdCCJYdo3YfPSL43APWL1vQdzMZjM2wn\",\"PublicKey\":\"CAESIM/Oukd0WYozap7KMmfW1QV65PbvRq0aRhz+xdbKz47N\",\"Addresses\":[\"/ip4/127.0.0.1/tcp/4001/p2p/12D3KooWPoZPm5khvdtczdCCJYdo3YfPSL43APWL1vQdzMZjM2wn\",\"/ip4/127.0.0.1/udp/4001/quic/p2p/12D3KooWPoZPm5khvdtczdCCJYdo3YfPSL43APWL1vQdzMZjM2wn\",\"/ip4/192.168.86.51/tcp/4001/p2p/12D3KooWPoZPm5khvdtczdCCJYdo3YfPSL43APWL1vQdzMZjM2wn\",\"/ip4/192.168.86.51/udp/4001/quic/p2p/12D3KooWPoZPm5khvdtczdCCJYdo3YfPSL43APWL1vQdzMZjM2wn\",\"/ip6/::1/tcp/4001/p2p/12D3KooWPoZPm5khvdtczdCCJYdo3YfPSL43APWL1vQdzMZjM2wn\",\"/ip6/::1/udp/4001/quic/p2p/12D3KooWPoZPm5khvdtczdCCJYdo3YfPSL43APWL1vQdzMZjM2wn\"],\"AgentVersion\":\"kubo/0.16.0/38117db6f\",\"ProtocolVersion\":\"ipfs/0.1.0\",\"Protocols\":[\"/ipfs/bitswap\",\"/ipfs/bitswap/1.0.0\",\"/ipfs/bitswap/1.1.0\",\"/ipfs/bitswap/1.2.0\",\"/ipfs/id/1.0.0\",\"/ipfs/id/push/1.0.0\",\"/ipfs/lan/kad/1.0.0\",\"/ipfs/ping/1.0.0\",\"/libp2p/autonat/1.0.0\",\"/libp2p/circuit/relay/0.1.0\",\"/libp2p/circuit/relay/0.2.0/stop\",\"/p2p/id/delta/1.0.0\",\"/x/\"]}\n".to_string();
        let response = create_mocked_response("".to_string(), body);

        let mut mock_reqwest_client = MockReqwestClient::new();
        mock_reqwest_client.expect_post().return_once(|_| response);

        let ipfs_client = IpfsClient {
            reqwest_client: mock_reqwest_client
        };

        let ipfs_id_response = ipfs_client.get_id().await;
        assert_eq!(ipfs_id_response.ID, "12D3KooWPoZPm5khvdtczdCCJYdo3YfPSL43APWL1vQdzMZjM2wn")

    }

    
}