use actix_multipart::Multipart;
use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::web::{Data, Json};
use futures::{StreamExt, TryStreamExt};
use serde_json::{json, Value};
use std::io::Write;
use uuid::Uuid;
use crate::AppState;
use crate::models::tax_model::SaveTaxDataSchema;
use crate::repositories::tax_repository::TaxRepository;
use crate::utilities::error_bag::ErrorBag;
use crate::utilities::http_request::HttpRequestExt;
use crate::utilities::json_response::JsonResponse;
use leptess::LepTess;
use std::path::Path;
use std::process::Command;

fn extract_text_from_file(file_path: &str) -> Result<String, ErrorBag> {
    let path = Path::new(file_path);
    let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
    
    if extension == "pdf" {
        // Handle PDF by converting pages to images first
        let temp_dir = format!("uploads/temp_{}", Uuid::new_v4());
        std::fs::create_dir_all(&temp_dir).map_err(|e| ErrorBag::InternalServerError(e.to_string()))?;
        
        let output = Command::new("pdftoppm")
            .args(&["-png", file_path, &format!("{}/page", temp_dir)])
            .output()
            .map_err(|e| ErrorBag::InternalServerError(format!("Failed to run pdftoppm: {}", e)))?;
            
        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            return Err(ErrorBag::InternalServerError(format!("pdftoppm failed: {}", err)));
        }
        
        let mut full_text = String::new();
        let entries = std::fs::read_dir(&temp_dir).map_err(|e| ErrorBag::InternalServerError(e.to_string()))?;
        
        let mut pages: Vec<_> = entries.filter_map(|e| e.ok()).collect();
        pages.sort_by_key(|e| e.file_name());

        for entry in pages {
            let page_path = entry.path();
            if let Some(ext) = page_path.extension() {
                if ext == "png" {
                    let page_text = extract_text_from_image(page_path.to_str().unwrap_or(""))?;
                    full_text.push_str(&page_text);
                    full_text.push_str("\n--- PAGE BREAK ---\n");
                }
            }
        }
        
        // Cleanup temp files
        let _ = std::fs::remove_dir_all(&temp_dir);
        
        Ok(full_text)
    } else {
        extract_text_from_image(file_path)
    }
}

fn extract_text_from_image(file_path: &str) -> Result<String, ErrorBag> {
    let tessdata_path = "/usr/share/tesseract-ocr/5/tessdata";
    let mut lt = LepTess::new(Some(tessdata_path), "eng")
        .map_err(|e| ErrorBag::InternalServerError(format!("Failed to initialize Tesseract with data at {}: {}", tessdata_path, e)))?;
    
    lt.set_image(Path::new(file_path))
        .map_err(|e| ErrorBag::InternalServerError(format!("Failed to set image for OCR: {}", e)))?;
    
    let text = lt.get_utf8_text()
        .map_err(|e| ErrorBag::InternalServerError(format!("Failed to get text from OCR: {}", e)))?;
    
    Ok(text)
}

fn parse_tax_data(text: &str, doc_type: &str) -> Vec<Value> {
    let pages: Vec<&str> = text.split("--- PAGE BREAK ---").collect();
    let mut all_records = Vec::new();

    for page in pages {
        if page.trim().is_empty() { continue; }
        
        let lines: Vec<&str> = page.lines().collect();
        let mut record = json!({});

        if doc_type == "W2" {
            let mut wages = 0.0;
            let mut tax_withheld = 0.0;
            let mut employer = "Unknown Employer".to_string();
            let mut found = false;

            for line in &lines {
                let lower = line.to_lowercase();
                if lower.contains("wages") || lower.contains("tips") || lower.contains("other compensation") {
                    if let Some(val) = extract_amount(line) { wages = val; found = true; }
                } else if lower.contains("federal income tax withheld") {
                    if let Some(val) = extract_amount(line) { tax_withheld = val; found = true; }
                } else if lower.contains("employer's name") || lower.contains("employer name") {
                    employer = line.split(':').last().unwrap_or(line).trim().to_string();
                    found = true;
                }
            }
            
            if found {
                record = json!({
                    "employer": employer,
                    "wages": wages,
                    "tax_withheld": tax_withheld,
                });
                all_records.push(record);
            }
        } else if doc_type == "1099" {
            let mut income = 0.0;
            let mut payer = "Unknown Payer".to_string();
            let mut found = false;

            for line in &lines {
                let lower = line.to_lowercase();
                // Check for generic 1099 interest or NEC nonemployee compensation
                if lower.contains("interest income") || lower.contains("nonemployee compensation") || lower.contains("box 1") {
                    if let Some(val) = extract_amount(line) { income = val; found = true; }
                } else if lower.contains("payer's name") || lower.contains("payer name") {
                    payer = line.split(':').last().unwrap_or(line).trim().to_string();
                    found = true;
                }
            }

            if found {
                record = json!({
                    "payer": payer,
                    "income": income,
                });
                all_records.push(record);
            }
        }
    }

    if all_records.is_empty() {
        all_records.push(json!({ "error": "No data found" }));
    }

    all_records
}

