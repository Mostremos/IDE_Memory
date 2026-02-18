//! Implementación del servidor MCP (Model Context Protocol)
//! 
//! Maneja la comunicación con IDEs usando el protocolo MCP estándar.

use crate::memory::{Memory, KnowledgeType};
use crate::metrics::Metrics;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{self, BufRead, BufReader, Write};
use std::time::Instant;

#[derive(Debug, Serialize, Deserialize)]
struct MCPRequest {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MCPResponse {
    jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<MCPError>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MCPError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Tool {
    name: String,
    description: String,
    input_schema: Value,
}

pub async fn run_stdio_server(memory: Memory, metrics: Option<Metrics>) -> anyhow::Result<()> {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut stdout = io::stdout();

    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => break, // EOF
            Ok(_) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                let start_time = Instant::now();
                let request_line = line.to_string();

                // Extraer id de la solicitud para poder responder con error si es necesario
                let request_id: Option<Value> = serde_json::from_str::<serde_json::Value>(&request_line)
                    .ok()
                    .and_then(|v| v.get("id").cloned())
                    .and_then(|v| if v.is_null() { None } else { Some(v) });

                let result = handle_request(&memory, &request_line, &metrics).await;
                let response_time = start_time.elapsed();

                match result {
                    Ok(Some(response)) => {
                        let response_json = serde_json::to_string(&response)?;
                        let response_size = response_json.len();
                        
                        // Registrar métricas
                        if let Some(ref m) = metrics {
                            let tool_name = extract_tool_name(&request_line);
                            let _ = m.record_request(
                                "mcp_request",
                                tool_name.as_deref(),
                                response_time,
                                response_size,
                                true,
                                None,
                            );
                        }

                        writeln!(stdout, "{}", response_json)?;
                        stdout.flush()?;
                    }
                    Ok(None) => {
                        // No response needed (notifications)
                        if let Some(ref m) = metrics {
                            let tool_name = extract_tool_name(&request_line);
                            let _ = m.record_request(
                                "mcp_notification",
                                tool_name.as_deref(),
                                response_time,
                                0,
                                true,
                                None,
                            );
                        }
                    }
                    Err(e) => {
                        // Si hay un id, debemos enviar una respuesta de error JSON-RPC
                        if let Some(id) = request_id {
                            let error_response = MCPResponse {
                                jsonrpc: "2.0".to_string(),
                                id: Some(id),
                                result: None,
                                error: Some(MCPError {
                                    code: -32700,
                                    message: format!("Error procesando solicitud: {}", e),
                                    data: None,
                                }),
                            };
                            let response_json = serde_json::to_string(&error_response)?;
                            
                            // Registrar error en métricas
                            if let Some(ref m) = metrics {
                                let tool_name = extract_tool_name(&request_line);
                                let _ = m.record_request(
                                    "mcp_request",
                                    tool_name.as_deref(),
                                    response_time,
                                    response_json.len(),
                                    false,
                                    Some(&e.to_string()),
                                );
                            }
                            
                            writeln!(stdout, "{}", response_json)?;
                            stdout.flush()?;
                        } else {
                            // No hay id, es una notificación - solo registrar métricas
                            if let Some(ref m) = metrics {
                                let tool_name = extract_tool_name(&request_line);
                                let _ = m.record_request(
                                    "mcp_notification",
                                    tool_name.as_deref(),
                                    response_time,
                                    0,
                                    false,
                                    Some(&e.to_string()),
                                );
                            }
                        }
                    }
                }
            }
            Err(_e) => {
                // Error fatal - solo en este caso escribir a stderr
                // Pero en producción, mejor salir silenciosamente
                // eprintln!("Error leyendo stdin: {}", _e);
                break;
            }
        }
    }

    Ok(())
}

fn extract_tool_name(request_json: &str) -> Option<String> {
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(request_json) {
        if let Some(params) = value.get("params") {
            if let Some(name) = params.get("name") {
                return name.as_str().map(|s| s.to_string());
            }
        }
    }
    None
}

