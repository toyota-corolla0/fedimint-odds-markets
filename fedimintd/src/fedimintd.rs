use fedimintd::fedimintd::Fedimintd;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Fedimintd::new()?
        .with_default_modules()
        .with_module(fedimint_dummy_server::OddsMarketsGen)
        .with_extra_module_inits_params(
            3,
            fedimint_dummy_server::KIND,
            fedimint_dummy_server::OddsMarketsGenParams::default(),
        )
        .run()
        .await
}
