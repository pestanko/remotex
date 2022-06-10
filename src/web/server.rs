use actix_web::{web, App, HttpServer};

use crate::domain::{
    project,
    settings::AppSettings,
};

use super::routes;

pub async fn serve_web_server(cfg: AppSettings) -> std::io::Result<()> {
    let host = cfg.web.addr.to_string();

    let projects = project::load_projects(&cfg);

    let ctx = RequestCtx { cfg, projects };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(ctx.clone()))
            .service(routes::health)
            .service(routes::list_projects)
            .service(routes::execute_project_tasks)
            .service(routes::get_single_project)
    })
    .bind(&host)?
    .run()
    .await
}

#[derive(Debug, Clone)]
pub struct RequestCtx {
    cfg: AppSettings,
    projects: Vec<project::Project>,
}

impl RequestCtx {
    pub fn cfg(&self) -> &AppSettings {
        &self.cfg
    }

    pub fn projects(&self) -> &[project::Project] {
        &self.projects
    }

    pub fn get_project(&self, codename: &str) -> Option<&project::Project> {
        self.projects().iter().find(|p| p.codename == codename)
    }
}
