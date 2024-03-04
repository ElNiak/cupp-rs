use clap::{Arg, Command, ArgGroup};
use reqwest;
use tokio;
use std::time::Duration;
use std::fs;
use std::fs::File;
use reqwest::Client;
use csv::ReaderBuilder;
use flate2::read::GzDecoder;
use std::{collections::HashSet, path::Path};
use std::io::{self, Write};
use std::collections::HashMap;
use std::io::BufRead;
use std::thread;
use std::time::Instant;

mod config;
use crate::config::Config;

struct Section {
    name: &'static str,
    files: Vec<&'static str>,
}

#[derive(Debug)]
struct Profile {
    name: String,
    surname: String,
    nickname: String,
    birthdate: String,
    partner_name: String,
    partner_nickname: String,
    partner_birthdate: String,
    child_name: String,
    child_nickname: String,
    child_birthdate: String,
    pet_name: String,
    company_name: String,
    keywords: Vec<String>,
    want_special_chars: bool,
    want_random_numbers: bool,
    leet_mode: bool,
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn interactive() -> Profile {
    println!("\n[+] Insert the information about the victim to make a dictionary");
    println!("[+] If you don't know all the info, just hit enter when asked! ;)\n");

    let mut profile = Profile {
        name: String::new(),
        surname: String::new(),
        nickname: String::new(),
        birthdate: String::new(),
        partner_name: String::new(),
        partner_nickname: String::new(),
        partner_birthdate: String::new(),
        child_name: String::new(),
        child_nickname: String::new(),
        child_birthdate: String::new(),
        pet_name: String::new(),
        company_name: String::new(),
        keywords: Vec::new(),
        want_special_chars: false,
        want_random_numbers: false,
        leet_mode: false,
    };

    profile.name = read_input("> First Name: ").to_lowercase();
    while profile.name.trim().is_empty() {
        println!("\n[-] You must enter a name at least!");
        profile.name = read_input("> Name: ").to_lowercase();
    }

    profile.surname = read_input("> Surname: ").to_lowercase();
    profile.nickname = read_input("> Nickname: ").to_lowercase();
    profile.birthdate = read_input("> Birthdate (DDMMYYYY): ").to_lowercase();
    while profile.birthdate.len() != 8 {
        println!("\n[-] Birthdate must be 8 characters long!");
        profile.birthdate = read_input("> Birthdate (DDMMYYYY): ").to_lowercase();
    }
    profile.partner_name = read_input("> Partner's name: ").to_lowercase();
    profile.partner_nickname = read_input("> Partner's nickname: ").to_lowercase();
    profile.partner_birthdate = read_input("> Partner's birthdate (DDMMYYYY): ").to_lowercase();
    while profile.partner_birthdate.len() != 8 {
        println!("\n[-] Birthdate must be 8 characters long!");
        profile.partner_birthdate = read_input("> Partner's birthdate (DDMMYYYY): ").to_lowercase();
    }
    profile.child_name = read_input("> Child's name: ").to_lowercase();
    profile.child_nickname = read_input("> Child's nickname: ").to_lowercase();
    profile.child_birthdate = read_input("> Child's birthdate (DDMMYYYY): ").to_lowercase();
    while profile.child_birthdate.len() != 8 {
        println!("\n[-] Birthdate must be 8 characters long!");
        profile.child_birthdate = read_input("> Child's birthdate (DDMMYYYY): ").to_lowercase();
    }
    profile.pet_name = read_input("> Pet's name: ").to_lowercase();
    profile.company_name = read_input("> Company name: ").to_lowercase();
    let keywords = read_input("> Do you want to add some key words about the victim? Y/[N]: ").to_lowercase();
    if keywords == "y" {
        let keywords = read_input("> Enter keywords separated by commas [i.e. hacker,juice,black]: ").to_lowercase();
        profile.keywords = keywords.split(',').map(|s| s.trim().to_string()).collect();
    }

    let want_special_chars = read_input("> Do you want to add special chars at the end of words? Y/[N]: ");
    profile.want_special_chars = want_special_chars.to_lowercase().trim() == "y";

    let want_random_numbers = read_input("> Do you want to add some random numbers at the end of words? Y/[N]: ");
    profile.want_random_numbers = want_random_numbers.to_lowercase().trim() == "y";

    let leet_mode = read_input("> Leet mode? (i.e. leet = 1337) Y/[N]: ");
    profile.leet_mode = leet_mode.to_lowercase().trim() == "y";

    profile
}

fn to_title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            word.char_indices()
                .map(|(i, c)| if i == 0 { c.to_uppercase().collect::<String>() } else { c.to_lowercase().collect() })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
}


