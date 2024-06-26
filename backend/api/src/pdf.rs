use crate::error::InternalError;
use chrono::format::StrftimeItems;
use chrono::{Datelike, Locale};
use hijri_date::HijriDate;
use models::penarikan::PenarikanJoined;
use models::pengantaran::PengantaranJoined;
use models::permohonan::PermohonanJoined;
use serde::Serialize;
use simple_pdf_generator::PrintOptions;
use simple_pdf_generator_derive::PdfTemplate;
use std::cell::RefCell;
use std::error::Error;
use std::io::Write;
use std::rc::Rc;
use std::sync::OnceLock;
use tera::Tera;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::debug;
use warp::{reject, Rejection};

fn tera() -> &'static Tera {
    static TERA: OnceLock<Tera> = OnceLock::new();
    TERA.get_or_init(|| match Tera::new("assets/templates/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            std::process::exit(1);
        }
    })
}

struct WrappedVecU8 {
    vec: Rc<RefCell<Vec<u8>>>,
}

impl WrappedVecU8 {
    pub fn new() -> Self {
        WrappedVecU8 {
            vec: Rc::new(RefCell::new(Vec::new())),
        }
    }
}

impl Write for WrappedVecU8 {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        <WrappedVecU8 as Write>::write(self, buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        <WrappedVecU8 as Write>::flush(self)
    }
}

mod permohonan {
    use crate::pdf::StudentTableData;
    use simple_pdf_generator_derive::{PdfTemplate, PdfTemplateForHtml};

    #[derive(PdfTemplateForHtml)]
    pub struct PermohonanPdf {}
}

mod pengantaran {
    use crate::pdf::StudentTableData;
    use simple_pdf_generator_derive::{PdfTemplate, PdfTemplateForHtml};

    #[derive(PdfTemplateForHtml)]
    pub struct PengantaranPdf {}
}

mod penarikan {
    use crate::pdf::StudentTableData;
    use simple_pdf_generator_derive::{PdfTemplate, PdfTemplateForHtml};

    #[derive(PdfTemplateForHtml)]
    pub struct PenarikanPdf {}
}

#[derive(Serialize)]
struct StudentTableData {
    index: u8,
    student_name: String,
    student_class: String,
}

pub async fn gen_penarikan_chromium(detail: &PenarikanJoined) -> Result<Vec<u8>, Rejection> {
    let hijri = {
        let date = HijriDate::from_gr(
            detail.created_at.year() as usize,
            detail.created_at.month() as usize,
            detail.created_at.day() as usize,
        )
        .map_err(|e| {
            reject::custom(InternalError::NotImplemented(format!(
                "error generating hijri date: {}",
                e.to_string()
            )))
        })?;
        let h_date = date.day();
        let h_year = date.year();
        let h_month = match date.month() {
            1 => "Muharam",
            2 => "Safar",
            3 => "Rabiul Awal",
            4 => "Rabiul Akhir",
            5 => "Jumadil Awal",
            6 => "Jumadil Akhir",
            7 => "Rajab",
            8 => "Sya'ban",
            9 => "Ramadan",
            10 => "Syawal",
            11 => "Dzulkaidah",
            _ => "Dzulhijjah",
        };

        format!("{h_date} {h_month} {h_year} H")
    };

    let mut context = tera::Context::new();
    // TODO: Use real attachment number
    context.insert("nomor_lampiran", &1);
    context.insert("month", &detail.created_at.month().to_string());
    context.insert("year", &detail.created_at.year().to_string());
    // TODO: Convert hijri date in arabic letters to latin
    context.insert("hijriah", &hijri);
    context.insert(
        "georgian",
        &detail
            .created_at
            .format_localized("%e %B %Y", Locale::id_ID)
            .to_string(),
    );
    context.insert("company", &detail.company.name);
    context.insert("company_address", &detail.company.address);
    // TODO: Use real school year
    context.insert("school_year", &"2024/2025".to_string());
    context.insert(
        "end_date",
        &detail
            .end_date
            .format_localized("%e %B %Y", Locale::id_ID)
            .to_string(),
    );
    context.insert("signature_title_1", &"Kepala Sekolah".to_string());
    context.insert("signature_title_2", &"Wakil Sekolah".to_string());
    context.insert("signature_name_1", &"AGAM AMINTAHA, S.Kom".to_string());
    context.insert("signature_name_2", &"H. ABDUL FATIQ, M.Kom".to_string());
    context.insert("students", &detail.students);

    let html = tera().render("penarikan.html", &context).map_err(|e| {
        let kind = &e.kind;
        let source = &e.source();
        debug!("{kind:#?}");
        debug!("{source:#?}");
        reject::custom(InternalError::PdfError(format!("{e}")))
    })?;

    let file = tokio::fs::File::create(format!(
        "assets/pdf/{}.html",
        chrono::Local::now().to_string()
    ))
    .await
    .ok();
    if let Some(mut v) = file {
        v.write_all(&html.as_bytes()).await.ok();
    }

    let letter = penarikan::PenarikanPdf {};

    let pdf_buf = letter
        .generate_pdf_from_html(
            html,
            &[],
            &PrintOptions {
                print_background: false,
                paper_width: Some(210.),
                paper_height: Some(297.),
                margin_top: Some(14.986),
                margin_bottom: Some(22.606),
                margin_left: Some(25.4),
                margin_right: Some(25.4),
                page_ranges: None,
                prefer_css_page_size: false,
                landscape: false,
            },
        )
        .await
        .map_err(|e| reject::custom(InternalError::PdfError(e.to_string())))?;

    Ok(pdf_buf)
}

