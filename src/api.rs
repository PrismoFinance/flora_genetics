use reqwest::Error;



// Build the esearch API URL
pub fn build_esearch_url(query: &str, field: &str) -> String {
    format!(
        "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esearch.fcgi?db=nuccore&term={}[{}]&retmode=json",
        query, field
    )
}

// Build the efetch API URL
pub fn build_efetch_url(id: &str, rettype: &str) -> String {
    format!(
        "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/efetch.fcgi?db=nuccore&id={}&rettype={}&retmode=json",
        id, rettype
    )
}

// Fetch API response
pub async fn fetch_api_response(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}