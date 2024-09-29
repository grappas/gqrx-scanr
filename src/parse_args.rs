use log::{debug, error};
use std::net::ToSocketAddrs;
use std::net::{IpAddr, Ipv4Addr};
pub fn print_usage(reason: String) -> Result<(), Box<dyn std::error::Error>> {
    // if reaseon is empty return ok
    if reason.is_empty() {
        Ok(())
    } else {
        // print the reason

        let usage_string = r#"
Usage:
        [-h|--host <host>] [-p|--port <port>] [-m|--mode <sweep|bookmark>]
        [-f <central frequency>] [-b|--min <from freq>] [-e|--max <to freq>]
        [-d|--delay <lingering time in milliseconds>]
        [-l|--max-listen <maximum listening time in milliseconds>]
        [-t|--tags <"tag1|tag2|...">]
        [-v|--verbose]

-h, --host <host>            Name of the host to connect. Default: localhost
-p, --port <port>            The number of the port to connect. Default: 7356
-m, --mode <mode>            Scan mode to be used. Default: sweep
                               Possible values for <mode>: sweep, bookmark
-f, --freq <freq>            Frequency to scan with a range of +- 1MHz.
                               Default: the current frequency tuned in Gqrx Incompatible with -b, -e
-b, --min <freq>             Frequency range begins with this <freq> in Hz. Incompatible with -f
-e, --max <freq>             Frequency range ends with this <freq> in Hz. Incompatible with -f
-s, --step <freq>            Frequency step <freq> in Hz. Default: 10000
-d, --delay <time>           Lingering time in milliseconds before the scanner reactivates. Default 2000
-l, --max-listen <time>      Maximum time to listen to an active frequency. Default 0, no maximum
-x, --speed <time>           Time in milliseconds for bookmark scan speed. Default 250 milliseconds.
                               If scan lands on wrong bookmark during search, use -x 500 (ms) to slow down speed
-y  --date                   Date Format, default is 0.
                               0 = mm-dd-yy
                               1 = dd-mm-yy
-q, --squelch_delta <dB>|a<dB> If set creates bottom squelch just for listening.
                             It may reduce unnecessary squelch audio supress.
                             Default: 0.0
                             Ex.: 6.5
                             Place "a" switch before <dB> value to turn into auto mode
                             It will determine squelch delta based on noise floor and
                             <dB> value will determine how far squelch delta will be placed from it.
                             Ex.: a0.5
-a, --squelch_delta_top <dB> It maps squelch levels for an each scanned frequency
                             based on noise floor + provided value.
-u, --udp_listen             Experimental: Trigger listening on UDP audio signal.
                             Make sure that UDP button is pushed.
                             for gqrx>=2.17.5
-t, --tags <"tags">         Filter signals. Match only on frequencies marked with a tag found in "tags"
                             "tags" is a quoted string with a '|' list separator: Ex: "Tag1|Tag2"
                             tags are case insensitive and match also for partial string contained in a tag
                             Works only with -m bookmark scan mode
-v, --verbose               Show verbose output
"#;
        eprintln!("{}", usage_string);
        error!("{}", reason);
        Err(reason.into())
    }
}

pub enum ScanModes {
    Sweep,
    Bookmark,
}

pub struct Args {
    pub host: IpAddr,
    pub port: u16,
    pub mode: ScanModes,
    pub freq: u64,
    pub min: u64,
    pub max: u64,
    pub step: u64,
    pub delay: u64,
    pub max_listen: u64,
    pub speed: u64,
    pub date: u64,
    pub squelch_delta: f64,
    pub squelch_delta_auto: bool,
    pub squelch_delta_top: f64,
    pub udp_listen: bool,
    pub tags: Vec<String>,
    pub verbose: bool,
}

impl Default for Args {
    fn default() -> Self {
        Args {
            host: match parse_address("localhost:0") {
                Ok(ip) => ip,
                Err(_) => Ipv4Addr::new(127, 0, 0, 1).into(),
            },
            port: 7356,
            mode: ScanModes::Sweep,
            freq: 0,
            min: 0,
            max: 0,
            step: 0,
            delay: 2000,
            max_listen: 0,
            speed: 250,
            date: 0,
            squelch_delta: 0.0,
            squelch_delta_auto: false,
            squelch_delta_top: 0.0,
            udp_listen: false,
            tags: Vec::new(),
            verbose: false,
        }
    }
}