pub async fn gen_permohonan_chromium(detail: &PermohonanJoined) -> Result<Vec<u8>, Rejection> {
    let hijri = {
        let date = HijriDate::from_gr(
            detail.created_at.year() as usize,
            detail.created_at.month() as usize,
            detail.created_at.day() as usize,
        )
        .map_err(|e| {
            reject::custom(InternalError::NotImplemented(format!(
                "error generating hijri date: {}",
                e.to_string()
            )))
        })?;
        let h_date = date.day();
        let h_year = date.year();
        let h_month = match date.month() {
            1 => "Muharam",
            2 => "Safar",
            3 => "Rabiul Awal",
            4 => "Rabiul Akhir",
            5 => "Jumadil Awal",
            6 => "Jumadil Akhir",
            7 => "Rajab",
            8 => "Sya'ban",
            9 => "Ramadan",
            10 => "Syawal",
            11 => "Dzulkaidah",
            _ => "Dzulhijjah",
        };

        format!("{h_date} {h_month} {h_year} H")
    };

    let mut context = tera::Context::new();
    // TODO: Use real attachment number
    context.insert("nomor_lampiran", &1);
    context.insert("month", &detail.created_at.month().to_string());
    context.insert("year", &detail.created_at.year().to_string());
    // TODO: Convert hijri date in arabic letters to latin
    context.insert("hijriah", &hijri);
    context.insert(
        "georgian",
        &detail
            .created_at
            .format_localized("%e %B %Y", Locale::id_ID)
            .to_string(),
    );
    context.insert("company", &detail.company.name);
    context.insert("company_address", &detail.company.address);
    // TODO: Use real school year
    context.insert("school_year", &"2024/2025".to_string());
    context.insert(
        "start_date",
        &detail
            .start_date
            .format_localized("%e %B %Y", Locale::id_ID)
            .to_string(),
    );
    context.insert(
        "end_date",
        &detail
            .end_date
            .format_localized("%e %B %Y", Locale::id_ID)
            .to_string(),
    );
    context.insert("signature_title_1", &"Kepala Sekolah".to_string());
    context.insert("signature_title_2", &"Wakil Sekolah".to_string());
    context.insert("signature_name_1", &"AGAM AMINTAHA, S.Kom".to_string());
    context.insert("signature_name_2", &"H. ABDUL FATIQ, M.Kom".to_string());
    context.insert("students", &detail.students);

    let html = tera().render("permohonan.html", &context).map_err(|e| {
        let kind = &e.kind;
        let source = &e.source();
        debug!("{kind:#?}");
        debug!("{source:#?}");
        reject::custom(InternalError::PdfError(format!("{e}")))
    })?;

    let file = tokio::fs::File::create(format!(
        "assets/pdf/{}.html",
        chrono::Local::now().to_string()
    ))
    .await
    .ok();
    if let Some(mut v) = file {
        v.write_all(&html.as_bytes()).await.ok();
    }

    let letter = permohonan::PermohonanPdf {};

    let pdf_buf = letter
        .generate_pdf_from_html(
            html,
            &[],
            &PrintOptions {
                print_background: false,
                paper_width: Some(210.),
                paper_height: Some(297.),
                margin_top: Some(14.986),
                margin_bottom: Some(22.606),
                margin_left: Some(25.4),
                margin_right: Some(25.4),
                page_ranges: None,
                prefer_css_page_size: false,
                landscape: false,
            },
        )
        .await
        .map_err(|e| reject::custom(InternalError::PdfError(e.to_string())))?;

    Ok(pdf_buf)
}