fn generate_birthday_combinations(birthdate: &str) -> Vec<String> {
    let birthdate_yy = &birthdate[6..];
    let birthdate_yyy = &birthdate[5..];
    let birthdate_yyyy = &birthdate[4..];
    let birthdate_xd = &birthdate[1..2];
    let birthdate_xm = &birthdate[3..4];
    let birthdate_dd = &birthdate[..2];
    let birthdate_mm = &birthdate[2..4];

    let parts = vec![
        birthdate_yy, birthdate_yyy, birthdate_yyyy, 
        birthdate_xd, birthdate_xm, birthdate_dd, birthdate_mm,
    ];

    let mut combinations = Vec::new();

    for &part1 in &parts {
        combinations.push(part1.to_string()); // Single part
        for &part2 in &parts {
            if part1 != part2 {
                combinations.push(format!("{}{}", part1, part2)); // Two-part combinations
                for &part3 in &parts {
                    if part1 != part3 && part2 != part3 {
                        combinations.push(format!("{}{}{}", part1, part2, part3)); // Three-part combinations
                    }
                }
            }
        }
    }
    combinations.dedup(); // Remove duplicates
    combinations
}

fn generate_combinations(words: Vec<String>, suffixes: Vec<String>, separator: Option<&str>) -> Vec<String> {
    let mut combinations = Vec::new();
    for word in words.iter() {
        for suffix in suffixes.iter() {
            match separator {
                Some(sep) => combinations.push(format!("{}{}{}", word, sep, suffix)),
                None => combinations.push(format!("{}{}", word, suffix)),
            }
        }
    }
    combinations
}

fn get_leet_mappings() -> HashMap<char, &'static str> {
    let mut mappings = HashMap::new();
    mappings.insert('a', "4");
    mappings.insert('e', "3");
    mappings.insert('i', "1");
    mappings.insert('o', "0");
    mappings.insert('t', "7");
    mappings.insert('l', "1");
    // Add more mappings as needed
    mappings
}

fn apply_leet_transformations(wordlist: Vec<String>) -> Vec<String> {
    let leet_mappings = get_leet_mappings();
    let mut transformed_wordlist = Vec::new();

    for word in wordlist {
        let mut transformed_word = String::new();
        for c in word.chars() {
            if let Some(&leet_char) = leet_mappings.get(&c) {
                transformed_word.push_str(leet_char);
            } else {
                transformed_word.push(c);
            }
        }
        transformed_wordlist.push(transformed_word);
    }

    transformed_wordlist
}

