use ureq;
// use serde_json::Value;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Template {
    pub name: String,
    pub source: String
}

fn make_request(url: String) -> Result<String, ()> {
    let request = ureq::get(&url)
        .set("Accept", "application/vnd.github.v3+json")
        .call();

    let body = match request {
        Ok(req) => req.into_string(),
        Err(_) => return Err(())
    };

    let data: String = match body {
        Ok(data) => data,
        Err(_) => return Err(())
    };

    Ok(data)
}

pub fn get_template_list() -> Result<Vec<String>, ()> {
    let json_text = make_request(
        "https://api.github.com/gitignore/templates".to_string()
    )?;

    let out: Vec<String> = serde_json::from_str(&json_text)
        .unwrap_or(vec![]);

    Ok(out)
}

pub fn get_template(name: String) -> Result<Template, ()> {
    let json_text = make_request(
        format!("https://api.github.com/gitignore/templates/{}", name)
    )?;

    let template: Template = match serde_json::from_str(&json_text) {
        Ok(t) => t,
        Err(_) => return Err(())
    };

    Ok(template)
}
