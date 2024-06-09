use crate::error::InternalError;
use chrono::Datelike;
use hijri_date::HijriDate;
use lopdf::{dictionary, Stream};
use models::penarikan::PenarikanJoined;
use models::permohonan::PermohonanJoined;
use serde::Serialize;
use simple_pdf_generator::PrintOptions;
use simple_pdf_generator_derive::PdfTemplate;
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;
use tokio::io::AsyncReadExt;
use tracing::debug;
use warp::{reject, Rejection};

pub async fn example_pdf() -> Result<Vec<u8>, Rejection> {
    let mut file = tokio::fs::File::open("assets/pdf/example.pdf")
        .await
        .map_err(|e| {
            reject::custom(InternalError::FilesystemError(
                "error opening example pdf".to_owned(),
            ))
        })?;

    let mut buffer = vec![];
    file.read_to_end(&mut buffer).await.map_err(|e| {
        reject::custom(InternalError::FilesystemError(
            "error opening example pdf".to_owned(),
        ))
    })?;

    Ok(buffer)
}

pub fn gen_genpdf(detail: &PermohonanJoined) -> Result<Vec<u8>, Rejection> {
    let font_family =
        genpdf::fonts::from_files("assets/fonts/ubuntu", "Ubuntu", None).map_err(|e| {
            InternalError::DatabaseError(format!(
                "error importing fonts while generating pdf: {}",
                e.to_string()
            ))
        })?;
    let mut doc = genpdf::Document::new(font_family);
    doc.set_title(format!("Surat Permohonan {}", detail.company.name));
    doc.push(genpdf::elements::Paragraph::new("Hello, PDF!"));
    doc.push(genpdf::elements::Paragraph::new("- Le Rustacean"));

    let mut buffer = WrappedVecU8::new();
    let status = doc.render(&mut buffer);

    debug!("{:?}", buffer.vec.clone().borrow());
    if let Err(e) = status {
        debug!("pdf error: {:?}", e.kind());
        return Err(reject::custom(InternalError::NotImplemented(e.to_string())));
    }

    Ok(buffer.vec.take())
}

