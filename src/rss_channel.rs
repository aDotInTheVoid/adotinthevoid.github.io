use crate::config::Config;

pub(crate) fn channel(config: &Config, items: Vec<rss::Item>) -> rss::Channel {
    // https://validator.w3.org/feed/docs/warning/MissingAtomSelfLink.html
    let self_link = rss::extension::atom::Link {
        rel: "self".to_string(),
        href: format!("https://{}{}index.xml", config.base_domain, config.base_url),
        mime_type: Some("application/rss+xml".to_owned()),
        ..Default::default()
    };

    let atom_ext = rss::extension::atom::AtomExtensionBuilder::default()
        .link(self_link)
        .build();

    rss::ChannelBuilder::default()
        .title("Alona Enraght-Moony")
        .link(format!("https://{}{}", config.base_domain, config.base_url))
        .items(items)
        .atom_ext(Some(atom_ext))
        .build()
}
