use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum ChooseDomain {
    Artgrid,
    Artlist,
    Envato,
    BadDomain
}

impl ChooseDomain {
    pub fn get_domain(&self) -> &str {
        match &self {
            ChooseDomain::Artgrid => "artgrid_cookies.json",
            ChooseDomain::Artlist => "artlist_cookies.json",
            ChooseDomain::Envato => "envato_cookies.json",
            ChooseDomain::BadDomain => "bad",

        }
    }

    pub fn get_origin_domain(&self) -> &str {
        match &self {
            ChooseDomain::Artgrid => "https://artgrid.io/",
            ChooseDomain::Artlist => "https://artlist.io",
            ChooseDomain::Envato => "https://elements.envato.com",
            ChooseDomain::BadDomain => "bad",
        }
    }

    pub fn compare_domain(&self, user_domain : ChooseDomain) -> bool
    {
        *self == user_domain
    }
}

impl From<Option<&str>> for ChooseDomain {
    fn from(value: Option<&str>) -> Self {
        if let Some(domain) = value {
            match domain {
                "artgrid.io" => ChooseDomain::Artgrid,
                "artlist.io" => ChooseDomain::Artlist,
                "elements.envato.com" => ChooseDomain::Envato,
                _ => ChooseDomain::BadDomain
            }
        }else { ChooseDomain::BadDomain }
    }
}

impl Display for ChooseDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ChooseDomain::Artgrid => String::from("artgrid.io"),
            ChooseDomain::Artlist => String::from("artlist.io"),
            ChooseDomain::Envato => String::from("elements.envato.com"),
            ChooseDomain::BadDomain => String::from("bad domain"),
        };
        write!(f, "{}", str)
    }
}