pub fn process<'a>(
    value: ftd::ast::VariableValue,
    kind: ftd::interpreter2::Kind,
    doc: &ftd::interpreter2::TDoc<'a>,
    config: &fastn_core::Config,
) -> ftd::interpreter2::Result<ftd::interpreter2::Value> {
    if let Some(ref sitemap) = config.package.sitemap {
        let doc_id = config
            .current_document
            .clone()
            .map(|v| fastn_core::utils::id_to_path(v.as_str()))
            .unwrap_or_else(|| {
                doc.name
                    .to_string()
                    .replace(config.package.name.as_str(), "")
            })
            .trim()
            .replace(std::path::MAIN_SEPARATOR, "/");

        if let Some(sitemap) = sitemap.get_sitemap_by_id(doc_id.as_str()) {
            return doc.from_json(&sitemap, &kind, value.line_number());
        }
    }
    doc.from_json(
        &fastn_core::sitemap::SiteMapCompat::default(),
        &kind,
        value.line_number(),
    )
}

pub fn full_sitemap_process<'a>(
    value: ftd::ast::VariableValue,
    kind: ftd::interpreter2::Kind,
    doc: &ftd::interpreter2::TDoc<'a>,
    config: &fastn_core::Config,
) -> ftd::interpreter2::Result<ftd::interpreter2::Value> {
    if let Some(ref sitemap) = config.package.sitemap {
        let doc_id = config
            .current_document
            .clone()
            .map(|v| fastn_core::utils::id_to_path(v.as_str()))
            .unwrap_or_else(|| {
                doc.name
                    .to_string()
                    .replace(config.package.name.as_str(), "")
            })
            .trim()
            .replace(std::path::MAIN_SEPARATOR, "/");

        let sitemap_compat = to_sitemap_compat(sitemap, doc_id.as_str());
        return doc.from_json(&sitemap_compat, &kind, value.line_number());
    }
    doc.from_json(
        &fastn_core::sitemap::SiteMapCompat::default(),
        &kind,
        value.line_number(),
    )
}

#[derive(Default, Debug, serde::Serialize)]
pub struct TocItemCompat {
    pub id: String,
    pub title: Option<String>,
    pub bury: bool,
    #[serde(rename = "extra-data")]
    pub extra_data: Vec<fastn_core::library2022::KeyValueData>,
    #[serde(rename = "is-active")]
    pub is_active: bool,
    #[serde(rename = "nav-title")]
    pub nav_title: Option<String>,
    pub children: Vec<fastn_core::sitemap::toc::TocItemCompat>,
    pub skip: bool,
    pub readers: Vec<String>,
    pub writers: Vec<String>,
}

#[derive(Default, Debug, serde::Serialize)]
pub struct SubSectionCompat {
    pub id: Option<String>,
    pub title: Option<String>,
    pub bury: bool,
    pub visible: bool,
    #[serde(rename = "extra-data")]
    pub extra_data: Vec<fastn_core::library2022::KeyValueData>,
    #[serde(rename = "is-active")]
    pub is_active: bool,
    #[serde(rename = "nav-title")]
    pub nav_title: Option<String>,
    pub toc: Vec<TocItemCompat>,
    pub skip: bool,
    pub readers: Vec<String>,
    pub writers: Vec<String>,
}

#[derive(Default, Debug, serde::Serialize)]
pub struct SectionCompat {
    id: String,
    title: Option<String>,
    bury: bool,
    #[serde(rename = "extra-data")]
    extra_data: Vec<fastn_core::library2022::KeyValueData>,
    #[serde(rename = "is-active")]
    is_active: bool,
    #[serde(rename = "nav-title")]
    nav_title: Option<String>,
    subsections: Vec<SubSectionCompat>,
    readers: Vec<String>,
    writers: Vec<String>,
}

#[derive(Default, Debug, serde::Serialize)]
pub struct SiteMapCompat {
    sections: Vec<SectionCompat>,
    readers: Vec<String>,
    writers: Vec<String>,
}

