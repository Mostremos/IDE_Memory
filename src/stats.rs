//! Ejecutable simple para mostrar métricas del servidor IDE Memory

mod metrics;

use clap::Parser;
use std::path::PathBuf;
use metrics::Metrics;

#[derive(Parser, Debug)]
#[command(name = "IDE_Memory_Stats")]
#[command(about = "Muestra estadísticas del servidor IDE Memory", long_about = None)]
struct Args {
    /// Ruta al archivo de base de datos de métricas
    #[arg(short, long, default_value = "ide_memory_metrics.db")]
    metrics_db: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    println!("📊 Estadísticas del Servidor IDE Memory");
    println!("========================================\n");

    // Inicializar métricas
    let metrics = match Metrics::new(&args.metrics_db) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("❌ Error: No se pudo abrir la base de datos de métricas: {}", e);
            eprintln!("   Ruta: {:?}", args.metrics_db);
            eprintln!("\n💡 Sugerencia: Asegúrate de que el servidor haya ejecutado al menos una vez.");
            std::process::exit(1);
        }
    };

    // Obtener estadísticas
    match metrics.get_server_stats() {
        Ok(stats) => {
            // Estadísticas generales
            println!("📈 Estadísticas Generales");
            println!("   Total de solicitudes: {}", stats.total_requests);
            
            let successful_requests = stats.total_requests - stats.total_errors;
            println!("   Solicitudes exitosas: {} ({:.1}%)", 
                successful_requests,
                if stats.total_requests > 0 {
                    (successful_requests as f64 / stats.total_requests as f64) * 100.0
                } else {
                    0.0
                }
            );
            println!("   Solicitudes con error: {} ({:.1}%)",
                stats.total_errors,
                if stats.total_requests > 0 {
                    (stats.total_errors as f64 / stats.total_requests as f64) * 100.0
                } else {
                    0.0
                }
            );
            println!("   Tiempo promedio de respuesta: {:.2} ms", stats.avg_response_time_ms);

            // Estadísticas por herramienta
            if !stats.tool_stats.is_empty() {
                println!("\n🔧 Estadísticas por Herramienta");
                println!("   {:<20} {:>10} {:>10} {:>10} {:>12}", 
                    "Herramienta", "Llamadas", "Éxitos", "Errores", "Tiempo (ms)");
                println!("   {}", "-".repeat(70));

                for tool in &stats.tool_stats {
                    println!("   {:<20} {:>10} {:>10} {:>10} {:>12.2}",
                        &tool.tool_name,
                        tool.total_calls,
                        tool.success_count,
                        tool.error_count,
                        tool.avg_response_time_ms
                    );
                }
            } else {
                println!("\n⚠️  No hay estadísticas de herramientas aún.");
            }

            // Información adicional
            println!("\n📁 Base de datos: {:?}", args.metrics_db);
            println!("\n💡 Para ver estadísticas en JSON, usa: IDE_Memory.exe --stats");
        }
        Err(e) => {
            eprintln!("❌ Error obteniendo estadísticas: {}", e);
            std::process::exit(1);
        }
    }

    // Pausar al final para mantener la ventana abierta (útil cuando se ejecuta desde doble clic)
    // Solo pausar si hay una consola disponible (no pausar si se redirige la salida)
    #[cfg(windows)]
    {
        use std::io::{self, Write};
        if atty::is(atty::Stream::Stdout) {
            print!("\nPresiona Enter para salir...");
            io::stdout().flush().ok();
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).ok();
        }
    }

    Ok(())
}