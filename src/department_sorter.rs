/**
 * Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company.
 * For example, “Add Sally to Engineering” or “Add Amir to Sales.”
 * Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
 */
pub mod department_sorter {
    use colored::Colorize;
    use serde_json;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{self, prelude::*, BufWriter};

    const SAVE_FILE_PATH: &str = "./dapartments.json";

    type DepartmentSorterDB = HashMap<String, Vec<String>>;

    struct DepartmentSorter {
        storage: DepartmentSorterDB,
    }

    pub fn runner() {
        println!(
            "\n\nWelcome to {}®",
            "Department Sorter".underline().bright_green()
        );
        print_help(None);

        let mut department_runner_instance = DepartmentSorter::new();
        let mut continue_run = true;

        while continue_run {
            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            continue_run = department_runner_instance.cmd_parser(input);
        }
    }

    fn print_help(error_txt: Option<&str>) {
        if error_txt.is_some() {
            println!("{}", error_txt.unwrap().yellow());
        }
        println!("Here is what you can do:");
        println!(
            "\t{}: You can add employees to departments | eg: \"Add Sally to Engineering",
            "Add".blue()
        );
        println!(
            "\t{}: You can remove employees from departments | eg: \"Remove Bob from HR",
            "Remove".blue()
        );
        println!(
            "\t{}: You can list employees from departments | eg: \"List Sales",
            "List".blue()
        );
        println!(
            "\t{}: You can save your data to a localized file.",
            "Save".blue()
        );
        println!(
            "\t{}: You can close the application. Your data will be saved before closing.",
            "Close".blue()
        );
    }

    impl DepartmentSorter {
        fn new() -> Self {
            let saved_data = Self::load_data();

            match saved_data {
                Some(data) => DepartmentSorter { storage: data },
                None => {
                    return DepartmentSorter {
                        storage: HashMap::new(),
                    }
                }
            }
        }

        fn add_to_department(&mut self, department_name: &str, employee: &str) {
            let department = self.storage.get_mut(department_name);

            match department {
                Some(department_vec) => {
                    department_vec.push(employee.to_string());
                }
                None => {
                    let dpeartment_vec = vec![employee.to_string()];
                    self.storage
                        .insert(department_name.to_string(), dpeartment_vec);
                }
            }
        }

        fn remove_from_department(&mut self, department_name: &str, employee: &str) {
            let department = self.storage.get_mut(department_name);

            match department {
                Some(department_vec) => {
                    department_vec.retain(|entry| *entry != employee.to_string());
                    println!(
                        "Removed employee {} from {}",
                        employee.yellow(),
                        department_name.yellow()
                    )
                }
                None => {
                    println!("Found no department called: {}", department_name.yellow())
                }
            }
        }

        fn list_department(&mut self, department_name: &str) {
            let department = self.storage.get_mut(department_name);

            match department {
                Some(department) => {
                    department.sort();
                    println!(
                        "Listing all employees in {}: {:?}",
                        department_name.green(),
                        department
                    );
                }
                None => {
                    println!("Found no department called: {}", department_name.yellow())
                }
            }
        }

        fn load_data() -> Option<DepartmentSorterDB> {
            let file = File::open(SAVE_FILE_PATH);
            match file {
                Ok(mut file) => {
                    let mut contents = String::new();
                    file.read_to_string(&mut contents)
                        .expect("Failed to process file");
                    let store = serde_json::from_str(&contents);

                    match store {
                        Ok(valid_store) => {
                            println!(
                                "{} {}",
                                "Restored data successfully from".green(),
                                SAVE_FILE_PATH.yellow()
                            );
                            return valid_store;
                        }
                        Err(_) => return None,
                    }
                }
                Err(_) => {
                    println!(
                        "{}",
                        "No previous record found. Starting from scratch.".yellow()
                    );
                    return None;
                }
            }
        }

        fn save_data(&self) {
            let file = File::create(SAVE_FILE_PATH);
            match file {
                Ok(file) => {
                    let mut writer = BufWriter::new(file);
                    let store = serde_json::to_writer(&mut writer, &self.storage);

                    writer.flush().unwrap();
                    match store {
                        Ok(_) => {
                            println!(
                                "{} {}",
                                "Successfully saved data to: ".green(),
                                SAVE_FILE_PATH.yellow()
                            )
                        }
                        Err(_) => {}
                    }
                }
                Err(err) => {
                    println!("{} {:?}", "Could not save data!".red(), err);
                }
            }
        }

        fn cmd_parser(&mut self, cmd: String) -> bool {
            match cmd.to_lowercase() {
                s if s.starts_with("add") => {
                    let cmd_arr: Vec<&str> = cmd.split_whitespace().collect();

                    if cmd_arr.len() < 4 {
                        print_help(Some("Bad fromat, try again."));
                        return true;
                    }
                    self.add_to_department(cmd_arr[3], cmd_arr[1]);
                }
                s if s.starts_with("remove") => {
                    let cmd_arr: Vec<&str> = cmd.split_whitespace().collect();

                    if cmd_arr.len() < 4 {
                        print_help(Some("Bad fromat, try again."));
                        return true;
                    }
                    self.remove_from_department(cmd_arr[3], cmd_arr[1]);
                }
                s if s.starts_with("list") => {
                    let cmd_arr: Vec<&str> = cmd.split_whitespace().collect();

                    if cmd_arr.len() < 2 {
                        print_help(Some("Bad fromat, try again."));
                        return true;
                    }
                    self.list_department(cmd_arr[1]);
                }
                s if s.contains("close") => {
                    self.save_data();
                    return false;
                }
                s if s.contains("save") => {
                    self.save_data();
                }
                _ => {
                    print_help(Some("Unknown command, try again..."));
                }
            }
            return true;
        }
    }
}