pub fn to_sitemap_compat(
    sitemap: &fastn_core::sitemap::Sitemap,
    current_document: &str,
) -> fastn_core::sitemap::SiteMapCompat {
    use itertools::Itertools;
    fn to_toc_compat(
        toc_item: &fastn_core::sitemap::toc::TocItem,
        current_document: &str,
    ) -> fastn_core::sitemap::toc::TocItemCompat {

        let mut is_child_active: bool = false;
        let mut children: Vec<fastn_core::sitemap::toc::TocItemCompat> = vec![];
        for child in toc_item.children.iter().filter(|t| !t.skip) {
            let child_to_toc_compat = to_toc_compat(child, current_document);
            if child_to_toc_compat.is_active {
                is_child_active = true;
            }
            children.push(child_to_toc_compat);
        }

        let toc_compat = fastn_core::sitemap::toc::TocItemCompat {
            url: Some(toc_item.id.clone()),
            number: None,
            title: toc_item.title.clone(),
            description: None,
            path: None,
            is_heading: false,
            font_icon: toc_item.icon.clone().map(|v| v.into()),
            bury: toc_item.bury,
            extra_data: toc_item.extra_data.to_owned(),
            is_active: fastn_core::utils::ids_matches(toc_item.id.as_str(), current_document) || is_child_active,
            is_open: false,
            nav_title: toc_item.nav_title.clone(),
            children,
            readers: toc_item.readers.clone(),
            writers: toc_item.writers.clone(),
            is_disabled: false,
            image_src: None,
            document: None,
        };
        toc_compat
    }

    fn to_subsection_compat(
        subsection: &fastn_core::sitemap::section::Subsection,
        current_document: &str,
    ) -> fastn_core::sitemap::toc::TocItemCompat {

        let mut is_child_active: bool = false;
        let mut children: Vec<fastn_core::sitemap::toc::TocItemCompat> = vec![];
        for child in subsection.toc.iter() {
            let child_to_toc_compat = to_toc_compat(child, current_document);
            if child_to_toc_compat.is_active {
                is_child_active = true;
            }
            children.push(child_to_toc_compat);
        }

        fastn_core::sitemap::toc::TocItemCompat {
            url: subsection.id.clone(),
            title: subsection.title.clone(),
            description: None,
            path: None,
            is_heading: false,
            font_icon: subsection.icon.clone().map(|v| v.into()),
            bury: subsection.bury,
            extra_data: subsection.extra_data.to_owned(),
            is_active: if let Some(ref subsection_id) = subsection.id {
                fastn_core::utils::ids_matches(subsection_id.as_str(), current_document) || is_child_active
            } else {
                is_child_active
            },
            is_open: false,
            image_src: None,
            nav_title: subsection.nav_title.clone(),
            children,
            readers: subsection.readers.clone(),
            writers: subsection.writers.clone(),
            number: None,
            is_disabled: false,
            document: None,
        }
    }

    fn to_section_compat(
        section: &fastn_core::sitemap::section::Section,
        current_document: &str,
    ) -> fastn_core::sitemap::toc::TocItemCompat {

        let mut is_child_active: bool = false;
        let mut children: Vec<fastn_core::sitemap::toc::TocItemCompat> = vec![];
        for child in section.subsections.iter().filter(|t| !t.skip && t.visible) {
            let child_to_subsection_compat = to_subsection_compat(child, current_document);
            if child_to_subsection_compat.is_active {
                is_child_active = true;
            }
            children.push(child_to_subsection_compat);
        }

        fastn_core::sitemap::toc::TocItemCompat {
            url: Some(section.id.to_string()),
            number: None,
            description: None,
            title: section.title.clone(),
            path: None,
            is_heading: false,
            font_icon: section.icon.clone().map(|v| v.into()),
            bury: section.bury,
            extra_data: section.extra_data.to_owned(),
            is_active: { fastn_core::utils::ids_matches(section.id.as_str(), current_document) || is_child_active },
            is_open: false,
            nav_title: section.nav_title.clone(),
            children,
            readers: section.readers.clone(),
            writers: section.writers.clone(),
            is_disabled: false,
            image_src: None,
            document: None,
        }
    }

    fastn_core::sitemap::SiteMapCompat {
        sections: sitemap
            .sections
            .iter()
            .filter(|s| !s.skip)
            .map(|s| to_section_compat(s, current_document))
            .collect_vec(),
        subsections: vec![],
        toc: vec![],
        current_section: None,
        current_subsection: None,
        current_page: None,
        readers: sitemap.readers.clone(),
        writers: sitemap.writers.clone(),
    }
}
