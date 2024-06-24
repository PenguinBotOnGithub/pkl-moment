use crate::error::InternalError;
use chrono::Datelike;
use hijri_date::HijriDate;
use models::penarikan::PenarikanJoined;
use models::pengantaran::PengantaranJoined;
use models::permohonan::PermohonanJoined;
use serde::Serialize;
use simple_pdf_generator::PrintOptions;
use simple_pdf_generator_derive::PdfTemplate;
use std::cell::RefCell;
use std::rc::Rc;
use warp::{reject, Rejection};

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

impl std::io::Write for WrappedVecU8 {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.vec.clone().borrow_mut().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.vec.clone().borrow_mut().flush()
    }
}

mod permohonan {
    use crate::pdf::StudentTableData;
    use simple_pdf_generator_derive::PdfTemplate;

    #[derive(PdfTemplate)]
    pub struct PermohonanPdf {
        pub nomor_lampiran: u16,
        pub month: String,
        pub year: String,
        pub hijriah: String,
        pub georgian: String,
        pub company: String,
        pub company_address: String,
        pub school_year: String,
        pub start_date: String,
        pub end_date: String,
        pub signature_title_1: String,
        pub signature_title_2: String,
        pub signature_name_1: String,
        pub signature_name_2: String,
        #[PdfTableData]
        pub student_table: Vec<StudentTableData>,
    }
}

mod pengantaran {
    use crate::pdf::StudentTableData;
    use simple_pdf_generator_derive::PdfTemplate;

    #[derive(PdfTemplate)]
    pub struct PengantaranPdf {
        pub nomor_lampiran: u16,
        pub month: String,
        pub year: String,
        pub hijriah: String,
        pub georgian: String,
        pub company: String,
        pub company_address: String,
        pub school_year: String,
        pub start_date: String,
        pub end_date: String,
        pub signature_title_1: String,
        pub signature_title_2: String,
        pub signature_name_1: String,
        pub signature_name_2: String,
        #[PdfTableData]
        pub student_table: Vec<StudentTableData>,
    }
}

mod penarikan {
    use crate::pdf::StudentTableData;
    use simple_pdf_generator_derive::PdfTemplate;

    #[derive(PdfTemplate)]
    pub struct PenarikanPdf {
        pub nomor_lampiran: u16,
        pub month: String,
        pub year: String,
        pub hijriah: String,
        pub georgian: String,
        pub company: String,
        pub company_address: String,
        pub school_year: String,
        pub end_date: String,
        pub signature_title_1: String,
        pub signature_title_2: String,
        pub signature_name_1: String,
        pub signature_name_2: String,
        #[PdfTableData]
        pub student_table: Vec<StudentTableData>,
    }
}

#[derive(Serialize)]
struct StudentTableData {
    index: u8,
    student_name: String,
    student_class: String,
}

pub async fn gen_penarikan_chromium(detail: &PenarikanJoined) -> Result<Vec<u8>, Rejection> {
    let hijri = HijriDate::from_gr(
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

    let letter = penarikan::PenarikanPdf {
        nomor_lampiran: 1,
        month: detail.created_at.month().to_string(),
        year: detail.created_at.year().to_string(),
        hijriah: hijri.format("%d %M %Y"),
        georgian: detail.created_at.format("%e %B %Y").to_string(),
        company: detail.company.name.clone(),
        company_address: detail.company.address.clone(),
        school_year: "2023/2024".to_string(),
        end_date: detail.end_date.format("%e %B %Y").to_string(),
        signature_title_1: "Kepala Sekolah".to_string(),
        signature_title_2: "Wakil Sekolah".to_string(),
        signature_name_1: "AGAM AMINTAHA, S.Kom".to_string(),
        signature_name_2: "H. ABDUL FATIQ, M.Kom".to_string(),
        student_table: detail
            .students
            .iter()
            .enumerate()
            .map(|v| StudentTableData {
                index: (v.0 + 1) as u8,
                student_name: v.1.name.clone(),
                student_class: v.1.class.clone(),
            })
            .collect(),
    };

    let pdf_buf = letter
        .generate_pdf(
            "assets/templates/penarikan.html".into(),
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
    let hijri = HijriDate::from_gr(
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

    let letter = permohonan::PermohonanPdf {
        nomor_lampiran: 1,
        month: detail.created_at.month().to_string(),
        year: detail.created_at.year().to_string(),
        hijriah: hijri.format("%d %M %Y"),
        georgian: detail.created_at.format("%e %B %Y").to_string(),
        company: detail.company.name.clone(),
        company_address: detail.company.address.clone(),
        school_year: "2023/2024".to_string(),
        start_date: detail.start_date.format("%e %B %Y").to_string(),
        end_date: detail.end_date.format("%e %B %Y").to_string(),
        signature_title_1: "Kepala Sekolah".to_string(),
        signature_title_2: "Wakil Sekolah".to_string(),
        signature_name_1: "AGAM AMINTAHA, S.Kom".to_string(),
        signature_name_2: "H. ABDUL FATIQ, M.Kom".to_string(),
        student_table: detail
            .students
            .iter()
            .enumerate()
            .map(|v| StudentTableData {
                index: (v.0 + 1) as u8,
                student_name: v.1.name.clone(),
                student_class: v.1.class.clone(),
            })
            .collect(),
    };

    let pdf_buf = letter
        .generate_pdf(
            "assets/templates/permohonan.html".into(),
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
    let hijri = HijriDate::from_gr(
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

    let letter = pengantaran::PengantaranPdf {
        nomor_lampiran: 1,
        month: detail.created_at.month().to_string(),
        year: detail.created_at.year().to_string(),
        hijriah: hijri.format("%d %M %Y"),
        georgian: detail.created_at.format("%e %B %Y").to_string(),
        company: detail.company.name.clone(),
        company_address: detail.company.address.clone(),
        school_year: "2023/2024".to_string(),
        start_date: detail.start_date.format("%e %B %Y").to_string(),
        end_date: detail.end_date.format("%e %B %Y").to_string(),
        signature_title_1: "Kepala Sekolah".to_string(),
        signature_title_2: "Wakil Sekolah".to_string(),
        signature_name_1: "AGAM AMINTAHA, S.Kom".to_string(),
        signature_name_2: "H. ABDUL FATIQ, M.Kom".to_string(),
        student_table: detail
            .students
            .iter()
            .enumerate()
            .map(|v| StudentTableData {
                index: (v.0 + 1) as u8,
                student_name: v.1.name.clone(),
                student_class: v.1.class.clone(),
            })
            .collect(),
    };

    let pdf_buf = letter
        .generate_pdf(
            "assets/templates/pengantaran.html".into(),
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