impl Args {
    pub fn new() -> Args {
        Args::default()
    }
    pub fn parse(&mut self, mut args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        // add empty string to the end of the vector
        // to make sure that the last argument is not a flag
        args.push("".to_string());
        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "-h" | "--host" => {
                    i += 1;
                    self.host = match parse_address(&args[i]) {
                        Ok(ip) => {
                            debug!("Host: {:?}", ip);
                            ip
                        }
                        Err(e) => {
                            return print_usage(format!(
                                "Failed to parse address: {} {}",
                                args[i], e
                            ))
                        }
                    };
                }
                "-p" | "--port" => {
                    i += 1;
                    self.port = match args[i].parse::<u16>() {
                        Ok(port) => {
                            debug!("Port: {:?}", port);
                            port
                        }
                        Err(e) => {
                            return print_usage(format!("Failed to parse port: {} {}", args[i], e))
                        }
                    };
                }
                "-m" | "--mode" => {
                    i += 1;
                    match args[i].as_str() {
                        "sweep" => {
                            debug!("Mode: Sweep");
                            self.mode = ScanModes::Sweep
                        }
                        "bookmark" => {
                            debug!("Mode: Bookmark");
                            self.mode = ScanModes::Bookmark
                        }
                        _ => {
                            return print_usage(format!("Invalid mode: {}", args[i]));
                        }
                    }
                }
                "-f" | "--freq" => {
                    i += 1;
                    self.freq = match args[i].parse::<u64>() {
                        Ok(freq) => {
                            debug!("Freq: {:?}", freq);
                            freq
                        }
                        Err(e) => {
                            return print_usage(format!(
                                "Failed to parse frequency: {} {}",
                                args[i], e
                            ))
                        }
                    };
                }
                "-b" | "--min" => {
                    i += 1;
                    self.min = match args[i].parse::<u64>() {
                        Ok(min) => {
                            debug!("Min: {:?}", min);

                            min
                        }
                        Err(e) => {
                            return print_usage(format!(
                                "Failed to parse min frequency: {} {}",
                                args[i], e
                            ))
                        }
                    };
                }
                "-e" | "--max" => {
                    i += 1;
                    self.max = match args[i].parse::<u64>() {
                        Ok(max) => {
                            debug!("Max: {:?}", max);
                            max
                        }
                        Err(e) => {
                            return print_usage(format!(
                                "Failed to parse max frequency: {} {}",
                                args[i], e
                            ))
                        }
                    };
                }
                "-s" | "--step" => {
                    i += 1;
                    self.step = match args[i].parse::<u64>() {
                        Ok(step) => {
                            debug!("Step: {:?}", step);

                            step
                        }
                        Err(e) => {
                            return print_usage(format!("Failed to parse step: {} {}", args[i], e))
                        }
                    };
                }
                "-d" | "--delay" => {
                    i += 1;
                    self.delay = match args[i].parse::<u64>() {
                        Ok(delay) => {
                            debug!("Delay: {:?}", delay);

                            delay
                        }
                        Err(e) => {
                            return print_usage(format!("Failed to parse delay: {} {}", args[i], e))
                        }
                    };
                }
                "-l" | "--max-listen" => {
                    i += 1;
                    self.max_listen = match args[i].parse::<u64>() {
                        Ok(max_listen) => {
                            debug!("Max Listen: {:?}", max_listen);
                            max_listen
                        }
                        Err(e) => {
                            return print_usage(format!(
                                "Failed to parse max listen: {} {}",
                                args[i], e
                            ))
                        }
                    };
                }
                "-x" | "--speed" => {
                    i += 1;
                    self.speed = match args[i].parse::<u64>() {
                        Ok(speed) => {
                            debug!("Speed: {:?}", speed);
                            speed
                        }
                        Err(e) => {
                            return print_usage(format!("Failed to parse speed: {} {}", args[i], e))
                        }
                    };
                }
                "-y" | "--date" => {
                    i += 1;
                    self.date = match args[i].parse::<u64>() {
                        Ok(date) => {
                            debug!("Date Format: {:?}", date);

                            date
                        }
                        Err(e) => {
                            return print_usage(format!("Failed to parse date: {} {}", args[i], e))
                        }
                    };
                }
                "-q" | "--squelch_delta" => {
                    i += 1;
                    if args[i].starts_with("a") {
                        self.squelch_delta_auto = true;
                        args[i] = args[i].trim_start_matches('a').to_string();
                    }
                    self.squelch_delta = match args[i].parse::<f64>() {
                        Ok(squelch_delta) => {
                            debug!("Squelch Delta: {:?}", squelch_delta);
                            debug!("Squelch Delta Auto: {:?}", self.squelch_delta_auto);
                            squelch_delta
                        }
                        Err(e) => {
                            return print_usage(format!(
                                "Failed to parse squelch delta: {} {}",
                                args[i], e
                            ))
                        }
                    };
                }
                "-a" | "--squelch_delta_top" => {
                    i += 1;
                    self.squelch_delta_top = match args[i].parse::<f64>() {
                        Ok(squelch_delta_top) => {
                            debug!("Squelch Delta Top: {:?}", squelch_delta_top);
                            squelch_delta_top
                        }
                        Err(e) => {
                            return print_usage(format!(
                                "Failed to parse squelch delta top: {} {}",
                                args[i], e
                            ))
                        }
                    };
                }
                "-u" | "--udp_listen" => {
                    self.udp_listen = true;
                }
                "-t" | "--tags" => {
                    // | is a delimiter for tags
                    i += 1;
                    // check if that next argument is not a next flag or nothing is provided
                    if args[i].starts_with("-") || args[i].is_empty() {
                        return print_usage("No tags provided".to_string());
                    }
                    self.tags = args[i].split('|').map(|s| s.to_string()).collect();
                    debug!("Tags: {:?}", self.tags);
                }
                "-v" | "--verbose" => {
                    self.verbose = true;
                    debug!("Verbose: {:?}", self.verbose);
                }
                "" => {}
                _ => {
                    return print_usage("Invalid argument".to_string());
                }
            }
            i += 1;
        }
        Ok(())
    }
}

pub fn parse_address(input: &str) -> Result<IpAddr, Box<dyn std::error::Error>> {
    // Try to parse the input as an IP address
    debug!("Parsing address: {}", input);
    if let Ok(ip) = input.parse::<IpAddr>() {
        return Ok(ip);
    }
    debug!("Failed to parse as IP address. Trying to parse as DNS address.");
    // If parsing as IP address fails, treat it as a DNS address
    if let Ok(socket_addrs) = input.to_socket_addrs() {
        if let Some(socket_addr) = socket_addrs.into_iter().next() {
            return Ok(socket_addr.ip());
        }
    }
    error!("Failed to parse {input} as DNS address. Falling back to localhost");
    // If the address is neither an IP address nor a DNS address, fallback to localhost
    if let Ok(localhost) = "localhost:0".to_socket_addrs() {
        if let Some(socket_addr) = localhost.into_iter().next() {
            return Ok(socket_addr.ip());
        }
    }
    error!("Failed to parse localhost. Returning error.");
    Err("Unable to parse address".into())
}
