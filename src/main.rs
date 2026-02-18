//! Servidor MCP para Memoria Persistente en IDEs
//! 
//! Este servidor expone herramientas de memoria persistente usando el protocolo MCP
//! (Model Context Protocol), permitiendo que IDEs como Cursor, Claude Code, etc.
//! tengan acceso a memoria persistente entre sesiones.

mod memory;
mod mcp_server;
mod metrics;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "IDE_Memory")]
#[command(about = "MCP Server para memoria persistente en IDEs", long_about = None)]
struct Args {
    /// Ruta al archivo de base de datos SQLite
    #[arg(short, long, default_value = "memory.db")]
    database: PathBuf,

    /// Puerto para el servidor HTTP (si se usa transporte HTTP)
    #[arg(short, long, default_value_t = 3000)]
    port: u16,

    /// Modo de transporte: stdio (default) o http
    #[arg(short, long, default_value = "stdio")]
    transport: String,

    /// Habilitar métricas y logging
    #[arg(long, default_value_t = true)]
    metrics: bool,

    /// Ruta para base de datos de métricas (si es diferente de la principal)
    #[arg(long)]
    metrics_db: Option<PathBuf>,

    /// Mostrar estadísticas y salir
    #[arg(long)]
    stats: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    
    // Solo mostrar mensajes de inicio si NO estamos en modo stdio (para evitar confundir a Cursor)
    // En modo stdio, Cursor espera solo JSON-RPC en stdout, cualquier cosa en stderr puede causar errores
    if args.transport != "stdio" {
        eprintln!("🚀 Iniciando servidor MCP de memoria persistente...");
        eprintln!("📁 Base de datos: {:?}", args.database);
        eprintln!("🚇 Transporte: {}", args.transport);
    }

    // Inicializar módulo de memoria
    let memory_db = memory::Memory::new(&args.database)?;
    if args.transport != "stdio" {
        eprintln!("✅ Base de datos inicializada");
    }

    // Inicializar métricas si están habilitadas
    let metrics_db_path = args.metrics_db.unwrap_or_else(|| {
        // Usar la misma base de datos pero con sufijo _metrics
        let mut path = args.database.clone();
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("memory");
        path.set_file_name(format!("{}_metrics.db", stem));
        path
    });

    let metrics = if args.metrics {
        match metrics::Metrics::new(&metrics_db_path) {
            Ok(m) => {
                if args.transport != "stdio" {
                    eprintln!("📊 Métricas habilitadas: {:?}", metrics_db_path);
                }
                Some(m)
            }
            Err(e) => {
                if args.transport != "stdio" {
                    eprintln!("⚠️  Advertencia: No se pudo inicializar métricas: {}", e);
                }
                None
            }
        }
    } else {
        None
    };

    // Si se solicita solo estadísticas, mostrarlas y salir
    if args.stats {
        if let Some(ref m) = metrics {
            match m.get_server_stats() {
                Ok(stats) => {
                    println!("{}", serde_json::to_string_pretty(&stats)?);
                }
                Err(e) => {
                    eprintln!("Error obteniendo estadísticas: {}", e);
                }
            }
        } else {
            eprintln!("Las métricas no están habilitadas. Use --metrics para habilitarlas.");
        }
        return Ok(());
    }

    // En modo stdio, NO escribir a stderr para evitar confundir a Cursor
    // Cursor espera solo JSON-RPC válido en stdout

    // Iniciar servidor MCP
    match args.transport.as_str() {
        "stdio" => {
            // NO escribir a stderr en modo stdio - Cursor puede interpretarlo como error
            mcp_server::run_stdio_server(memory_db, metrics).await?;
        }
        "http" => {
            if args.transport != "stdio" {
                eprintln!("🌐 Modo HTTP: servidor en puerto {}", args.port);
            }
            mcp_server::run_http_server(memory_db, args.port, metrics).await?;
        }
        _ => {
            anyhow::bail!("Transporte no soportado: {}. Use 'stdio' o 'http'", args.transport);
        }
    }

    Ok(())
}
