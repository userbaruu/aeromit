//! # Module Get Users Handler
//!
//! Module ini digunakan untuk ambil keseluruhan pengguna sebagai `handlers`.
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::pengguna::handlers::get_users::{...}
//! ```
use mongodb::Database;
use actix_session::Session;
use actix_web::{
    web,
    HttpResponse,
};
use crate::app::dto::UmpanBalik;
use crate::app::errors::AppErrors;
use crate::pengguna::{
    dto::DocProps,
    services::get_users,
};
use crate::app::permissions::UserPermissions;

/// # Fungsi all
///
/// Fungsi ini untuk menampilkan _response_ umpan balik hasil penambahan pengguna baru
/// saat mengunjungi _endpoint root_ `v1/pengguna`.
///
/// <br />
///
/// # Masukan
///
/// * `doc_props` - properti dokumen untuk kelola limit dan skip..
/// * `session` - Actix session
/// * `db` - mongodb Database type yang dishare melalui _application state_.
///
/// <br />
///
/// # Keluaran
///
/// * `Result<HttpResponse, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari kumpulan
/// `HttpResponse` dan _Enum_ `AppErrors`.
pub async fn all(
    doc_props: web::Query<DocProps>,
    session: Session,
    db: web::Data<Database>,
) -> Result<HttpResponse, AppErrors> {
    UserPermissions::is_admin(session, db.clone()).await?;

    let seluruh_pengguna = get_users::all(doc_props, db).await?;
    let res = UmpanBalik::new(
        true,
        "Pengguna berhasil ditampilkan",
        seluruh_pengguna
    );

    Ok(HttpResponse::Ok().json(res))
}