fn generate_wordlist_from_profile(profile: &Profile, config: &Config) {
    // Start the timer
    let start = Instant::now();
    let mut wordlist = Vec::new();

    let chars = config.specialchars.chars.clone();
    let nums = (config.nums.from..config.nums.to).collect::<Vec<u32>>();
    let years = config.years.years.clone();

    // Basic information
    if !profile.name.is_empty() {
        wordlist.push(profile.name.clone());
    }
    if !profile.surname.is_empty() {
        wordlist.push(profile.surname.clone());
    }
    if !profile.nickname.is_empty() {
        wordlist.push(profile.nickname.clone());
    }

    // Partner and child details
    if !profile.partner_name.is_empty() {
        wordlist.push(profile.partner_name.clone());
    }
    if !profile.child_name.is_empty() {
        wordlist.push(profile.child_name.clone());
    }

    // Company and pet
    if !profile.company_name.is_empty() {
        wordlist.push(profile.company_name.clone());
    }
    if !profile.pet_name.is_empty() {
        wordlist.push(profile.pet_name.clone());
    }
    // Keywords
    for keyword in &profile.keywords {
        if !keyword.is_empty() {
            wordlist.push(keyword.clone());
        }
    }

    let birthdate_yy = &profile.birthdate[6..];
    let birthdate_yyy = &profile.birthdate[5..];
    let birthdate_yyyy = &profile.birthdate[4..];
    let birthdate_xd = &profile.birthdate[1..2];
    let birthdate_xm = &profile.birthdate[3..4];
    let birthdate_dd = &profile.birthdate[..2];
    let birthdate_mm = &profile.birthdate[2..4];

    let partner_birthdate_yy = &profile.partner_birthdate[6..];
    let partner_birthdate_yyy = &profile.partner_birthdate[5..];
    let partner_birthdate_yyyy = &profile.partner_birthdate[4..];
    let partner_birthdate_xd = &profile.partner_birthdate[1..2];
    let partner_birthdate_xm = &profile.partner_birthdate[3..4];
    let partner_birthdate_dd = &profile.partner_birthdate[..2];
    let partner_birthdate_mm = &profile.partner_birthdate[2..4];

    let child_birthdate_yy = &profile.child_birthdate[6..];
    let child_birthdate_yyy = &profile.child_birthdate[5..];
    let child_birthdate_yyyy = &profile.child_birthdate[4..];
    let child_birthdate_xd = &profile.child_birthdate[1..2];
    let child_birthdate_xm = &profile.child_birthdate[3..4];
    let child_birthdate_dd = &profile.child_birthdate[..2];
    let child_birthdate_mm = &profile.child_birthdate[2..4];

    let nameup = to_title_case(&profile.name);
    let surnameup = to_title_case(&profile.surname);
    let nickup = to_title_case(&profile.nickname);
    let wifeup = to_title_case(&profile.partner_name);
    let wifenup = to_title_case(&profile.partner_nickname);
    let kidup = to_title_case(&profile.child_name);
    let kidnup = to_title_case(&profile.child_nickname);
    let petup = to_title_case(&profile.pet_name);
    let companyup = to_title_case(&profile.company_name);

    let wordsup: Vec<String> = profile.keywords.iter().map(|word| to_title_case(word)).collect();
    let word: Vec<String> = [profile.keywords.clone(), wordsup].concat();

    let rev_name = profile.name.chars().rev().collect::<String>();
    let rev_nameup = nameup.chars().rev().collect::<String>();
    let rev_nick = profile.nickname.chars().rev().collect::<String>();
    let rev_nickup = nickup.chars().rev().collect::<String>();
    let rev_wife = profile.partner_name.chars().rev().collect::<String>();
    let rev_wifeup = wifeup.chars().rev().collect::<String>();
    let rev_kid = profile.child_name.chars().rev().collect::<String>();
    let rev_kidup = kidup.chars().rev().collect::<String>();

    let bd_combinations = generate_birthday_combinations(&profile.birthdate);
    wordlist.extend(bd_combinations.clone());

    let bd_combinations_partner = generate_birthday_combinations(&profile.partner_birthdate);
    wordlist.extend(bd_combinations_partner.clone());

    let bd_combinations_child = generate_birthday_combinations(&profile.child_birthdate);
    wordlist.extend(bd_combinations_child.clone());

    // Stop the timer
    let duration_bd = start.elapsed();
    // Print the elapsed time in seconds
    println!("Time elapsed for birthday combination is: {:?}", duration_bd);

    let kombinaac = vec![&profile.pet_name, &petup, &profile.company_name, &companyup];
    let kombina = vec![&profile.name, &profile.surname, &profile.nickname, &nameup, &surnameup, &nickup];
    let kombinaw = vec![&profile.partner_name, &profile.partner_nickname, &wifeup, &wifenup, &profile.surname, &surnameup];
    let kombinak = vec![&profile.child_name, &profile.child_nickname, &kidup, &kidnup, &profile.surname, &surnameup];

    let mut all_combinations = HashSet::new();

    for kombina1 in &kombina {
        for kombina2 in &kombina {
            if kombina1 != kombina2 && !all_combinations.contains(&(kombina1.to_lowercase() + kombina2)) {
                all_combinations.insert(kombina1.to_string() + kombina2);
            }
        }
    }

    // Logic for kombinaw (partner information)
    for kombina1 in &kombinaw {
        for kombina2 in &kombinaw {
            if kombina1 != kombina2 && !all_combinations.contains(&(kombina1.to_lowercase() + kombina2)) {
                all_combinations.insert(kombina1.to_string() + kombina2);
            }
        }
    }

    // Logic for kombinak (child information)
    for kombina1 in &kombinak {
        for kombina2 in &kombinak {
            if kombina1 != kombina2 && !all_combinations.contains(&(kombina1.to_lowercase() + kombina2)) {
                all_combinations.insert(kombina1.to_string() + kombina2);
            }
        }
    }

    
    // Placeholder for a HashMap to store all combinations
    let mut kombi: std::collections::HashMap<usize, Vec<String>> = std::collections::HashMap::new();

    let years_as_strings: Vec<String> = years.iter().map(|&year| year.to_string()).collect();
    let kombinaac_as_strings: Vec<String> = kombina.iter().map(|&year| year.to_string()).collect();
    let kombina_as_strings: Vec<String> = kombina.iter().map(|&year| year.to_string()).collect();
    let kombinaw_as_strings: Vec<String> = kombina.iter().map(|&year| year.to_string()).collect();
    let kombinak_as_strings: Vec<String> = kombina.iter().map(|&year| year.to_string()).collect();

    // Generating combinations
    kombi.insert(1, generate_combinations(kombina_as_strings.clone(), bd_combinations.clone(), None));
    kombi.insert(1, [kombi[&1].clone(), generate_combinations(kombina_as_strings.clone(), bd_combinations.clone(), Some("_"))].concat());

    // Generate combinations for partner information (kombinaw) with and without separator
    kombi.insert(2, generate_combinations(kombinaw_as_strings.clone(), bd_combinations_partner.clone(), None));
    kombi.entry(2).and_modify(|e| e.extend(generate_combinations(kombinaw_as_strings.clone(), bd_combinations_partner.clone(), Some("_"))));

    // Generate combinations for child information (kombinak) with and without separator
    kombi.insert(3, generate_combinations(kombinak_as_strings.clone(), bd_combinations_child.clone(), None));
    kombi.entry(3).and_modify(|e| e.extend(generate_combinations(kombinak_as_strings.clone(), bd_combinations_child.clone(), Some("_"))));

    // Generate combinations for personal information (kombina) with years
    kombi.insert(4, generate_combinations(kombina_as_strings.clone(), years_as_strings.clone(), None));
    kombi.entry(4).and_modify(|e| e.extend(generate_combinations(kombina_as_strings.clone(), years_as_strings.clone(), Some("_"))));

    // Generate combinations for pet and kombinaac_as_strings information (kombinaac) with years
    kombi.insert(5, generate_combinations(kombinaac_as_strings.clone(), years_as_strings.clone(), None));
    kombi.entry(5).and_modify(|e| e.extend(generate_combinations(kombinaac_as_strings.clone(), years_as_strings.clone(), Some("_"))));

    // Repeat the process for partner and child information with years
    kombi.insert(6, generate_combinations(kombinaw_as_strings.clone(), years_as_strings.clone(), None));
    kombi.entry(6).and_modify(|e| e.extend(generate_combinations(kombinaw_as_strings.clone(), years_as_strings.clone(), Some("_"))));

    kombi.insert(7, generate_combinations(kombinak_as_strings.clone(), years_as_strings.clone(), None));
    kombi.entry(7).and_modify(|e| e.extend(generate_combinations(kombinak_as_strings.clone(), years_as_strings.clone(), Some("_"))));

    // For word combinations with birthdate segments and years
    kombi.insert(8, generate_combinations(word.clone(), bd_combinations.clone(), None));
    kombi.entry(8).and_modify(|e| e.extend(generate_combinations(word.clone(), bd_combinations.clone(), Some("_"))));

    // Extend for words with partner's birthdate segments
    kombi.insert(9, generate_combinations(word.clone(), bd_combinations_partner.clone(), None));
    kombi.entry(9).and_modify(|e| e.extend(generate_combinations(word.clone(), bd_combinations_partner.clone(), Some("_"))));

    // Extend for words with child's birthdate segments
    kombi.insert(10, generate_combinations(word.clone(), bd_combinations_child.clone(), None));
    kombi.entry(10).and_modify(|e| e.extend(generate_combinations(word.clone(), bd_combinations_child.clone(), Some("_"))));

    // Extend for words with years
    kombi.insert(11, generate_combinations(word.clone(), years_as_strings.clone(), None));
    kombi.entry(11).and_modify(|e| e.extend(generate_combinations(word.clone(), years_as_strings.clone(), Some("_"))));

    // Stop the timer
    let duration_kom = start.elapsed();
    // Print the elapsed time in seconds
    println!("Time elapsed for birthday kombina is: {:?}", duration_kom);

    // Extend each word with special characters if the user wants them
    if profile.want_special_chars {
        let special_chars = config.specialchars.chars.clone();
        let mut extended_wordlist = Vec::new();
        for word in &wordlist {
            // Add the original word
            extended_wordlist.push(word.clone());

            // Extend the word with each special character
            for special_char in &special_chars {
                extended_wordlist.push(format!("{}{}", word, special_char));
            }
        }

        // Replace the original wordlist with the extended one
        wordlist = extended_wordlist;
    }

    // Stop the timer
    let duration_sc = start.elapsed();
    // Print the elapsed time in seconds
    println!("Time elapsed for special char is: {:?}", duration_sc);

    if profile.want_random_numbers {
        let mut extended_wordlist = Vec::new();
        let number_range = nums.clone(); 

        for word in &wordlist {
            // Add the original word
            extended_wordlist.push(word.clone());

            // Extend the word with each number in the range
            for number in number_range.clone() {
                extended_wordlist.push(format!("{}{}", word, number));
            }
        }

        // Replace the original wordlist with the extended one
        wordlist = extended_wordlist;
    }

    // Stop the timer
    let duration_rn = start.elapsed();
    // Print the elapsed time in seconds
    println!("Time elapsed for random number is: {:?}", duration_rn);

    if profile.want_special_chars {
        let special_chars = config.specialchars.chars.clone();
        // Append special characters to each combination
        for (key, combinations) in kombi.iter_mut() {
            let mut extended_combinations = Vec::new();
            for combination in combinations.iter() {
                for char in &special_chars {
                    extended_combinations.push(format!("{}{}", combination, char));
                }
            }
            wordlist.extend(extended_combinations);
        }
    }

    // Stop the timer
    let duration_sc2 = start.elapsed();
    // Print the elapsed time in seconds
    println!("Time elapsed for special char 2 is: {:?}", duration_sc2);
    
    if profile.want_random_numbers {
        let number_range = nums.clone(); 
        // Append numbers to each combination
        for (key, combinations) in kombi.iter_mut() {
            let mut extended_combinations = Vec::new();
            for combination in combinations.iter() {
                for number in number_range.clone() {
                    extended_combinations.push(format!("{}{}", combination, number));
                }
            }
            wordlist.extend(extended_combinations);
        }
    }


    // Stop the timer
    let duration_rn2 = start.elapsed();
    // Print the elapsed time in seconds
    println!("Time elapsed for random number 2 is: {:?}", duration_rn);

    if profile.leet_mode {
        wordlist = apply_leet_transformations(wordlist);
    }

    // Stop the timer
    let duration_leet = start.elapsed();
    // Print the elapsed time in seconds
    println!("Time elapsed for leet is: {:?}", duration_leet);



    // Deduplicate and sort
    wordlist.sort_unstable();
    wordlist.dedup();

    // Writing to a file
    let file_path = format!("{}.txt", profile.name);
    let mut file = File::create(file_path.clone()).expect("Failed to create file");
    for word in wordlist.clone() {
        writeln!(file, "{}", word).expect("Failed to write to file");
    }

    println!("\n[+] Wordlist generated with {} words", wordlist.clone().len());
    println!("[+] File saved as: {}", file_path);
    // Stop the timer
    let duration = start.elapsed();

    // Print the elapsed time in seconds
    println!("Time elapsed is: {:?}", duration);
    hyperspeed_print(&file_path.clone()).expect("Failed to print file");
}


