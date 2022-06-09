use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use crate::domain::{
    project::{self, load_projects, Project},
    runtime::execute_project,
    settings::AppSettings,
};

pub async fn serve_web_server(cfg: AppSettings) -> std::io::Result<()> {
    let cfg_clone = cfg.clone();
    let projects = load_projects(&cfg);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(cfg_clone.clone()))
            .app_data(web::Data::new(projects.clone()))
            .service(list_projects)
            .service(execute_project_tasks)
            .service(get_single_project)
    })
    .bind(&cfg.web.addr)?
    .run()
    .await
}

#[get("/api/projects")]
async fn list_projects(projects: web::Data<Vec<Project>>) -> impl Responder {
    // TODO: Require admin token!
    web::Json(projects)
}

#[get("/api/projects/{name}")]
async fn get_single_project(
    name: web::Path<String>,
    projects: web::Data<Vec<Project>>,
) -> impl Responder {
    let proj = projects
        .iter()
        .find(|p| p.codename == name.to_string())
        .cloned();

    web::Json(proj)
}

#[post("/api/projects/{name}/execute")]
async fn execute_project_tasks(
    name: web::Path<String>,
    projects: web::Data<Vec<Project>>,
) -> impl Responder {
    let proj = projects
        .iter()
        .find(|p| p.codename == name.to_string())
        .cloned();

    if proj.is_none() {
        return HttpResponse::NotFound();
    }

    match execute_project(&proj.unwrap()).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::BadRequest(),
    }
}