pub fn gen_lopdf(detail: &PermohonanJoined) -> Result<Vec<u8>, Rejection> {
    // with_version specifes the PDF version this document complies with.
    let mut doc = lopdf::Document::with_version("1.7");
    // lopdf::Object IDs are used for cross referencing in PDF documents. `lopdf` helps keep track of them
    // for us. They are simple integers.
    // Calls to `doc.new_object_id` and `doc.add_object` return an object id

    // pages is the root node of the page tree
    let pages_id = doc.new_object_id();

    // fonts are dictionaries. The type, subtype and basefont tags
    // are straight out of the PDF reference manual
    //
    // The dictionary macro is a helper that allows complex
    // key, value relationships to be represented in a simpler
    // visual manner, similar to a match statement.
    // Dictionary is linkedHashMap of byte vector, and object
    let font_id = doc.add_object(dictionary! {
        // type of dictionary
        "Type" => "Font",
        // type of font, type1 is simple postscript font
        "Subtype" => "Type1",
        // basefont is postscript name of font for type1 font.
        // See PDF reference document for more details
        "BaseFont" => "Courier",
    });

    // font dictionaries need to be added into resource dictionaries
    // in order to be used.
    // Resource dictionaries can contain more than just fonts,
    // but normally just contains fonts
    // Only one resource dictionary is allowed per page tree root
    let resources_id = doc.add_object(dictionary! {
        // fonts are actually triplely nested dictionaries. Fun!
        "Font" => dictionary! {
            // F1 is the font name used when writing text.
            // It must be unique in the document. It does not
            // have to be F1
            "F1" => font_id,
        },
    });

    // lopdf::Content is a wrapper struct around an operations struct that contains a vector of operations
    // The operations struct contains a vector of operations that match up with a particular PDF
    // operator and operands.
    // Reference the PDF reference for more details on these operators and operands.
    // Note, the operators and operands are specified in a reverse order than they
    // actually appear in the PDF file itself.
    let content = lopdf::content::Content {
        operations: vec![
            // BT begins a text element. it takes no operands
            lopdf::content::Operation::new("BT", vec![]),
            // Tf specifies the font and font size. Font scaling is complicated in PDFs. Reference
            // the reference for more info.
            // The into() methods are defined based on their paired .from() methods (this
            // functionality is built into rust), and are converting the provided values into
            // An enum that represents the basic object types in PDF documents.
            lopdf::content::Operation::new("Tf", vec!["F1".into(), 48.into()]),
            // Td adjusts the translation components of the text matrix. When used for the first
            // time after BT, it sets the initial text position on the page.
            // Note: PDF documents have Y=0 at the bottom. Thus 600 to print text near the top.
            lopdf::content::Operation::new("Td", vec![100.into(), 600.into()]),
            // Tj prints a string literal to the page. By default, this is black text that is
            // filled in. There are other operators that can produce various textual effects and
            // colors
            lopdf::content::Operation::new(
                "Tj",
                vec![lopdf::Object::string_literal(detail.company.name.clone())],
            ),
            // ET ends the text element
            lopdf::content::Operation::new("ET", vec![]),
        ],
    };

    let students = detail.students.iter().fold(String::new(), |mut acc, e| {
        if acc.len() == 0 {
            &acc.push_str(&e.name.clone());
            acc
        } else {
            &acc.push_str("\n");
            &acc.push_str(&e.name.clone());
            acc
        }
    });
    let content1 = lopdf::content::Content {
        operations: vec![
            // BT begins a text element. it takes no operands
            lopdf::content::Operation::new("BT", vec![]),
            // Tf specifies the font and font size. Font scaling is complicated in PDFs. Reference
            // the reference for more info.
            // The into() methods are defined based on their paired .from() methods (this
            // functionality is built into rust), and are converting the provided values into
            // An enum that represents the basic object types in PDF documents.
            lopdf::content::Operation::new("Tf", vec!["F1".into(), 28.into()]),
            // Td adjusts the translation components of the text matrix. When used for the first
            // time after BT, it sets the initial text position on the page.
            // Note: PDF documents have Y=0 at the bottom. Thus 600 to print text near the top.
            lopdf::content::Operation::new("Td", vec![50.into(), 600.into()]),
            // Tj prints a string literal to the page. By default, this is black text that is
            // filled in. There are other operators that can produce various textual effects and
            // colors
            lopdf::content::Operation::new("Tj", vec![lopdf::Object::string_literal(students)]),
            // ET ends the text element
            lopdf::content::Operation::new("ET", vec![]),
        ],
    };

    // Streams are a dictionary followed by a sequence of bytes. What that sequence of bytes
    // represents depends on context
    // The stream dictionary is set internally to lopdf and normally doesn't
    // need to be manually manipulated. It contains keys such as
    // Length, Filter, DecodeParams, etc
    //
    // content is a stream of encoded content data.
    let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode().unwrap()));
    let content_id1 = doc.add_object(Stream::new(dictionary! {}, content1.encode().unwrap()));

    // Page is a dictionary that represents one page of a PDF file.
    // It has a type, parent and contents
    let page_id = doc.add_object(dictionary! {
        "Type" => "Page",
        "Parent" => pages_id,
        "Contents" => content_id,
    });
    let page_id1 = doc.add_object(dictionary! {
        "Type" => "Page",
        "Parent" => pages_id,
        "Contents" => content_id1,
    });

    // Again, pages is the root of the page tree. The ID was already created
    // at the top of the page, since we needed it to assign to the parent element of the page
    // dictionary
    //
    // This is just the basic requirements for a page tree root object. There are also many
    // additional entries that can be added to the dictionary if needed. Some of these can also be
    // defined on the page dictionary itself, and not inherited from the page tree root.
    let pages = dictionary! {
        // Type of dictionary
        "Type" => "Pages",
        // Vector of page IDs in document. Normally would contain more than one ID and be produced
        // using a loop of some kind
        "Kids" => vec![page_id.into(), page_id1.into()],
        // Page count
        "Count" => 2,
        // ID of resources dictionary, defined earlier
        "Resources" => resources_id,
        // a rectangle that defines the boundaries of the physical or digital media. This is the
        // "Page Size"
        "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
    };

    // using insert() here, instead of add_object() since the id is already known.
    doc.objects
        .insert(pages_id, lopdf::Object::Dictionary(pages));

    // Creating document catalog.
    // There are many more entries allowed in the catalog dictionary.
    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });

    // Root key in trailer is set here to ID of document catalog,
    // remainder of trailer is set during doc.save().
    doc.trailer.set("Root", catalog_id);
    doc.compress();

    let mut buffer = WrappedVecU8::new();

    doc.save_to(&mut buffer)
        .map_err(|e| reject::custom(InternalError::NotImplemented(e.to_string())))?;

    Ok(buffer.vec.take())
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
