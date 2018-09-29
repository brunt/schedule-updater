extern crate reqwest;

use std::fs;
use std::io::Write;
use std::thread;
use time::Duration;

fn main() {
    //TODO: random thread::sleep offset on each child thread to bypass the site's rate limits
    //TODO: finish formatting of response to a useful csv
    let west_weekday_handle = thread::spawn(|| {
        thread::sleep(Duration::from_secs())
        match schedule_request("https://www.metrostlouis.org/wp-admin/admin-ajax.php?action=metro_build_metrolink_html_table&direction=west&day_type=weekdays"){
        Ok(s) => {
            let csv = filter_content(s);
            save_to_csv(csv, "westbound-weekday-schedule.csv").unwrap();
        }
        Err(_) => {
            writeln!(std::io::stderr(), "error retrieving westbound weekday schedule");
        }
    }
    });

    let east_weekday_handle = thread::spawn(|| {
        match schedule_request("https://www.metrostlouis.org/wp-admin/admin-ajax.php?action=metro_build_metrolink_html_table&direction=east&day_type=weekdays"){
            Ok(s) => {
                let csv = filter_content(s);
                save_to_csv(csv, "westbound-weekday-schedule.csv").unwrap();
            }
            Err(_) => {
                writeln!(std::io::stderr(), "error retrieving westbound weekday schedule");
            }
        }
    });

    let west_saturday_handle = thread::spawn(|| {
        match schedule_request("https://www.metrostlouis.org/wp-admin/admin-ajax.php?action=metro_build_metrolink_html_table&direction=west&day_type=saturday"){
            Ok(s) => {
                let csv = filter_content(s);
                save_to_csv(csv, "westbound-saturday-schedule.csv").unwrap();
            }
            Err(_) => {
                writeln!(std::io::stderr(), "error retrieving westbound saturday schedule");
            }
        }
    });
    let east_saturday_handle = thread::spawn(|| {
        match schedule_request("https://www.metrostlouis.org/wp-admin/admin-ajax.php?action=metro_build_metrolink_html_table&direction=east&day_type=saturday"){
            Ok(s) => {
                let csv = filter_content(s);
                save_to_csv(csv, "westbound-saturday-schedule.csv").unwrap();
            }
            Err(_) => {
                writeln!(std::io::stderr(), "error retrieving westbound saturday schedule");
            }
        }
    });
    let west_sunday_handle = thread::spawn(|| {
        match schedule_request("https://www.metrostlouis.org/wp-admin/admin-ajax.php?action=metro_build_metrolink_html_table&direction=west&day_type=sunday"){
            Ok(s) => {
                let csv = filter_content(s);
                save_to_csv(csv, "westbound-sunday-schedule.csv").unwrap();
            }
            Err(_) => {
                writeln!(std::io::stderr(), "error retrieving westbound sunday schedule");
            }
        }
    });
    let east_sunday_handle = thread::spawn(|| {
        match schedule_request("https://www.metrostlouis.org/wp-admin/admin-ajax.php?action=metro_build_metrolink_html_table&direction=east&day_type=sunday"){
            Ok(s) => {
                let csv = filter_content(s);
                save_to_csv(csv, "eastbound-sunday-schedule.csv").unwrap();
            }
            Err(_) => {
                writeln!(std::io::stderr(), "error retrieving eastbound sunday schedule");
            }
        }
    });

    west_weekday_handle.join().unwrap();
    east_weekday_handle.join().unwrap();
    west_saturday_handle.join().unwrap();
    east_saturday_handle.join().unwrap();
    west_sunday_handle.join().unwrap();
    east_sunday_handle.join().unwrap();
}

//call the metro endpoints used to build the html table, get back a very long string of the html
fn schedule_request(url: &str) -> Result<String, reqwest::Error> {
    let table = reqwest::get(url)?.text()?;
    Ok(table)
}

//edit string or return clone? depends what the string helpers do
fn filter_content(junk: String) -> String {
    return junk.replace("{\"type\":\"success\",\"html\":", "") //remove start and end lines
        .replace("}", "")
        .replace(" pm", "P") //convert am/pm to A/P
        .replace(" am", "A")
        .replace("metro", "") //remove html class words
        .replace("undefined", "")
        .replace("blue", "")
        .replace("line", "")
        .replace("background", "")
        .replace("red", "")
        .replace("bold", "")
        .replace("indicator", "")
        .replace("schedule", "")
        .replace("system", "")
        .replace("table", "")
        .replace("link", "")
        .replace("-td", "")
        .replace("span", "") //remove html tag names
        .replace("class=", "")
        .replace("thead", "")
        .replace("time", "")
        .replace("/", "") //remove particles
//        .replace("\t", "")
        .replace("\\t", "")
        .replace("\n", "")
        .replace("\\n", "")
        .replace("\\", "")
        .replace("\"", "")
        .replace("<", "")
        .replace(">", "")
        .replace("tr", "\n")
        .replace("-", "")
        .replace("td", ",") //comma-separate
        .replace("  ", "") //truncate spaces
        .replace(", ", ",")
        .replace("d,", ",")
        .replace(",d", ","); //trim whitespace
}

//write to filesystem
fn save_to_csv(contents: String, name: &str) -> std::io::Result<()> {
    let mut f = fs::File::create(format!("../out/{}", name))?;
    f.write_all(contents.as_bytes())?;
    Ok(())
}

//    let contents = fs::read_to_string("sample-data/html-response.txt").expect("something went wrong")
//        .replace("{\"type\":\"success\",\"html\":", "") //remove start and end lines
//        .replace("}", "")
//        .replace(" pm", "P") //convert am/pm to A/P
//        .replace(" am", "A")
//        .replace("metro", "") //remove html class words
//        .replace("undefined", "")
//        .replace("blue", "")
//        .replace("line", "")
//        .replace("background", "")
//        .replace("red", "")
//        .replace("bold", "")
//        .replace("indicator", "")
//        .replace("schedule", "")
//        .replace("system", "")
//        .replace("table", "")
//        .replace("link", "")
//        .replace("-td", "")
//        .replace("span", "") //remove html tag names
//        .replace("class=", "")
//        .replace("thead", "")
//        .replace("time", "")
//        .replace("/", "") //remove particles
////        .replace("\t", "")
//        .replace("\\t", "")
//        .replace("\n", "")
//        .replace("\\n", "")
//        .replace("\\", "")
//        .replace("\"", "")
//        .replace("<", "")
//        .replace(">", "")
//        .replace("tr", "\n")
//        .replace("-", "")
//        .replace("td", ",") //comma-separate
//        .replace("  ", "") //truncate spaces
//        .replace(", ", ","); //trim whitespace
//
//
//    println!("{:?}", contents);
