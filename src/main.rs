extern crate regex;
extern crate reqwest;
use regex::Regex;
use std::fs;
use std::io::Write;
use std::thread;

fn main() {
    //create output directory if it's not there already
    if !std::path::Path::new(&"./out").exists() {
        fs::create_dir("./out").expect("error creating output directory");
    }

    let west_weekday_handle = thread::spawn(|| {
        generate_schedule("west", "weekdays");
    });
    let east_weekday_handle = thread::spawn(|| {
        generate_schedule("east", "weekdays");
    });

    let west_weekends_handle = thread::spawn(|| {
        generate_schedule("west", "weekends");
    });
    let east_weekends_handle = thread::spawn(|| {
        generate_schedule("east", "weekends");
    });

    west_weekday_handle.join().unwrap();
    east_weekday_handle.join().unwrap();
    west_weekends_handle.join().unwrap();
    east_weekends_handle.join().unwrap();

}

//call the metro endpoints used to build the html table, get back a very long string of the html
fn schedule_request(url: String) -> Result<String, reqwest::Error> {
    reqwest::blocking::get(url)?.text()
}

fn filter_content(junk: String) -> String {
    let junk = junk
        .replace("{\"type\":\"success\",\"html\":", "") //remove start and end lines
        .replace("}", "")
        .replace(r#"<\/thead>"#, "\n") //separate into rows and columns
        .replace(r#"<\/tr>"#, "\n")
        .replace(r#"<\/td>"#, ",")
        .replace(r#"<\/th>"#, ",");
    //remove the remaining html tags
    let re = Regex::new(r#"<[\w|\s|\d|=|"|\-|\\|/]*>"#).unwrap();
    let s = re.replace_all(&junk, "");
    s
        .replace(" pm", "P")
        .replace(" am", "A")
        .replace("-", "")
        .replace("\\t", "")
        .replace("\\n", "")
        .replace("\"", "")
        .replace(",\n", "\n")
        .replacen("\n", "", 1)
}

fn save_to_csv(contents: String, name: &str) -> std::io::Result<()> {
    let mut f = fs::File::create(format!("./out/{}", name))?;
    f.write_all(contents.as_bytes())?;
    Ok(())
}

// fn generate_schedule(url: &str, filename: &str) {
fn generate_schedule(direction: &str, day_type: &str) {
    let url = format!("https://www.metrostlouis.org/wp-admin/admin-ajax.php?action=metro_build_metrolink_html_table&direction={direction}&day_type={day_type}");

    if let Ok(schedule) = schedule_request(url) {
        let csv = filter_content(schedule);
        save_to_csv(csv, &format!("{direction}bound-{day_type}-schedule.csv")).expect("error saving csv");
    }
}