pub async fn gen_pengantaran_chromium(detail: &PengantaranJoined) -> Result<Vec<u8>, Rejection> {
    let hijri = {
        let date = HijriDate::from_gr(
            detail.created_at.year() as usize,
            detail.created_at.month() as usize,
            detail.created_at.day() as usize,
        )
        .map_err(|e| {
            reject::custom(InternalError::NotImplemented(format!(
                "error generating hijri date: {}",
                e.to_string()
            )))
        })?;
        let h_date = date.day();
        let h_year = date.year();
        let h_month = match date.month() {
            1 => "Muharam",
            2 => "Safar",
            3 => "Rabiul Awal",
            4 => "Rabiul Akhir",
            5 => "Jumadil Awal",
            6 => "Jumadil Akhir",
            7 => "Rajab",
            8 => "Sya'ban",
            9 => "Ramadan",
            10 => "Syawal",
            11 => "Dzulkaidah",
            _ => "Dzulhijjah",
        };

        format!("{h_date} {h_month} {h_year} H")
    };

    let mut context = tera::Context::new();
    // TODO: Use real attachment number
    context.insert("nomor_lampiran", &1);
    context.insert("month", &detail.created_at.month().to_string());
    context.insert("year", &detail.created_at.year().to_string());
    // TODO: Convert hijri date in arabic letters to latin
    context.insert("hijriah", &hijri);
    context.insert(
        "georgian",
        &detail
            .created_at
            .format_localized("%e %B %Y", Locale::id_ID)
            .to_string(),
    );
    context.insert("company", &detail.company.name);
    context.insert("company_address", &detail.company.address);
    // TODO: Use real school year
    context.insert("school_year", &"2024/2025".to_string());
    context.insert(
        "start_date",
        &detail
            .start_date
            .format_localized("%e %B %Y", Locale::id_ID)
            .to_string(),
    );
    context.insert(
        "end_date",
        &detail
            .end_date
            .format_localized("%e %B %Y", Locale::id_ID)
            .to_string(),
    );
    context.insert("signature_title_1", &"Kepala Sekolah".to_string());
    context.insert("signature_title_2", &"Wakil Sekolah".to_string());
    context.insert("signature_name_1", &"AGAM AMINTAHA, S.Kom".to_string());
    context.insert("signature_name_2", &"H. ABDUL FATIQ, M.Kom".to_string());
    context.insert("students", &detail.students);

    let html = tera().render("pengantaran.html", &context).map_err(|e| {
        let kind = &e.kind;
        let source = &e.source();
        debug!("{kind:#?}");
        debug!("{source:#?}");
        reject::custom(InternalError::PdfError(format!("{e}")))
    })?;

    let file = tokio::fs::File::create(format!(
        "assets/pdf/{}.html",
        chrono::Local::now().to_string()
    ))
    .await
    .ok();
    if let Some(mut v) = file {
        v.write_all(&html.as_bytes()).await.ok();
    }

    let letter = pengantaran::PengantaranPdf {};

    let pdf_buf = letter
        .generate_pdf_from_html(
            html,
            &[],
            &PrintOptions {
                print_background: false,
                paper_width: Some(210.),
                paper_height: Some(297.),
                margin_top: Some(14.986),
                margin_bottom: Some(22.606),
                margin_left: Some(25.4),
                margin_right: Some(25.4),
                page_ranges: None,
                prefer_css_page_size: false,
                landscape: false,
            },
        )
        .await
        .map_err(|e| reject::custom(InternalError::PdfError(e.to_string())))?;

    Ok(pdf_buf)
}
