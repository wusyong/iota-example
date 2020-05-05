use anyhow::Result;

#[smol_potat::main]
async fn main() -> Result<()> {
    let iota = iota_client::Client::new("https://nodes.comnet.thetangle.org");
    let node_info = iota.get_node_info().await?;
    println!("{:#?}", node_info);
    Ok(())
}
