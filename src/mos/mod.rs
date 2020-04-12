use chrono::prelude::*;
use chrono::{DateTime, Duration};
use regex::Regex;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

pub mod error;

#[derive(Debug, Deserialize, Serialize)]
pub struct MOSMeta {
    icao: String,
    timestamp: DateTime<Utc>,
}

impl Default for MOSMeta {
    fn default() -> Self {
        MOSMeta {
            icao: String::from(""),
            timestamp: Utc.ymd(1970, 1, 1).and_hms(0, 0, 0),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MOSEntry {
    timestamp: DateTime<Utc>,
    nx: Option<isize>,
    tmp: Option<isize>,
    dpt: Option<isize>,
    cld: Option<String>,
    wdr: Option<isize>,
    wsp: Option<isize>,
    p06: Option<isize>,
    p12: Option<isize>,
    q06: Option<isize>,
    q12: Option<isize>,
    t06: Option<(isize, isize)>,
    t12: Option<(isize, isize)>,
    poz: Option<isize>,
    pos: Option<isize>,
    typ: Option<String>,
    snw: Option<isize>,
    cig: Option<isize>,
    vis: Option<isize>,
    obv: Option<String>,
}

impl Default for MOSEntry {
    fn default() -> Self {
        MOSEntry {
            timestamp: Utc.ymd(1970, 1, 1).and_hms(0, 0, 0),
            nx: None,
            tmp: None,
            dpt: None,
            cld: None,
            wdr: None,
            wsp: None,
            p06: None,
            p12: None,
            q06: None,
            q12: None,
            t06: None,
            t12: None,
            poz: None,
            pos: None,
            typ: None,
            snw: None,
            cig: None,
            vis: None,
            obv: None,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct MOS {
    pub meta: MOSMeta,
    pub entries: Vec<MOSEntry>,
    pub raw: String,
}

impl MOS {
    pub fn new(raw_mos: &str) -> Result<MOS, error::TaggedError> {
        let lines: Vec<&str> = raw_mos.split("\n").collect();
        let mut mos = MOS::default();
        mos.raw = raw_mos.to_string();

        // Metadata
        let meta_line = match lines.iter().next() {
            Some(line) => line,
            None => return Err(error::new("mos string is empty")),
        };
        mos.meta = MOS::parse_meta(meta_line)?;

        // Get the start and end indices of the data in the text
        let chunks = match lines
            .iter()
            .filter(|line| {
                let prefix_re = match Regex::new(r"^ *([^ ]+) +.*$") {
                    Ok(re) => re,
                    Err(_) => return false,
                };
                let prefix_captures = match prefix_re.captures_iter(line).next() {
                    Some(prefix) => prefix,
                    None => return false,
                };
                let prefix = prefix_captures[1].to_string();

                prefix.as_str() == "HR"
            })
            .next()
            .and_then(|line| {
                let data_re = match Regex::new(r"( *[0-9][0-9])") {
                    Ok(re) => re,
                    Err(_) => return None,
                };
                Some(
                    data_re
                        .find_iter(line)
                        .map(|time| (time.start(), time.end()))
                        .collect::<Vec<(usize, usize)>>(),
                )
            }) {
            Some(chunks) => chunks,
            None => return Err(error::new("could not parse hour line")),
        };

        // Build out the entries
        mos.entries = chunks
            .iter()
            .enumerate()
            .map(|(i, chunk)| {
                let mut entry = MOSEntry::default();
                lines.iter().for_each(|line| {
                    let prefix_re = match Regex::new(r"^ *([^ ]+)") {
                        Ok(re) => re,
                        Err(_) => return,
                    };
                    let prefix = match prefix_re.find(line) {
                        Some(prefix) => prefix,
                        None => return,
                    };
                    let prefix_str = line[prefix.start()..prefix.end()].trim();

                    let data: &str;
                    if i == 0 {
                        data = &line[prefix.end()..chunk.1].trim();
                    } else {
                        data = &line[chunk.0..chunk.1].trim();
                    }

                    match prefix_str {
                        "N/X" | "X/N" => {
                            let num = match data.parse::<isize>() {
                                Ok(num) => Some(num),
                                Err(_) => None,
                            };
                            entry.nx = num;
                        }
                        "TMP" => {
                            let num = match data.parse::<isize>() {
                                Ok(num) => Some(num),
                                Err(_) => None,
                            };
                            entry.tmp = num;
                        }
                        "DPT" => {
                            let num = match data.parse::<isize>() {
                                Ok(num) => Some(num),
                                Err(_) => None,
                            };
                            entry.dpt = num;
                        }
                        "CLD" => {
                            entry.cld = Some(data.to_string());
                        }
                        "WDR" => {
                            let num = match data.parse::<isize>() {
                                Ok(num) => Some(num),
                                Err(_) => None,
                            };
                            entry.wdr = num;
                        }
                        "WSP" => {
                            let num = match data.parse::<isize>() {
                                Ok(num) => Some(num),
                                Err(_) => None,
                            };
                            entry.wsp = num;
                        }
                        "P06" => {
                            let num = match data.parse::<isize>() {
                                Ok(num) => Some(num),
                                Err(_) => None,
                            };
                            entry.p06 = num;
                        }
                        "P12" => {
                            let num = match data.parse::<isize>() {
                                Ok(num) => Some(num),
                                Err(_) => None,
                            };
                            entry.p12 = num;
                        }
                        "Q06" => {
                            let num = match data.parse::<isize>() {
                                Ok(num) => Some(num),
                                Err(_) => None,
                            };
                            entry.q06 = num;
                        }
                        "Q12" => {
                            let num = match data.parse::<isize>() {
                                Ok(num) => Some(num),
                                Err(_) => None,
                            };
                            entry.q12 = num;
                        }
                        "POZ" => {
                            let num = match data.parse::<isize>() {
                                Ok(num) => Some(num),
                                Err(_) => None,
                            };
                            entry.poz = num;
                        }
                        "POS" => {
                            let num = match data.parse::<isize>() {
                                Ok(num) => Some(num),
                                Err(_) => None,
                            };
                            entry.pos = num;
                        }
                        "TYP" => {
                            entry.typ = Some(data.to_string());
                        }
                        "SNW" => {
                            let num = match data.parse::<isize>() {
                                Ok(num) => Some(num),
                                Err(_) => None,
                            };
                            entry.snw = num;
                        }
                        "CIG" => {
                            let num = match data.parse::<isize>() {
                                Ok(num) => Some(num),
                                Err(_) => None,
                            };
                            entry.cig = num;
                        }
                        "VIS" => {
                            let num = match data.parse::<isize>() {
                                Ok(num) => Some(num),
                                Err(_) => None,
                            };
                            entry.vis = num;
                        }
                        "OBV" => {
                            entry.obv = Some(data.to_string());
                        }
                        _ => return,
                    }
                });

                entry
            })
            .collect();

        // Add timestamps for all entries
        let num_entries = mos.entries.len();
        let base_ts = mos.meta.timestamp;
        mos.entries = mos
            .entries
            .into_iter()
            .enumerate()
            .map(|(i, mut entry)| {
                let mut add_hours: i64 = i as i64 * 3i64 + 6;
                if num_entries > 2 && i >= num_entries - 2 {
                    let mult = (3 - (num_entries - i)) as i64;
                    add_hours += 3 * mult;
                }
                entry.timestamp = base_ts + Duration::hours(add_hours);
                entry
            })
            .collect();

        Ok(mos)
    }

    fn parse_meta(meta_line: &str) -> Result<MOSMeta, error::TaggedError> {
        let mut all_meta = meta_line.split_whitespace();
        let icao = match all_meta.nth(0) {
            Some(icao) => icao,
            None => return Err(error::new("no icao in the first line of the mos")),
        };
        let date = match all_meta.nth(3) {
            Some(date) => date,
            None => return Err(error::new("no date in the first line of the mos")),
        };
        let time = match all_meta.nth(0) {
            Some(time) => time,
            None => return Err(error::new("no time in the first line of the mos")),
        };
        Ok(MOSMeta {
            icao: icao.to_string(),
            timestamp: Utc.datetime_from_str(&format!("{} {}", date, time), "%m/%d/%Y %H%M")?,
        })
    }
}

pub fn get(icao: &str) -> Result<MOS, error::TaggedError> {
    let body = reqwest::get(&format!(
        "https://www.nws.noaa.gov/cgi-bin/mos/getmav.pl?sta={}",
        icao.to_string().to_uppercase()
    ))?
    .text()?;
    let raw_mos = extract_pre(&body)?;

    MOS::new(&raw_mos)
}

fn extract_pre(html: &str) -> Result<String, error::TaggedError> {
    let doc = Html::parse_document(html);
    let pre_selector = Selector::parse("pre")?;

    let pre_elem = match doc.select(&pre_selector).next() {
        Some(pre) => pre,
        None => return Err(error::new("did not find a pre block containing the data")),
    };
    let data_vec = pre_elem.text().collect::<Vec<_>>();
    Ok(data_vec[0].to_string())
}
