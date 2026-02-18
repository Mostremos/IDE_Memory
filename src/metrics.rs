//! Sistema de métricas y logging para el servidor MCP
//! 
//! Registra estadísticas de uso, tiempos de respuesta, errores, etc.

use chrono::Utc;
use rusqlite::{Connection, Result as SqlResult, params};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestMetrics {
    pub id: i64,
    pub method: String,
    pub tool_name: Option<String>,
    pub response_time_ms: i64,
    pub response_size_bytes: i64,
    pub success: bool,
    pub error_message: Option<String>,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolUsageStats {
    pub tool_name: String,
    pub total_calls: i64,
    pub success_count: i64,
    pub error_count: i64,
    pub avg_response_time_ms: f64,
    pub total_response_size_bytes: i64,
    pub last_called: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerStats {
    pub total_requests: i64,
    pub total_errors: i64,
    pub avg_response_time_ms: f64,
    pub uptime_seconds: i64,
    pub tool_stats: Vec<ToolUsageStats>,
}

pub struct Metrics {
    conn: Mutex<Connection>,
    start_time: Instant,
}

impl Metrics {
    pub fn new(db_path: &std::path::Path) -> SqlResult<Self> {
        let conn = Connection::open(db_path)?;
        let metrics = Metrics {
            conn: Mutex::new(conn),
            start_time: Instant::now(),
        };
        metrics.init_schema()?;
        Ok(metrics)
    }

    fn init_schema(&self) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        
        // Tabla de métricas de solicitudes
        conn.execute(
            "CREATE TABLE IF NOT EXISTS request_metrics (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                method TEXT NOT NULL,
                tool_name TEXT,
                response_time_ms INTEGER NOT NULL,
                response_size_bytes INTEGER NOT NULL,
                success INTEGER NOT NULL,
                error_message TEXT,
                timestamp INTEGER NOT NULL
            )",
            [],
        )?;

        // Índices para consultas rápidas
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_metrics_timestamp ON request_metrics(timestamp DESC)",
            [],
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_metrics_tool ON request_metrics(tool_name)",
            [],
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_metrics_method ON request_metrics(method)",
            [],
        )?;

        Ok(())
    }

    pub fn record_request(
        &self,
        method: &str,
        tool_name: Option<&str>,
        response_time: Duration,
        response_size: usize,
        success: bool,
        error_message: Option<&str>,
    ) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        let timestamp = Utc::now().timestamp();

        conn.execute(
            "INSERT INTO request_metrics 
            (method, tool_name, response_time_ms, response_size_bytes, success, error_message, timestamp)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                method,
                tool_name,
                response_time.as_millis() as i64,
                response_size as i64,
                if success { 1 } else { 0 },
                error_message,
                timestamp
            ],
        )?;

        Ok(())
    }

    pub fn get_tool_stats(&self, tool_name: &str) -> SqlResult<ToolUsageStats> {
        let conn = self.conn.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT 
                COUNT(*) as total_calls,
                SUM(CASE WHEN success = 1 THEN 1 ELSE 0 END) as success_count,
                SUM(CASE WHEN success = 0 THEN 1 ELSE 0 END) as error_count,
                AVG(response_time_ms) as avg_response_time_ms,
                SUM(response_size_bytes) as total_response_size_bytes,
                MAX(timestamp) as last_called
             FROM request_metrics
             WHERE tool_name = ?1"
        )?;

        let row = stmt.query_row(params![tool_name], |row| {
            Ok(ToolUsageStats {
                tool_name: tool_name.to_string(),
                total_calls: row.get(0)?,
                success_count: row.get(1)?,
                error_count: row.get(2)?,
                avg_response_time_ms: row.get(3).unwrap_or(0.0),
                total_response_size_bytes: row.get(4)?,
                last_called: row.get(5)?,
            })
        })?;

        Ok(row)
    }

    pub fn get_server_stats(&self) -> SqlResult<ServerStats> {
        let conn = self.conn.lock().unwrap();

        // Estadísticas generales
        let mut stmt = conn.prepare(
            "SELECT 
                COUNT(*) as total_requests,
                SUM(CASE WHEN success = 0 THEN 1 ELSE 0 END) as total_errors,
                AVG(response_time_ms) as avg_response_time_ms
             FROM request_metrics"
        )?;

        let (total_requests, total_errors, avg_response_time_ms) = stmt.query_row([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, f64>(2).unwrap_or(0.0),
            ))
        })?;

        // Estadísticas por herramienta
        let mut tool_stats = Vec::new();
        let mut stmt = conn.prepare(
            "SELECT DISTINCT tool_name FROM request_metrics WHERE tool_name IS NOT NULL"
        )?;

        let tool_names: Vec<String> = stmt
            .query_map([], |row| Ok(row.get::<_, String>(0)?))?
            .collect::<Result<_, _>>()?;

        for tool_name in tool_names {
            if let Ok(stats) = self.get_tool_stats(&tool_name) {
                tool_stats.push(stats);
            }
        }

        Ok(ServerStats {
            total_requests,
            total_errors,
            avg_response_time_ms,
            uptime_seconds: self.start_time.elapsed().as_secs() as i64,
            tool_stats,
        })
    }

    pub fn get_recent_requests(&self, limit: i32) -> SqlResult<Vec<RequestMetrics>> {
        let conn = self.conn.lock().unwrap();
        let mut results = Vec::new();

        let mut stmt = conn.prepare(
            "SELECT id, method, tool_name, response_time_ms, response_size_bytes, 
                    success, error_message, timestamp
             FROM request_metrics
             ORDER BY timestamp DESC
             LIMIT ?1"
        )?;

        let rows = stmt.query_map(params![limit], |row| {
            let success_int: i64 = row.get(5)?;
            Ok(RequestMetrics {
                id: row.get(0)?,
                method: row.get(1)?,
                tool_name: row.get(2)?,
                response_time_ms: row.get(3)?,
                response_size_bytes: row.get(4)?,
                success: success_int == 1,
                error_message: row.get(6)?,
                timestamp: row.get(7)?,
            })
        })?;

        for row in rows {
            results.push(row?);
        }

        Ok(results)
    }

    pub fn export_stats_json(&self) -> Result<String, Box<dyn std::error::Error>> {
        let stats = self.get_server_stats()?;
        Ok(serde_json::to_string_pretty(&stats)?)
    }
}
