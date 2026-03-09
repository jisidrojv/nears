use std::env;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    println!("cargo:rerun-if-changed=app");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("nears_routes_generated.rs");

    let mut routes = Vec::new();

    for entry in WalkDir::new("app").into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let configs = build_route_configs(path);
            routes.extend(configs);
        }
    }

    let generated_code = render_router_code(&routes);
    fs::write(&dest_path, generated_code).unwrap();
}

#[derive(Debug)]
struct RouteConfig {
    url: String,
    method: String,
    handler: String,
}

fn build_route_configs(path: &Path) -> Vec<RouteConfig> {
    let mut configs = Vec::new();
    let file_name = match path.file_name().and_then(|n| n.to_str()) {
        Some(name) => name,
        None => return configs,
    };

    if file_name != "page.rs" && file_name != "route.rs" {
        return configs;
    }

    let content = fs::read_to_string(path).unwrap_or_default();
    
    let mut url = path.parent().unwrap().to_string_lossy().to_string();
    url = url.replacen("app", "", 1);
    url = url.replace("\\", "/");
    
    // Convertir parámetros dinámicos manuales si existen
    url = url.replace("/id_editar", "/{id}/editar");
    url = url.replace("/id", "/{id}");
    
    // Ignorar (Route Groups) estilo Next.js: Eliminar todos los segmentos de ruta que comiencen por _
    // Ej: /_app_auth/cuenta -> /cuenta
    let segments: Vec<&str> = url.split('/').collect();
    let clean_segments: Vec<&str> = segments.into_iter()
        .filter(|v| !v.starts_with('_'))
        .collect();
    url = clean_segments.join("/");
    
    if url.is_empty() {
        url = "/".to_string();
    }
    
    let module_path = path.with_extension("");
    let module_str = module_path.to_string_lossy().replace("\\", "/").replace("/", "::");
    let base_module = format!("crate::{}", module_str);

    if file_name == "page.rs" && content.contains("pub async fn render") {
        configs.push(RouteConfig {
            url,
            method: "get".to_string(),
            handler: format!("{}::render", base_module),
        });
        return configs;
    }

    if file_name == "route.rs" {
        for method in ["get", "post", "put", "delete"] {
            let fn_signature = format!("pub async fn {}", method);
            if content.contains(&fn_signature) {
                configs.push(RouteConfig {
                    url: url.clone(),
                    method: method.to_string(),
                    handler: format!("{}::{}", base_module, method),
                });
            }
        }
    }

    configs
}

fn render_router_code(routes: &[RouteConfig]) -> String {
    let mut code = String::new();
    
    code.push_str("use axum::Router;\n");
    code.push_str("use axum::routing::{get, post, put, delete};\n\n");
    
    code.push_str("pub fn auto_router() -> Router {\n");
    code.push_str("    Router::new()\n");

    for route in routes {
        code.push_str(&format!(
            "        .route(\"{}\", {}({}))\n",
            route.url, route.method, route.handler
        ));
    }

    code.push_str("}\n");
    
    code
}
