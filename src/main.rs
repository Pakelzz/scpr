use clap::Parser;

use crate::{cli::Cli, error::MyError, storage::{read_config, write_config}};

mod model;
mod api;
mod error;
mod cli;
mod date;
mod validation;
mod storage;


#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    if let Some(name) = &cli.name
        && let Some(date) = cli.date
    {
        let res = api::get_id(name).await;
        match res {
            Ok(id) => {
                
                if cli.default {
                    let res_month = api::get_schedule_month(
                        &id,
                        date::now())
                        .await;
                    match res_month {
                        Ok(s) => {
                            match write_config(s) {
                                Ok(()) => {},
                                Err(e) => eprintln!("{e}")
                            }
                        },
                        Err(e) => eprintln!("{e}")
                    }
                }

                let res = api::get_schedule_time(&id, &date).await;
                match res {
                    Ok(pray) => {
                        pray.print_schedule();
                    },
                    Err(e) => {
                        eprintln!("{e}");
                        std::process::exit(1);
                    }
                }
            },
            Err(e) => eprintln!("{e}")
        }
    } else if let Some(city) = &cli.name {
        let res = api::get_id(city).await;
        match res {
            Ok(id) => {
                if cli.default {
                    let res_month = api::get_schedule_month(
                        &id,
                        date::now())
                        .await;
                    match res_month {
                        Ok(s) => {
                            match write_config(s) {
                                Ok(()) => {},
                                Err(e) => eprintln!("{e}")
                            }
                        },
                        Err(e) => eprintln!("{e}")
                    }
                }

                let res = api::get_schedule_today(&id).await;
                match res {
                    Ok(pray) => {
                        pray.print_schedule();
                    },
                    Err(e) => {
                        eprintln!("{e}");
                        std::process::exit(1);
                    }
                }
            },
            Err(e) => eprintln!("{e}")
        } 
    } else {
        match read_config() {
            Ok(()) => {},
            Err(e) => {
                match e {
                    MyError::ConfigOutOfDate => {
                        let id = api::get_id_local().unwrap();
                        let res_month = api::get_schedule_month(
                            &id,
                            date::now()
                        )
                            .await;

                        match res_month {
                            Ok(s) => {
                                match write_config(s) {
                                    Ok(()) => {
                                        match read_config() {
                                            Ok(()) => {},
                                            Err(e) => eprintln!("{e}")
                                        }
                                    },
                                    Err(e) => eprintln!("{e}")
                                }
                            },
                            Err(e) => eprintln!("{e}")
                        }
                    },
                    _ => eprintln!("{e}")
                }
            }
        }
    }

}