async fn alectodb_download(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let target_file = "alectodb.csv.gz";
    let client = Client::new();

    if !Path::new(target_file).exists() {
        println!("\n[+] Checking if alectodb is not present...");
        let response = client.get(url).send().await?.bytes().await?;
        tokio::fs::write(target_file, &response).await?;
    }

    let gz = GzDecoder::new(File::open(target_file)?);
    let mut rdr = ReaderBuilder::new().from_reader(gz);

    let mut usernames = HashSet::new();
    let mut passwords = HashSet::new();

    for result in rdr.records() {
        let record = result?;
        usernames.insert(record[5].to_string());
        passwords.insert(record[6].to_string());
    }

    let mut gus: Vec<_> = usernames.into_iter().collect();
    let mut gpa: Vec<_> = passwords.into_iter().collect();
    gus.sort_unstable();
    gpa.sort_unstable();

    println!("\n[+] Exporting to alectodb-usernames.txt and alectodb-passwords.txt\n[+] Done.");

    tokio::fs::write("alectodb-usernames.txt", gus.join("\n")).await?;
    tokio::fs::write("alectodb-passwords.txt", gpa.join("\n")).await?;

    Ok(())
}

async fn download_wordlist_http(section: &str, file_names: Vec<&str>, base_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let dir_path = format!("dictionaries/{}", section);
    fs::create_dir_all(&dir_path)?;

    let client = reqwest::Client::new();
    for file_name in file_names {
        let url = format!("{}/{}/{}", base_url, section, file_name);
        let res = client.get(&url).send().await?;

        let file_path = Path::new(&dir_path).join(file_name);
        let mut file = tokio::fs::File::create(file_path).await?;
        let content = res.bytes().await?;
        tokio::io::copy(&mut content.as_ref(), &mut file).await?;
    }

    Ok(())
}

