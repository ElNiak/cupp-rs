# RustCUPP
(Common User Passwords Profiler (CUPP))[https://github.com/Mebus/cupp/]  in Rust. 

The most common form of authentication is the combination of a username and a password or passphrase. If both match values stored within a locally stored table, the user is authenticated for a connection. Password strength is a measure of the difficulty involved in guessing or breaking the password through cryptographic techniques or library-based automated testing of alternate values.

A weak password might be very short or only use alphanumberic characters, making decryption simple. A weak password can also be one that is easily guessed by someone profiling the user, such as a birthday, nickname, address, name of a pet or relative, or a common word such as God, love, money or password.

That is why CUPP was born, and it can be used in situations like legal penetration tests or forensic crime investigations.

## Requirements

- Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

## Usage

```bash
$ cd cupp_rust

# Build 
$ cargo build --release
```


```bash
# Run
$ cargo run -- -h
Common User Passwords Profiler 
A tool for creating personalized wordlists based on user information

USAGE:
    cupp_rust [OPTIONS]

OPTIONS:
    -a                   Parse default usernames and passwords directly from Alecto DB. Project
                         Alecto uses purified databases of Phenoelit and CIRT which were merged and
                         enhanced
    -h, --help           Print help information
    -i, --interactive    Interactive questions for user password profiling
    -l                   Download huge wordlists from repository
    -q, --quiet          Quiet mode (don't print banner)
    -v, --version        Show the version of this program.
    -w <improve>         Use this option to improve existing dictionary, or WyD.pl output to make
                         some pwnsauce
```

```bash
# Run binrary
$ ./target/release/cupp_rs -h
OPTIONS:
    -a                   Parse default usernames and passwords directly from Alecto DB. Project
                         Alecto uses purified databases of Phenoelit and CIRT which were merged and
                         enhanced
    -h, --help           Print help information
    -i, --interactive    Interactive questions for user password profiling
    -l                   Download huge wordlists from repository
    -q, --quiet          Quiet mode (don't print banner)
    -v, --version        Show the version of this program.
    -w <improve>         Use this option to improve existing dictionary, or WyD.pl output to make
                         some pwnsauce
```

## Configuration

   CUPP has configuration file cupp.cfg with instructions.

## Benchmarks

Rust version:

```bash
$ ./target/release/cupp_rs -i
Leet mapping for 'a': 4
Special chars: ["!", "@", "#", "$", "%%", "&", "*"]
Num range: 0 to 100
Word length from: 5, to: 12
Threshold: 200
Alecto URL: https://github.com/yangbh/Hammer/raw/b0446396e8d67a7d4e53d6666026e078262e5bab/lib/cupp/alectodb.csv.gz
Downloader URL: http://ftp.funet.fi/pub/unix/security/passwd/crack/dictionaries/
Successfully loaded config: Config { years: Years { years: [1990, 1991, 1992, 1993, 1994, 1995, 1996, 1997, 1998, 1999, 2000, 2001, 2002, 2003, 2004, 2005, 2006, 2007, 2008, 2009, 2010, 2011, 2012, 2013, 2014, 2015, 2016, 2017, 2018, 2019, 2020] }, leet: Leet { a: 4, i: 1, e: 3, t: 7, o: 0, s: 5, g: 9, z: 2 }, specialchars: SpecialChars { chars: ["!", "@", "#", "$", "%%", "&", "*"] }, nums: Nums { from: 0, to: 100 }, wls: Wls { wcfrom: 5, wcto: 12 }, threshold: Threshold { threshold: 200 }, wordlist: Wordlist { alectourl: "https://github.com/yangbh/Hammer/raw/b0446396e8d67a7d4e53d6666026e078262e5bab/lib/cupp/alectodb.csv.gz", dicturl: "http://ftp.funet.fi/pub/unix/security/passwd/crack/dictionaries/" } }

Interactive mode selected

[+] Insert the information about the victim to make a dictionary
[+] If you don't know all the info, just hit enter when asked! ;)

> First Name: Chris     
> Surname: Crock
> Nickname: ElNiak
> Birthdate (DDMMYYYY): 12182000
> Partner's name: Marie
> Partner's nickname: Madeleine
> Partner's birthdate (DDMMYYYY): 10311992
> Child's name: Jesus 
> Child's nickname: Nazareth
> Child's birthdate (DDMMYYYY): 12130000
> Pet's name: Garfield
> Company name: UCLouvain
> Do you want to add some key words about the victim? Y/[N]: N
> Do you want to add special chars at the end of words? Y/[N]: Y
> Do you want to add some random numbers at the end of words? Y/[N]: Y
> Leet mode? (i.e. leet = 1337) Y/[N]: Y
Time elapsed for birthday combination is: 471.062µs
Time elapsed for birthday kombina is: 3.766712ms
Time elapsed for special char is: 4.836014ms
Time elapsed for random number is: 84.831641ms
Time elapsed for special char 2 is: 92.856952ms
Time elapsed for random number 2 is: 84.831641ms
Time elapsed for leet is: 593.471858ms
[+] Wordlist generated with 1413721 words
[+] File saved as: chris.txt
Time elapsed is: 5.270197439s
```

Python version:

```bash
$ python3 cupp.py -i
 ___________ 
   cupp.py!                 # Common
      \                     # User
       \   ,__,             # Passwords
        \  (oo)____         # Profiler
           (__)    )\   
              ||--|| *      [ Muris Kurgas | j0rgan@remote-exploit.org ]
                            [ Mebus | https://github.com/Mebus/]


[+] Insert the information about the victim to make a dictionary
[+] If you don't know all the info, just hit enter when asked! ;)

> First Name: Chris
> Surname: Crock
> Nickname: ElNiak
> Birthdate (DDMMYYYY): 12182000


> Partners) name: Marie
> Partners) nickname: Madeleine
> Partners) birthdate (DDMMYYYY): 10311992


> Child's name: Jesus
> Child's nickname: Nazareth
> Child's birthdate (DDMMYYYY): 12130000


> Pet's name: Garfield
> Company name: UCLouvain


> Do you want to add some key words about the victim? Y/[N]: N
> Do you want to add special chars at the end of words? Y/[N]: Y
> Do you want to add some random numbers at the end of words? Y/[N]:Y
> Leet mode? (i.e. leet = 1337) Y/[N]: Y

[+] Now making a dictionary...
[+] Sorting list and removing duplicates...
duration: 0.19226741790771484
[+] Saving dictionary to chris.txt, counting 34018 words.

```

It is strange that Rust version is slower than Python version per words:
* Rust:   0,000012109s per word
* Python: 0,000005652s per word

It is also strange that the Rust version produce more words than the Python version.

It is my first Rust program, so it will be fun to analyse !

Probably due to:
* Strings in Python are immutable, and one of benefits of this is they can be cloned by reference. This allows an optimization: if one of strings is empty, then there is no concatenation performed, instead reference to another non-empty string is returned. If you change initial string from "" to something else like "_", then they cannot do this trick anymore and have to do a fair byte-by-byte copy. With this small change python runtime will double and will be about the same as Rust. 
* In Rust this kind of optimizations cannot be implemented in existing push_str because it accepts borrowed string slice and taking reference to it will violate ownership rules. 

## TODO

* [ ] Add tests
* [ ] Optimize code
* [ ] Add CI/CD
* [ ] Add more benchmarks
* [ ] Add more documentation
