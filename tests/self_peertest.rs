#[cfg(test)]
mod test {
    use ethportal_peertest as peertest;
    use std::{thread, time};
    use trin_core::cli::TrinConfig;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_launches() {
        tracing_subscriber::fmt::init();

        // Run a client, as a buddy peer for ping tests, etc.
        let (bootnode, peertest_nodes) = peertest::launch_peertest_nodes(2).await;
        // Short sleep to make sure all peertest nodes can connect
        thread::sleep(time::Duration::from_secs(1));

        let peertest_config = peertest::PeertestConfig::default();

        // Run a client, to be tested
        let trin_config = TrinConfig::new_from(
            [
                "trin",
                "--internal-ip",
                "--web3-ipc-path",
                &peertest_config.target_ipc_path,
            ]
            .iter(),
        )
        .unwrap();
        let test_client_exiter = trin::run_trin(trin_config, String::new()).await.unwrap();

        peertest::jsonrpc::test_jsonrpc_endpoints_over_ipc(
            peertest_config,
            bootnode.enr.to_base64(),
        )
        .await;

        bootnode.exiter.exit();
        peertest_nodes.iter().for_each(|node| node.exiter.exit());
        test_client_exiter.exit();
    }
}