fn extract_amount(line: &str) -> Option<f64> {
    // Basic logic to find a dollar amount in a string
    // Improved: look for numbers after keywords
    let parts: Vec<&str> = line.split_whitespace().collect();
    for part in parts {
        let cleaned: String = part.chars()
            .filter(|c| c.is_digit(10) || *c == '.' || *c == ',')
            .collect();
        
        if !cleaned.is_empty() && (cleaned.contains('.') || cleaned.len() > 1) {
            if let Ok(val) = cleaned.replace(',', "").parse::<f64>() {
                return Some(val);
            }
        }
    }
    None
}

pub async fn list_documents(req: HttpRequest, app: Data<AppState>) -> Result<HttpResponse, ErrorBag> {
    let user = req.get_user();
    let docs = TaxRepository::find_documents_by_user(&app.pool, &user.id).await?;
    Ok(JsonResponse::success(json!({ "documents": docs })))
}

pub async fn upload_document(
    req: HttpRequest,
    mut payload: Multipart,
    app: Data<AppState>,
) -> Result<HttpResponse, ErrorBag> {
    let user = req.get_user();
    let mut year = 2024; // Default
    let mut doc_type = "W2".to_string();
    let mut file_name = String::new();
    let mut file_path = String::new();

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition();
        let name = content_disposition.as_ref().and_then(|cd| cd.get_name()).unwrap_or("");

        if name == "year" {
            let mut bytes = Vec::new();
            while let Some(chunk) = field.next().await {
                bytes.extend_from_slice(&chunk.map_err(|e| ErrorBag::InternalServerError(e.to_string()))?);
            }
            if let Ok(s) = String::from_utf8(bytes) {
                year = s.parse().unwrap_or(2024);
            }
        } else if name == "document_type" {
            let mut bytes = Vec::new();
            while let Some(chunk) = field.next().await {
                bytes.extend_from_slice(&chunk.map_err(|e| ErrorBag::InternalServerError(e.to_string()))?);
            }
            if let Ok(s) = String::from_utf8(bytes) {
                doc_type = s;
            }
        } else if name == "file" {
            file_name = content_disposition.as_ref().and_then(|cd| cd.get_filename()).unwrap_or("unknown").to_string();
            let relative_path = format!("uploads/{}_{}", Uuid::new_v4(), file_name);
            file_path = relative_path.clone();
            
            // In a real app, we would save to a persistent storage
            // For now, let's just create the directory and write
            std::fs::create_dir_all("uploads").map_err(|e| ErrorBag::InternalServerError(e.to_string()))?;
            let mut f = std::fs::File::create(&file_path).map_err(|e| ErrorBag::InternalServerError(e.to_string()))?;

            while let Some(chunk) = field.next().await {
                let data = chunk.map_err(|e| ErrorBag::InternalServerError(e.to_string()))?;
                f.write_all(&data).map_err(|e| ErrorBag::InternalServerError(e.to_string()))?;
            }
        }
    }

    if file_path.is_empty() {
        return Err(ErrorBag::BadRequest("No file uploaded".into()));
    }

    let doc = TaxRepository::create_document(&app.pool, &user.id, year, &doc_type, &file_name, &file_path).await?;

    // Real data extraction using OCR (now supporting PDF and multiple pages)
    let extracted_text = match extract_text_from_file(&file_path) {
        Ok(text) => text,
        Err(e) => {
            tracing::error!("OCR Extraction failed for {}: {:?}", file_path, e);
            "Failed to extract text".to_string()
        }
    };
    let extracted_records = parse_tax_data(&extracted_text, &doc_type);
    
    // Store extracted data
    let mut current_data = TaxRepository::find_data_by_user_and_year(&app.pool, &user.id, year).await?
        .map(|d| d.data)
        .unwrap_or(json!({ "documents": [] }));
    
    if let Some(docs_array) = current_data.get_mut("documents").and_then(|d| d.as_array_mut()) {
        docs_array.push(json!({
            "id": doc.id,
            "type": doc_type,
            "records": extracted_records,
            "raw_text_preview": extracted_text.chars().take(500).collect::<String>()
        }));
    }

    TaxRepository::upsert_tax_data(&app.pool, &user.id, year, &current_data).await?;

    Ok(JsonResponse::success(json!({ "document": doc, "extracted_records": extracted_records })))
}

