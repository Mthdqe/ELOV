/**
 * \file entity.rs
 *
 * \brief  Data that are computed by the scraper
 *
 * \author Mathieu Dique
 */
/* -------------------------------------------------------------------------- */
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

/* -------------------------------------------------------------------------- */
/** \struct Competition
 *  \brief  Entity describing a scraped competition
 */
#[derive(Deserialize, Serialize, PartialEq, Getters)]
pub struct Competition {
    name: String, /*< The name of the competition */
    url: String,  /*< The url to follow the competition in the web page */
}

impl Competition {
    /**
     * \brief Constructor of the competition entity
     * \param name Name of the competition
     * \param url The url of the web page of this competition
     */
    pub fn new(name: &str, url: &str) -> Competition {
        Competition {
            name: String::from(name),
            url: String::from(url),
        }
    }
}
/* -------------------------------------------------------------------------- */
/** \struct Department
 *  \brief  Entity describing a scraped department
 */
#[derive(Deserialize, Serialize, PartialEq, Getters)]
pub struct Department {
    name: String, /*< The name of the department */
    url: String,  /*< Url of the pools of the department */
}

impl Department {
    /**
     * \brief Constructor of the department entity
     * \param name Name of the department
     * \param url The matching url of the department pools
     */
    pub fn new(name: &str, url: &str) -> Department {
        Department {
            name: String::from(name),
            url: String::from(url),
        }
    }
}

/* -------------------------------------------------------------------------- */
/** \struct region
 *  \brief  entity describing a scraped region
 */
#[derive(Deserialize, Serialize, PartialEq, Getters)]
pub struct Region {
    name: String, /*< The name of the region */
    url: String,  /*< Url of the pools */
}

impl Region {
    /**
     * \brief Constructor of the Region entity
     * \param name Name of the Region
     * \param url The url of the pools of this region
     */
    pub fn new(name: &str, url: &str) -> Region {
        Region {
            name: String::from(name),
            url: String::from(url),
        }
    }
}