async fn handle_request(
    memory: &Memory,
    request_json: &str,
    _metrics: &Option<Metrics>,
) -> anyhow::Result<Option<MCPResponse>> {
    let request: MCPRequest = serde_json::from_str(request_json)?;
    
    // Normalizar id: si es null o None, tratarlo como notificación
    let id = request.id.and_then(|v| {
        if v.is_null() {
            None
        } else {
            Some(v)
        }
    });
    
    // Si no hay id válido, es una notificación y no debemos responder
    if id.is_none() {
        return Ok(None);
    }

    let result = match request.method.as_str() {
        "initialize" => {
            serde_json::json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": "IDE_Memory",
                    "version": "0.1.0"
                }
            })
        }
        "tools/list" => {
            let tools = vec![
                Tool {
                    name: "mem_search".to_string(),
                    description: "Busca conocimiento relevante usando búsqueda compacta (Progressive Disclosure Capa 1)".to_string(),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "query": {
                                "type": "string",
                                "description": "Consulta de búsqueda"
                            },
                            "limit": {
                                "type": "integer",
                                "description": "Número máximo de resultados",
                                "default": 5
                            }
                        },
                        "required": ["query"]
                    }),
                },
                Tool {
                    name: "mem_save".to_string(),
                    description: "Guarda una entrada de conocimiento en la memoria persistente".to_string(),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "knowledge_type": {
                                "type": "string",
                                "enum": ["decision", "bugfix", "pattern", "config", "context", "summary"],
                                "description": "Tipo de conocimiento"
                            },
                            "title": {
                                "type": "string",
                                "description": "Título de la entrada"
                            },
                            "content": {
                                "type": "string",
                                "description": "Contenido completo"
                            },
                            "summary": {
                                "type": "string",
                                "description": "Resumen compacto (~100 tokens)"
                            },
                            "tags": {
                                "type": "array",
                                "items": {"type": "string"},
                                "description": "Tags para categorización"
                            },
                            "project_path": {
                                "type": "string",
                                "description": "Ruta del proyecto (opcional)"
                            }
                        },
                        "required": ["knowledge_type", "title", "content", "summary"]
                    }),
                },
                Tool {
                    name: "mem_get_detail".to_string(),
                    description: "Obtiene el detalle completo de una entrada (Progressive Disclosure Capa 3)".to_string(),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "id": {
                                "type": "integer",
                                "description": "ID de la entrada de conocimiento"
                            }
                        },
                        "required": ["id"]
                    }),
                },
                Tool {
                    name: "mem_timeline".to_string(),
                    description: "Obtiene el timeline de una entrada (Progressive Disclosure Capa 2)".to_string(),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "id": {
                                "type": "integer",
                                "description": "ID de la entrada de conocimiento"
                            }
                        },
                        "required": ["id"]
                    }),
                },
            ];
            serde_json::json!({
                "tools": tools
            })
        }
        "tools/call" => {
            if let Some(params) = request.params {
                match handle_tool_call(memory, params).await {
                    Ok(result) => result,
                    Err(e) => {
                        // Convertir error en respuesta JSON-RPC con error
                        return Ok(Some(MCPResponse {
                            jsonrpc: "2.0".to_string(),
                            id: id.clone(),
                            result: None,
                            error: Some(MCPError {
                                code: -32603,
                                message: format!("Error interno: {}", e),
                                data: None,
                            }),
                        }));
                    }
                }
            } else {
                return Ok(Some(MCPResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: None,
                    error: Some(MCPError {
                        code: -32602,
                        message: "Faltan parámetros para tools/call".to_string(),
                        data: None,
                    }),
                }));
            }
        }
        _ => {
            return Ok(Some(MCPResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: None,
                error: Some(MCPError {
                    code: -32601,
                    message: format!("Método no encontrado: {}", request.method),
                    data: None,
                }),
            }));
        }
    };

    Ok(Some(MCPResponse {
        jsonrpc: "2.0".to_string(),
        id,
        result: Some(result),
        error: None,
    }))
}

async fn handle_tool_call(memory: &Memory, params: Value) -> anyhow::Result<Value> {
    let name = params
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Falta 'name' en parámetros"))?;

    let arguments = params
        .get("arguments")
        .ok_or_else(|| anyhow::anyhow!("Faltan 'arguments' en parámetros"))?;

    match name {
        "mem_search" => {
            let query = arguments
                .get("query")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("Falta 'query' en arguments"))?;
            let limit = arguments
                .get("limit")
                .and_then(|v| v.as_i64())
                .unwrap_or(5) as i32;

            let results = memory.search_compact(query, limit)?;
            Ok(serde_json::to_value(results)?)
        }
        "mem_save" => {
            let knowledge_type_str = arguments
                .get("knowledge_type")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("Falta 'knowledge_type' en arguments"))?;
            let knowledge_type = KnowledgeType::from_str(knowledge_type_str)
                .ok_or_else(|| anyhow::anyhow!("Tipo de conocimiento inválido: {}", knowledge_type_str))?;

            let title = arguments
                .get("title")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("Falta 'title' en arguments"))?;
            let content = arguments
                .get("content")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("Falta 'content' en arguments"))?;
            let summary = arguments
                .get("summary")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("Falta 'summary' en arguments"))?;

            let tags: Vec<String> = arguments
                .get("tags")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default();

            let project_path = arguments
                .get("project_path")
                .and_then(|v| v.as_str());

            let id = memory.save_knowledge(
                knowledge_type,
                title,
                content,
                summary,
                &tags,
                project_path,
            )?;

            Ok(serde_json::json!({
                "id": id,
                "success": true
            }))
        }
        "mem_get_detail" => {
            let id = arguments
                .get("id")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow::anyhow!("Falta 'id' en arguments"))?;

            match memory.get_detail(id)? {
                Some(entry) => Ok(serde_json::to_value(entry)?),
                None => Err(anyhow::anyhow!("Entrada no encontrada: {}", id)),
            }
        }
        "mem_timeline" => {
            let id = arguments
                .get("id")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow::anyhow!("Falta 'id' en arguments"))?;

            let timeline = memory.get_timeline(id)?;
            Ok(serde_json::to_value(timeline)?)
        }
        _ => Err(anyhow::anyhow!("Herramienta desconocida: {}", name)),
    }
}

pub async fn run_http_server(_memory: Memory, _port: u16, _metrics: Option<Metrics>) -> anyhow::Result<()> {
    // TODO: Implementar servidor HTTP para MCP
    anyhow::bail!("Modo HTTP aún no implementado. Use 'stdio' por ahora.");
}
