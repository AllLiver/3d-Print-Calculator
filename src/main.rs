// General imports
use std::fmt::Debug;
use std::io::stdin;

// File imports
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;

// Name for the config file
const CONFIG_NAME: &str = "config.txt";

// Struct for the main configuration
#[derive(Debug)]
struct GeneralConfig {
    name: String,
    hourly_fee: f32,
    markup: f32,
}

// Functions for main configuration
impl GeneralConfig {
    // Creates a GeneralConfig struct from lines in the config file
    fn init(inp_vec: Vec<&str>) -> GeneralConfig {
        let mut new_gconfig = GeneralConfig {
            name: String::from(""),
            hourly_fee: 0.0,
            markup: 0.0,
        };

        // Iterates over every line and puts it into a GeneralConfig struct
        for i in inp_vec {
            let setting: Vec<&str> = i.split("=").collect();

            match setting[0].trim() {
                "printer_name" => new_gconfig.name = setting[1].trim().to_string(),
                "hourly_fee" => {
                    new_gconfig.hourly_fee = setting[1]
                        .trim()
                        .parse::<f32>()
                        .expect("hourly_fee should be a float!")
                }
                "markup" => {
                    new_gconfig.markup = setting[1]
                        .trim()
                        .parse::<f32>()
                        .expect("markup should be a float!")
                }
                _ => {}
            }
        }

        return new_gconfig;
    }
}

// Struct for filament configurations
#[derive(Debug, Clone)]
struct FilamentConfig {
    name: String,
    material: String,
    color: String,
    spool_grams: f32,
    spool_price: f32,
}

// Functions for filament configurations
impl FilamentConfig {
    // Function that writes a new filament config to file and prompts the user for the appropriate values
    fn write_to_file(raw_contents: String) {
        println!("\nWhat is the filament's name?");
        let name: &str = &accept_input();

        println!("\nWhat is the filament's material type?");
        let material: &str = &accept_input();

        println!("\nWhat is the filament's color?");
        let color: &str = &accept_input();

        println!("\nHow many grams of material come with the spool?");
        let spool_grams: &str = &accept_float().to_string();

        println!("\nWhat is the spool's price?");
        let spool_price: &str = &accept_float().to_string();

        let write = raw_contents
            + "\n\n! Filament\nname = "
            + name
            + "\nmaterial = "
            + material
            + "\ncolor = "
            + color
            + "\nspool_grams = "
            + spool_grams
            + "\nspool_price = "
            + spool_price;
        let write = write.as_bytes();

        let mut config_file = File::create(CONFIG_NAME).expect("Could not open a config.txt");
        match File::write(&mut config_file, write) {
            Ok(_) => println!("\nWrote to config successfully!"),
            Err(e) => println!("Unable to write to file! {}", e.to_string()),
        }
    }

    // Function to initialize a FilamentConfig object from lines in config txt
    fn init(inp_vec: Vec<&str>) -> FilamentConfig {
        let mut new_fconfig = FilamentConfig {
            name: String::from(""),
            material: String::from(""),
            color: String::from(""),
            spool_grams: 0.0,
            spool_price: 0.0,
        };

        // Iterates over every line and puts it into a FilamentConfig struct
        for i in inp_vec {
            let setting: Vec<&str> = i.split("=").collect();

            match setting[0].trim() {
                "name" => {
                    new_fconfig.name = setting[1].trim().to_string();
                }
                "material" => {
                    new_fconfig.material = setting[1].trim().to_string();
                }
                "color" => {
                    new_fconfig.color = setting[1].trim().to_string();
                }
                "spool_grams" => {
                    new_fconfig.spool_grams = setting[1]
                        .trim()
                        .parse::<f32>()
                        .expect("spool_grams should be a float!");
                }
                "spool_price" => {
                    new_fconfig.spool_price = setting[1]
                        .trim()
                        .parse::<f32>()
                        .expect("spool_price should be a float!");
                }
                _ => {}
            }
        }
        new_fconfig
    }
}

