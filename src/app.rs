use crate::{date::clock_int, model::Pray};


pub fn next(pray: Pray) -> String {
    let vec_pray = [
        pray.subuh,
        pray.dzuhur,
        pray.ashar,
        pray.maghrib,
        pray.isya
    ];

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

#[test]
fn test_next() {
    let pray = Pray {
        kabko: "KAB. Pasuruan".to_string(),
        date: "14/02/2026".to_string(),
        subuh: "04:13".to_string(),
        dzuhur: "11:46".to_string(),
        ashar: "14:58".to_string(),
        maghrib: "17:57".to_string(),
        isya: "19:08".to_string(),
    };
    pray.print_schedule();
    println!("\nNext pray");
    println!("{}", next(pray));
}
