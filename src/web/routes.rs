use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

use crate::{
    domain::{project::Project, runtime::execute_project},
    web::auth::require_project_auth,
};

use super::server::RequestCtx;

#[get("/api/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(HealthDto {
        status: "ok".into(),
    })
}

#[get("/api/projects")]
async fn list_projects(ctx: web::Data<RequestCtx>) -> impl Responder {
    // TODO: Require admin token!
    HttpResponse::Ok().json(
        ctx.projects()
            .iter()
            .map(ProjectDto::from)
            .collect::<Vec<ProjectDto>>(),
    )
}

#[get("/api/projects/{name}")]
async fn get_single_project(name: web::Path<String>, ctx: web::Data<RequestCtx>) -> impl Responder {
    // TODO: Require admin token!

    match ctx.get_project(&name).map(ProjectDto::from) {
        Some(proj) => HttpResponse::Ok().json(proj),
        None => HttpResponse::NotFound().json(ErrorDto {
            error: "not_found".into(),
            error_description: "Unable to find project".into(),
        }),
    }
}

#[post("/api/projects/{name}/execute")]
async fn execute_project_tasks(
    name: web::Path<String>,
    ctx: web::Data<RequestCtx>,
    req: HttpRequest,
) -> impl Responder {
    let res = require_project_auth(&ctx, &req, &name);
    if res.is_err() {
        return HttpResponse::Unauthorized().json(ErrorDto {
            error: "unathorized".into(),
            error_description: "Invalid credentials provided".into(),
        });
    }

    match ctx.get_project(&name) {
        Some(proj) => do_execute(proj).await,

        None => HttpResponse::NotFound().json(ErrorDto {
            error: "not_found".into(),
            error_description: "Unable to find project".into(),
        }),
    }
}

async fn do_execute(project: &Project) -> HttpResponse {
    match execute_project(project).await {
        Ok(_) => HttpResponse::Ok().json(HealthDto {
            status: "ok".into(),
        }),
        Err(_) => HttpResponse::BadRequest().json(ErrorDto {
            error: "exec_fail".into(),
            error_description: "Execution failed".into(),
        }),
    }
}

#[derive(Serialize)]
struct ProjectDto {
    name: String,
    codename: String,
    desc: String,
    enabled: bool,
}

impl From<&Project> for ProjectDto {
    fn from(p: &Project) -> Self {
        Self {
            name: p.name.to_string(),
            codename: p.codename.to_string(),
            desc: p.desc.to_string(),
            enabled: p.enabled,
        }
    }
}

#[derive(Serialize)]
struct HealthDto {
    status: String,
}

#[derive(Serialize)]
struct ErrorDto {
    error: String,
    error_description: String,
}
