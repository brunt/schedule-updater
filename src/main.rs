extern crate reqwest;
extern crate regex;
use std::fs;
use std::io::Write;
use std::thread;
use regex::Regex;

fn main() {
    //create output directory if it's not there already
    if !std::path::Path::new(&"./out").exists() {
        std::fs::create_dir("./out").expect("error creating output directory");
    }

    let west_weekday_handle = thread::spawn(|| {
        generate_schedule(
            "https://www.metrostlouis.org/wp-admin/admin-ajax.php?action=metro_build_metrolink_html_table&direction=west&day_type=weekdays",
            "westbound-weekday-schedule.csv");
    });
    let east_weekday_handle = thread::spawn(|| {
        generate_schedule(
            "https://www.metrostlouis.org/wp-admin/admin-ajax.php?action=metro_build_metrolink_html_table&direction=east&day_type=weekdays",
            "eastbound-weekday-schedule.csv"
        );
    });

    let west_saturday_handle = thread::spawn(|| {
        generate_schedule(
            "https://www.metrostlouis.org/wp-admin/admin-ajax.php?action=metro_build_metrolink_html_table&direction=west&day_type=saturday",
            "westbound-saturday-schedule.csv"
        );
    });
    let east_saturday_handle = thread::spawn(|| {
        generate_schedule(
            "https://www.metrostlouis.org/wp-admin/admin-ajax.php?action=metro_build_metrolink_html_table&direction=east&day_type=saturday",
            "eastbound-saturday-schedule.csv"
        );
    });
    let west_sunday_handle = thread::spawn(|| {
        generate_schedule(
            "https://www.metrostlouis.org/wp-admin/admin-ajax.php?action=metro_build_metrolink_html_table&direction=west&day_type=sunday",
            "westbound-sunday-schedule.csv"
        );
    });
    let east_sunday_handle = thread::spawn(|| {
        generate_schedule(
            "https://www.metrostlouis.org/wp-admin/admin-ajax.php?action=metro_build_metrolink_html_table&direction=east&day_type=sunday",
            "eastbound-sunday-schedule.csv"
        );
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

fn filter_content(junk: String) -> String {
    let junk = junk.replace("{\"type\":\"success\",\"html\":", "") //remove start and end lines
        .replace("}", "")
        .replace(r#"<\/thead>"#, "\n") //separate into rows and columns
        .replace(r#"<\/tr>"#, "\n")
        .replace(r#"<\/td>"#, ",");
    //remove the remaining html tags
    let re = Regex::new(r#"<[\w|\s|\d|=|"|\-|\\|/]*>"#).unwrap();
    let s = re.replace_all(&junk, "");
    return s
        .replace(" pm", "P")
        .replace(" am", "A")
        .replace("-", "")
        .replace("\\t", "")
        .replace("\\n", "")
        .replace("\"", "")
        .replace(",\n", "\n")
        .replacen("\n", "", 1);
}

fn save_to_csv(contents: String, name: &str) -> std::io::Result<()> {
    let mut f = fs::File::create(format!("./out/{}", name))?;
    f.write_all(contents.as_bytes())?;
    Ok(())
}

fn generate_schedule(url: &str, filename: &str) {
    match schedule_request(url) {
        Ok(s) => {
            let csv = filter_content(s);
            save_to_csv(csv, filename).unwrap();
        }
        Err(_) => {
            writeln!(std::io::stderr(), "error retrieving schedule from url");
        }
    }
}