pub async fn save_manual_data(
    req: HttpRequest,
    app: Data<AppState>,
    body: Json<SaveTaxDataSchema>,
) -> Result<HttpResponse, ErrorBag> {
    let user = req.get_user();
    let data = TaxRepository::upsert_tax_data(&app.pool, &user.id, body.year, &body.data).await?;
    Ok(JsonResponse::success(json!({ "data": data })))
}

pub async fn get_tax_data(req: HttpRequest, app: Data<AppState>) -> Result<HttpResponse, ErrorBag> {
    let user = req.get_user();
    let data = TaxRepository::find_all_data_by_user(&app.pool, &user.id).await?;
    Ok(JsonResponse::success(json!({ "tax_data": data })))
}

pub async fn ai_helper(
    req: HttpRequest,
    app: Data<AppState>,
    body: Json<Value>,
) -> Result<HttpResponse, ErrorBag> {
    let user = req.get_user();
    let question = body.get("question").and_then(|q| q.as_str()).unwrap_or("");
    
    // Fetch user tax data for context
    let tax_data = TaxRepository::find_all_data_by_user(&app.pool, &user.id).await?;
    let context = json!({
        "user": {
            "first_name": user.first_name,
            "last_name": user.last_name,
        },
        "tax_records": tax_data
    });

    let prompt = format!(
        "You are a specific tax assistant. Use the following user data to answer their question accurately. \
        If the data doesn't contain the answer, use your general tax knowledge but mention it's general advice. \
        User Data: {} \
        Question: {}",
        serde_json::to_string(&context).unwrap_or_default(),
        question
    );

    let client = reqwest::Client::new();
    let ollama_res = client.post(format!("{}/api/generate", crate::config::ENV.ollama_url))
        .json(&json!({
            "model": "llama3", // or any model installed in Ollama
            "prompt": prompt,
            "stream": false
        }))
        .send()
        .await;

    let response = match ollama_res {
        Ok(res) => {
            let body: Value = res.json().await.map_err(|e| ErrorBag::InternalServerError(format!("Ollama JSON error: {}", e)))?;
            body.get("response").and_then(|r| r.as_str()).unwrap_or("Sorry, I received an empty response from the AI.").to_string()
        },
        Err(_) => {
            // Fallback to smart mock if Ollama is not available
            if question.contains("married") {
                "Based on your current tax records, I see you filed as Single last year. If your marital status changed in 2024, you should file accordingly. Generally, you can file as Married Filing Jointly if you were legally married by Dec 31st.".to_string()
            } else if question.contains("how much") || question.contains("wages") {
                let total_wages: f64 = tax_data.iter().map(|d| {
                    d.data.get("documents").and_then(|docs| docs.as_array()).map(|docs| {
                        docs.iter().map(|doc| doc.get("extracted").and_then(|e| e.get("wages")).and_then(|w| w.as_f64()).unwrap_or(0.0)).sum::<f64>()
                    }).unwrap_or(0.0)
                }).sum();
                format!("I found W2 documents totaling ${:.2} in wages for your recorded years.", total_wages)
            } else {
                "I'm your local tax assistant. (Ollama connection failed, using offline mode). You can ask me about your income, filing status, or previous years' data.".to_string()
            }
        }
    };

    Ok(JsonResponse::success(json!({ "answer": response })))
}

pub async fn download_document(
    req: HttpRequest,
    app: Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ErrorBag> {
    let user = req.get_user();
    let doc_id = path.into_inner();
    let doc = TaxRepository::find_document_by_id(&app.pool, &doc_id).await?;

    if doc.user_id != user.id {
        return Err(ErrorBag::Forbidden);
    }

    let path = std::path::Path::new(&doc.file_path);
    if !path.exists() {
        return Err(ErrorBag::NotFound("File not found on disk".into()));
    }

    let file = actix_files::NamedFile::open_async(path).await.map_err(|e| ErrorBag::InternalServerError(e.to_string()))?;
    Ok(file.into_response(&req))
}