async fn download_wordlist(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    println!("Choose the section you want to download:\n");
    // Print options similar to the Python version...
    println!("     1   Moby            14      french          27      places");
    println!("     2   afrikaans       15      german          28      polish");
    println!("     3   american        16      hindi           29      random");
    println!("     4   aussie          17      hungarian       30      religion");
    println!("     5   chinese         18      italian         31      russian");
    println!("     6   computer        19      japanese        32      science");
    println!("     7   croatian        20      latin           33      spanish");
    println!("     8   czech           21      literature      34      swahili");
    println!("     9   danish          22      movieTV         35      swedish");
    println!("    10   databases       23      music           36      turkish");
    println!("    11   dictionaries    24      names           37      yiddish");
    println!("    12   dutch           25      net             38      exit program");
    println!("    13   finnish         26      norwegian       \n");
    println!("\n\tFiles will be downloaded from {} repository", config.wordlist.dicturl);
    println!("\n\tTip: After downloading wordlist, you can improve it with -w option\n");

    let mut selection = String::new();
    std::io::stdin().read_line(&mut selection).expect("Failed to read line");
    let selection: u32 = selection.trim().parse().expect("Please type a number!");

    // Define your wordlist sections and file names similar to the Python version...
    let arguments : Vec<(usize, &str, Vec<&str>)> = vec![
        (1, "Moby", vec!["mhyph.tar.gz", "mlang.tar.gz", "moby.tar.gz", "mpos.tar.gz", "mpron.tar.gz", "mthes.tar.gz", "mwords.tar.gz"]),
        (2, "afrikaans", vec!["afr_dbf.zip"]),
        (3, "american", vec!["dic-0294.tar.gz"]),
        (4, "aussie", vec!["oz.gz"]),
        (5, "chinese", vec!["chinese.gz"]),
        (6, "computer", vec!["Domains.gz", "Dosref.gz", "Ftpsites.gz", "Jargon.gz", "common-passwords.txt.gz", "etc-hosts.gz", "foldoc.gz", "language-list.gz", "unix.gz"]),
        (7, "croatian", vec!["croatian.gz"]),
        (8, "czech", vec!["czech-wordlist-ascii-cstug-novak.gz"]),
        (9, "danish", vec!["danish.words.gz", "dansk.zip"]),
        (10, "databases", vec!["acronyms.gz", "att800.gz", "computer-companies.gz", "world_heritage.gz"]),
        (11, "dictionaries", vec!["Antworth.gz", "CRL.words.gz", "Roget.words.gz", "Unabr.dict.gz", "Unix.dict.gz", "englex-dict.gz", "knuth_britsh.gz", "knuth_words.gz", "pocket-dic.gz", "shakesp-glossary.gz", "special.eng.gz", "words-english.gz"]),
        (12, "dutch", vec!["words.dutch.gz"]),
        (13, "finnish", vec!["finnish.gz", "firstnames.finnish.gz", "words.finnish.FAQ.gz"]),
        (14, "french", vec!["dico.gz"]),
        (15, "german", vec!["deutsch.dic.gz", "germanl.gz", "words.german.gz"]),
        (16, "hindi", vec!["hindu-names.gz"]),
        (17, "hungarian", vec!["hungarian.gz"]),
        (18, "italian", vec!["words.italian.gz"]),
        (19, "japanese", vec!["words.japanese.gz"]),
        (20, "latin", vec!["wordlist.aug.gz"]),
        (21, "literature", vec!["LCarrol.gz", "Paradise.Lost.gz", "aeneid.gz", "arthur.gz", "cartoon.gz", "cartoons-olivier.gz", "charlemagne.gz", "fable.gz", "iliad.gz", "myths-legends.gz", "odyssey.gz", "sf.gz", "shakespeare.gz", "tolkien.words.gz"]),
        (22, "movieTV", vec!["Movies.gz", "Python.gz", "Trek.gz"]),
        (23, "music", vec!["music-classical.gz", "music-country.gz", "music-jazz.gz", "music-other.gz", "music-rock.gz", "music-shows.gz", "rock-groups.gz"]),
        (24, "names", vec!["ASSurnames.gz", "Congress.gz", "Family-Names.gz", "Given-Names.gz", "actor-givenname.gz", "actor-surname.gz", "cis-givenname.gz", "cis-surname.gz", "crl-names.gz", "famous.gz", "fast-names.gz", "female-names-kantr.gz", "female-names.gz", "givennames-ol.gz", "male-names-kantr.gz", "male-names.gz", "movie-characters.gz", "names.french.gz", "names.hp.gz", "other-names.gz", "shakesp-names.gz", "surnames-ol.gz", "surnames.finnish.gz", "usenet-names.gz"]),
        (25, "net", vec!["hosts-txt.gz", "inet-machines.gz", "usenet-loginids.gz", "usenet-machines.gz", "uunet-sites.gz"]),
        (26, "norwegian", vec!["words.norwegian.gz"]),
        (27, "places", vec!["Colleges.gz", "US-counties.gz", "World.factbook.gz", "Zipcodes.gz", "places.gz"]),
        (28, "polish", vec!["words.polish.gz"]),
        (29, "random", vec!["Ethnologue.gz", "abbr.gz", "chars.gz", "dogs.gz", "drugs.gz", "junk.gz", "numbers.gz", "phrases.gz", "sports.gz", "statistics.gz"]),
        (30, "religion", vec!["Koran.gz", "kjbible.gz", "norse.gz"]),
        (31, "russian", vec!["russian.lst.gz", "russian_words.koi8.gz"]),
        (32, "science", vec!["Acr-diagnosis.gz", "Algae.gz", "Bacteria.gz", "Fungi.gz", "Microalgae.gz", "Viruses.gz", "asteroids.gz", "biology.gz", "tech.gz"]),
        (33, "spanish", vec!["words.spanish.gz"]),
        (34, "swahili", vec!["swahili.gz"]),
        (35, "swedish", vec!["words.swedish.gz"]),
        (36, "turkish", vec!["turkish.dict.gz"]),
        (37, "yiddish", vec!["yiddish.gz"]),
    ];

    if let Some((_, section, file_names)) = arguments.get((selection - 1) as usize) {
        let base_url = &config.wordlist.dicturl;
        if let Err(e) = download_wordlist_http(section, file_names.clone(), base_url).await {
            eprintln!("Error downloading wordlist: {}", e);
        }
    } else {
        println!("Invalid selection.");
    }
    Ok(())
}