fn main() -> std::io::Result<()> {
    // Variable that stores how many times main has looped (so it knows how many newlines to put before main menu text)
    let mut loops = 0;

    'main: loop {
        loops += 1;
        // Opens config file or creates one if it is missing or unreadable
        let config_file = match File::open(CONFIG_NAME) {
            Ok(file) => file,
            Err(_) => {
                println!("{CONFIG_NAME} not found! Creating one now...");
                let mut new_file = File::create(CONFIG_NAME)?;
                File::write(&mut new_file, b"# 3d Print Calculator config\n# Do not edit configuration names and only edit values, any incorrect value types will cause the program to crash\n# Any lines starting with a \"!\" mark the beginning of a category, don't mess with or worry about these\n# Filament configurations are added via the main program\n# Any line starting with a \"#\" is a commented line\n\n! General Config\nprinter_name = Printer\nhourly_fee = 0.5\nmarkup = 3.0")?;
                drop(new_file);

                println!("{CONFIG_NAME} generated successfully! If you would like to use anything but the default general config exit the program and configure it.\n");

                File::open(CONFIG_NAME)?
            }
        };

        // Reads the file contents to a string
        let mut buff_reader = BufReader::new(config_file);
        let mut raw_contents = String::new();
        buff_reader.read_to_string(&mut raw_contents)?;

        // Converts the contents to a vector and trims extra whitespace
        let mut contents: Vec<&str> = raw_contents.split("\n").collect();
        for i in 0..contents.len() {
            contents[i] = contents[i].trim();
        }

        // Removes every line that is empty or starts with a #
        let mut contents: Vec<&str> = contents
            .iter()
            .filter(|x| !x.is_empty())
            .filter(|x| !x.starts_with("#"))
            .map(|x| *x)
            .collect();
        
        // Prints extra newlines if it has looped
        if loops > 1 {
            println!("\n\n");
        }
        println!("Enter \"create\" to create a new filament configuration, \"calc\" to calculate a print, or \"exit\" to exit the program.");

        // Input loop: guarantees a valid input
        let action = loop {
            let inp = accept_input();

            if inp != String::from("create") && inp != String::from("calc") && inp != String::from("exit") {
                println!("\nInvalid input, please enter \"create\", \"calc\", or \"exit\"");
                continue;
            }

            break inp;
        };

        // Adds a new filament config to config txt
        if action == "create" {
            FilamentConfig::write_to_file(raw_contents.clone());
            continue 'main;
        } else if action == "calc" {
            // Creates GeneralConfig struct from file contents
            contents.remove(0); // Removes the ! General Config as it is not needed to iterate over everything and read filament configs
            let general_config: Vec<&str> = contents
                .iter()
                .take_while(|x| !x.starts_with("!"))
                .map(|x| *x)
                .collect();
            let general_config = GeneralConfig::init(general_config);

            // Removes already read lines
            drain_for_parse(&mut contents);

            // Generate a list of every filament config in the config txt
            let mut filament_configs: Vec<FilamentConfig> = Vec::new();
            while !contents.is_empty() {
                let filament_config: Vec<&str> = contents
                    .iter()
                    .take_while(|x| !x.starts_with("!"))
                    .map(|x| *x)
                    .collect();

                drain_for_parse(&mut contents);

                filament_configs.push(FilamentConfig::init(filament_config));
            }

            if !filament_configs.is_empty() {
                // Print every filament config's name so the user knows what options there is
                println!("\nAvailible filament configurations:\n");
                for i in &filament_configs {
                    println!(
                        "{}:\n\tMaterial type: {}\n\tColor: {}\n\tWeight {}g\n\tPrice: ${}\n",
                        i.name, i.material, i.color, i.spool_grams, i.spool_price
                    );
                }
                println!("Type in the start of the filament config's name you would like to use.\nYou can also enter \"e\" to exit to the main menu or \"m\" to manually enter a filament config.")
            } else {
                println!("\nIt looks like you do not have any filament configs, enter \"e\" to exit back to the main menu or \"m\" to manually enter a filament config.");
            }

            // Gets input and proper FilamentConfig from user (if not exit)
            let filament: FilamentConfig;
            loop {
                let inp = accept_input();

                // Indexes the filament configurations to see if input matches any of them
                let idx = filament_configs
                    .iter()
                    .position(|x| x.name.starts_with(&inp));

                match idx {
                    // If it does match, set selected filament config to the respective FilamentConfig struct
                    Some(val) => {
                        filament = filament_configs[val].clone();
                        break;
                    }
                    // If it does not, check if the user is manaully entering a FilamentConfig
                    None => {
                        if inp == String::from("m") {
                            println!("\nWhat is the filament's name?");
                            let name: String = accept_input();

                            println!("\nWhat is the filament's material type?");
                            let material: String = accept_input();

                            println!("\nWhat is the filament's color?");
                            let color: String = accept_input();

                            println!("\nHow many grams of material come with the spool?");
                            let spool_grams: f32 = accept_float();

                            println!("\nWhat is the spool's price?");
                            let spool_price: f32 = accept_float();

                            filament = FilamentConfig {
                                name: name,
                                material: material,
                                color: color,
                                spool_grams: spool_grams,
                                spool_price: spool_price,
                            };

                            break;
                        
                        // Check if the user wishes to exit
                        } else if inp == String::from("e") {
                            continue 'main;

                        // If the input does not meet any of the previous conditions, restart the input loop
                        } else {
                            println!("Invalid input! Please enter \"e\", \"m\", or the beginning of any filament config name.\n");
                            continue;
                        }
                    }
                }
            }

            // Take print information from the user
            println!("\nFilament config {} loaded!\n", filament.name);

            println!("How heavy is the print?");
            let print_weight = accept_float();

            println!("\nHow many hours is the print? (ignoring minutes)");
            let print_hours = accept_float();

            println!("\nHow many minutes is the print? (ignoring hours)");
            let print_minutes = accept_float();

            // Calculate the print price
            let mut print_price = print_weight * (filament.spool_price / filament.spool_grams)
                + (general_config.hourly_fee * (print_hours + print_minutes / 60.0));
            print_price *= general_config.markup;

            println!("\nThe print will cost ${print_price}, would you like to write the receipt to a txt file? (y or N)");

            // Write receipt to a txt if the user desires
            if accept_input() == "y" {
                println!("\nWho is the customer?");
                let customer_name = accept_input();

                // Generate file name
                let mut receipt_name = String::new();
                for i in customer_name.to_lowercase().split(" ") {
                    receipt_name += i;
                }

                receipt_name += "RECEIPT"; 

                // Create and write to receipt file
                let mut receipt = File::create(&receipt_name)?;
                let receipt_text = customer_name
                    + "'s receipt\nPrice: $"
                    + &print_price.to_string()
                    + "\nPrinter: "
                    + &general_config.name
                    + "\nFilament: "
                    + &filament.name
                    + "\n\tMaterial: "
                    + &filament.material
                    + "\n\tColor: "
                    + &filament.color;

                let receipt_text = receipt_text.as_bytes();
                File::write(&mut receipt, receipt_text)?;

                println!("Wrote receipt to {receipt_name}");
                continue 'main;
            }  else {
                continue 'main;
            }
        
        // If the user exits in the main menu, stop the program
        } else if action == "exit" {
            panic!("User exit")
        }
        return Ok(());
    }
}

// Function to accept input via the console
fn accept_input() -> String {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Unable to read line!");

    input.trim().to_string()
}

// Special function that forces the user to enter a valid float
fn accept_float() -> f32 {
    let input = loop {
        let inp = accept_input();

        let inp: f32 = match inp.parse() {
            Ok(val) => val,
            Err(_) => {
                println!("\nInvalid input! Please enter a valid float!");
                continue;
            }
        };

        break inp;
    };

    input
}

// Drain the config vector up until a line that starts with a "!"
fn drain_for_parse(contents: &mut Vec<&str>) {
    // Indexes the vector for a "!"
    let temp_index = contents.iter().position(|x| x.starts_with("!"));
    let temp_index = match temp_index {
        Some(val) => val,
        None => contents.len() - 1,
    };
    
    contents.drain(..temp_index + 1);
}