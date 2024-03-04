use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub years: Years,
    pub leet: Leet,
    pub specialchars: SpecialChars,
    pub nums: Nums,
    pub wls: Wls,
    pub threshold: Threshold,
    pub wordlist: Wordlist,
}

#[derive(Deserialize, Debug)]
pub struct Years {
    pub years: Vec<u32>,
}

#[derive(Deserialize, Debug)]
pub struct Leet {
    pub a: u32,
    pub i: u32,
    pub e: u32,
    pub t: u32,
    pub o: u32,
    pub s: u32,
    pub g: u32,
    pub z: u32,
}

#[derive(Deserialize, Debug)]
pub struct SpecialChars {
    pub chars: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Nums {
    pub from: u32,
    pub to: u32,
}

#[derive(Deserialize, Debug)]
pub struct Wls {
    pub wcfrom: u32,
    pub wcto: u32,
}

#[derive(Deserialize, Debug)]
pub struct Threshold {
    pub threshold: u32,
}


#[derive(Deserialize, Debug)]
pub struct Wordlist {
    pub alectourl: String,
    pub dicturl: String,
}

pub fn load_config(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config_content = std::fs::read_to_string(file_path)?;
    let config: Config = toml::from_str(&config_content)?;
    println!("Leet mapping for 'a': {}", config.leet.a);
    println!("Special chars: {:?}", config.specialchars.chars);
    println!("Num range: {} to {}", config.nums.from, config.nums.to);
    println!("Word length from: {}, to: {}", config.wls.wcfrom, config.wls.wcto);
    println!("Threshold: {}", config.threshold.threshold);
    println!("Alecto URL: {}", config.wordlist.alectourl);
    println!("Downloader URL: {}", config.wordlist.dicturl);
    Ok(config)
}    