fn get_cli_app() -> Command<'static> {
    Command::new("Common User Passwords Profiler")
        .about("A tool for creating personalized wordlists based on user information")
        .arg(Arg::new("interactive")
            .short('i')
            .long("interactive")
            .help("Interactive questions for user password profiling")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("improve")
            .short('w')
            .help("Use this option to improve existing dictionary, or WyD.pl output to make some pwnsauce")
            .takes_value(true))
        .arg(Arg::new("download_wordlist")
            .short('l')
            .help("Download huge wordlists from repository")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("alecto")
            .short('a')
            .help("Parse default usernames and passwords directly from Alecto DB. Project Alecto uses purified databases of Phenoelit and CIRT which were merged and enhanced")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("version")
            .short('v')
            .long("version")
            .help("Show the version of this program.")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("quiet")
            .short('q')
            .long("quiet")
            .help("Quiet mode (don't print banner)")
            .action(clap::ArgAction::SetTrue))
        .group(ArgGroup::new("mode")
            .args(&["interactive", "improve", "download_wordlist", "alecto", "version"]))
}

fn load_wordlist_from_file(filename: &str) -> Result<Vec<String>, io::Error> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let buf = io::BufReader::new(file);

    let mut wordlist = Vec::new();

    for line_result in buf.lines() {
        let line = line_result?;
        if !line.trim().is_empty() { // Skip empty lines
            wordlist.push(line);
        }
    }

    Ok(wordlist)
}

