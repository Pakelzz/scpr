use crate::{api, cli::Cli, date::{self, clock_int}, error::MyError, model::Pray, storage::{read_config, write_config}};

pub fn next(pray: Pray) -> String {
    let vec_pray = [pray.subuh, pray.dzuhur, pray.ashar, pray.maghrib, pray.isya];

    let clock_int = clock_int();
    let mut count = 1;

    for i in &vec_pray {
        let mut parts = i.split(':');
        let hour = parts.next().unwrap().parse::<u32>().unwrap();
        let minute = parts.next().unwrap().parse::<u32>().unwrap();
        let clock = format!("{}{:02}", hour, minute).parse::<u32>().unwrap();

        if clock_int < clock {
            break;
        }
        count += 1;
    }

    let result;
    if count == 2 {
        result = format!("Dzuhur {}", vec_pray[1]);
    } else if count == 3 {
        result = format!("Ashar {}", vec_pray[2]);
    } else if count == 4 {
        result = format!("Maghrib {}", vec_pray[3]);
    } else if count == 5 {
        result = format!("Isya {}", vec_pray[4]);
    } else {
        result = format!("Subuh {}", vec_pray[0]);
    }

    result.to_string()
}

pub async fn run(cli: Cli) {
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
        match read_config(cli.next) {
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
                                        match read_config(cli.next) {
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