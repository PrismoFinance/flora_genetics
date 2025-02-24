use serde::Deserialize;

// Struct to deserialize the esearch API response
#[derive(Deserialize, Debug)]
pub struct NcbiResponse {
    pub esearchresult: ESearchResult,
}

#[derive(Deserialize, Debug)]
pub struct ESearchResult {
    pub idlist: Vec<String>,
    #[serde(rename = "count")]
    pub count: String,
}

// Struct to deserialize the efetch API response (GenBank format)
#[derive(Deserialize, Debug)]
pub struct NcbiEfetchResponse {
    #[serde(rename = "GBSeq")]
    pub gbseq: Vec<GBSeq>,
}

#[derive(Deserialize, Debug)]
pub struct GBSeq {
    #[serde(rename = "GBSeq_length")]
    pub length: String,
    #[serde(rename = "GBSeq_organism")]
    pub organism: String,
    #[serde(rename = "GBSeq_sequence")]
    pub sequence: String,
    #[serde(rename = "GBSeq_create-date")]
    pub create_date: String,
    #[serde(rename = "GBSeq_references")]
    pub references: Vec<GBReference>,
}

#[derive(Deserialize, Debug)]
pub struct GBReference {
    #[serde(rename = "GBReference_authors")]
    pub authors: Option<Vec<String>>,
}

// Struct to deserialize the efetch API response (FASTA format)
#[derive(Deserialize, Debug)]
pub struct FastaResponse {
    pub header: String,
    pub sequence: String,
}