fn hyperspeed_print(filename: &str) -> io::Result<()> {
    println!("> Hyperspeed Print? (Y/n) :");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    if input.trim().eq_ignore_ascii_case("y") {
        let file = File::open(filename)?;
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            println!("[{}] {}", filename, line?);
            thread::sleep(Duration::from_millis(100)); // Sleep for a short duration
        }
    }

    Ok(())
}

fn improve_dictionary(filename: &str, config: &Config) {
    match load_wordlist_from_file(filename) {
        Ok(wordlist) => {
            let improved_wordlist = improve_wordlist(wordlist, &config);
            // Now you can use improved_wordlist as needed
            // Save the improved wordlist back to a file or a new file
            // Writing to a file
            let file_path = format!("{}_improved.txt", filename);
            let mut file = File::create(file_path).expect("Failed to create file");
            for word in improved_wordlist {
                writeln!(file, "{}", word).expect("Failed to write to file");
            }
        },
        Err(e) => eprintln!("Failed to load wordlist: {}", e),
    }
}  

fn improve_wordlist(mut wordlist: Vec<String>, config: &Config) -> Vec<String> {
    let mut improved_wordlist = Vec::new();

    // Append special characters
    for word in &wordlist {
        for special_char in &config.specialchars.chars {
            improved_wordlist.push(format!("{}{}", word, special_char));
        }
    }

    // Append numbers
    for word in &wordlist {
        for number in config.nums.from..=config.nums.to {
            improved_wordlist.push(format!("{}{}", word, number));
        }
    }


    improved_wordlist = apply_leet_transformations(improved_wordlist);

    // Deduplication and sorting
    improved_wordlist.sort_unstable();
    improved_wordlist.dedup();

    improved_wordlist
}


