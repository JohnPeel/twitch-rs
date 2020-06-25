use std::error::Error;

pub fn extend_url<S: Into<String>, T: serde::Serialize>(url: S, name: S, value: &Option<Vec<T>>) -> Result<String, Box<dyn Error>> {
    let mut url = url.into();
    if let Some(ref value) = value {
        let name = name.into();
        let extension = serde_urlencoded::to_string(&value.iter().map(|x| (&name, x)).collect::<Vec<(&String, &T)>>())?;
        if !extension.is_empty() {
            url.push_str("?");
            url.push_str(&extension);
        }
    };
    Ok(url)
}