fn main() {
    let config_path = "cupp.cfg"; // Adjust the path as necessary
    match config::load_config(config_path) {
    Ok(config) => {
        println!("Successfully loaded config: {:?}", config);
        // Use the config as needed
        let matches = get_cli_app().get_matches();
        if matches.get_flag("interactive") {
            println!("Interactive mode selected");
            let profile = interactive();
            generate_wordlist_from_profile(&profile, &config);
        } else if let Some(filename) = matches.get_one::<String>("improve") {
            println!("Improving dictionary using file: {}", filename);
            improve_dictionary(filename, &config);
        } else if matches.get_flag("download_wordlist") {
            println!("Downloading wordlists");
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                download_wordlist(&config).await;
            });
        } else if matches.get_flag("alecto") {
            println!("Using Alecto DB");
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                if let Err(e) = alectodb_download(&config.wordlist.alectourl).await {
                    eprintln!("Failed to download or process Alecto DB: {}", e);
                }
            });
        } else if matches.get_flag("version") {
            println!("Version 1.0.0"); 
        }
    
        if matches.get_flag("quiet") {
            println!("Quiet mode enabled");
            // Implement logic for quiet mode
            // TODO 
        }
    },
    Err(e) => eprintln!("Failed to load configuration: {}", e),
    };


}
