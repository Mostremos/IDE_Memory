# Sistema de Métricas del MCP Server

## Descripción

El servidor MCP incluye un sistema completo de métricas que permite analizar el funcionamiento, rendimiento y uso del servidor.

## Métricas Registradas

### Por Solicitud

Cada solicitud MCP registra:

- **Método**: Tipo de solicitud (ej: "mcp_request", "mcp_notification")
- **Herramienta**: Nombre de la herramienta MCP llamada (ej: "mem_search", "mem_save")
- **Tiempo de respuesta**: En milisegundos
- **Tamaño de respuesta**: En bytes
- **Éxito/Error**: Si la solicitud fue exitosa
- **Mensaje de error**: Si hubo error, el mensaje
- **Timestamp**: Fecha y hora de la solicitud

### Estadísticas Agregadas

- **Total de solicitudes**: Número total de solicitudes procesadas
- **Total de errores**: Número de solicitudes fallidas
- **Tiempo promedio de respuesta**: Tiempo promedio en milisegundos
- **Tiempo de actividad**: Segundos desde que se inició el servidor
- **Estadísticas por herramienta**: Desglose detallado por cada herramienta MCP

## Uso

### Habilitar Métricas

Las métricas están habilitadas por defecto. Se guardan en una base de datos SQLite separada:

```bash
# Usa la base de datos principal con sufijo _metrics
IDE_Memory --database memory.db

# O especifica una base de datos de métricas diferente
IDE_Memory --database memory.db --metrics-db metrics.db
```

### Deshabilitar Métricas

```bash
IDE_Memory --database memory.db --no-metrics
```

### Ver Estadísticas

Para ver las estadísticas sin iniciar el servidor:

```bash
IDE_Memory --database memory.db --stats
```

Esto mostrará un JSON con todas las estadísticas:

```json
{
  "total_requests": 150,
  "total_errors": 2,
  "avg_response_time_ms": 45.3,
  "uptime_seconds": 3600,
  "tool_stats": [
    {
      "tool_name": "mem_search",
      "total_calls": 80,
      "success_count": 79,
      "error_count": 1,
      "avg_response_time_ms": 32.5,
      "total_response_size_bytes": 245000,
      "last_called": 1704067200
    },
    {
      "tool_name": "mem_save",
      "total_calls": 50,
      "success_count": 50,
      "error_count": 0,
      "avg_response_time_ms": 58.2,
      "total_response_size_bytes": 12000,
      "last_called": 1704067100
    }
  ]
}
```

## Consultas SQL Directas

Puedes consultar la base de datos de métricas directamente usando SQLite:

```bash
sqlite3 memory_metrics.db
```

### Ejemplos de Consultas

**Ver las últimas 10 solicitudes:**
```sql
SELECT * FROM request_metrics 
ORDER BY timestamp DESC 
LIMIT 10;
```

**Estadísticas por herramienta:**
```sql
SELECT 
    tool_name,
    COUNT(*) as total_calls,
    AVG(response_time_ms) as avg_time_ms,
    SUM(CASE WHEN success = 1 THEN 1 ELSE 0 END) as successes,
    SUM(CASE WHEN success = 0 THEN 1 ELSE 0 END) as errors
FROM request_metrics
WHERE tool_name IS NOT NULL
GROUP BY tool_name;
```

**Solicitudes más lentas:**
```sql
SELECT 
    tool_name,
    response_time_ms,
    timestamp
FROM request_metrics
ORDER BY response_time_ms DESC
LIMIT 10;
```

**Errores recientes:**
```sql
SELECT 
    tool_name,
    error_message,
    timestamp
FROM request_metrics
WHERE success = 0
ORDER BY timestamp DESC;
```

**Distribución de tiempos de respuesta:**
```sql
SELECT 
    CASE 
        WHEN response_time_ms < 50 THEN '< 50ms'
        WHEN response_time_ms < 100 THEN '50-100ms'
        WHEN response_time_ms < 200 THEN '100-200ms'
        ELSE '> 200ms'
    END as time_range,
    COUNT(*) as count
FROM request_metrics
GROUP BY time_range;
```

## Análisis de Rendimiento

### Métricas Clave a Monitorear

1. **Tiempo promedio de respuesta**: Debe ser < 100ms para la mayoría de herramientas
2. **Tasa de errores**: Debe ser < 1% de las solicitudes
3. **Tamaño de respuestas**: Monitorear si crece demasiado (puede indicar problemas)
4. **Uso por herramienta**: Identificar qué herramientas se usan más

### Alertas Recomendadas

- **Tiempo de respuesta > 500ms**: Investigar posibles problemas de rendimiento
- **Tasa de errores > 5%**: Revisar logs y errores específicos
- **Tamaño de respuesta > 1MB**: Considerar optimizar respuestas

## Exportar Datos

### Exportar a JSON

```bash
IDE_Memory --database memory.db --stats > stats.json
```

### Exportar a CSV (usando SQLite)

```bash
sqlite3 -header -csv memory_metrics.db \
  "SELECT * FROM request_metrics" > metrics.csv
```

## Integración con Monitoreo

Las métricas pueden integrarse con sistemas de monitoreo externos:

1. **Exportar periódicamente**: Usar `--stats` en un cron job
2. **Consultar SQLite**: Desde scripts de monitoreo
3. **Logs estructurados**: Los errores se registran en stderr

## Estructura de la Base de Datos

```sql
CREATE TABLE request_metrics (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    method TEXT NOT NULL,              -- Tipo de solicitud
    tool_name TEXT,                     -- Nombre de la herramienta
    response_time_ms INTEGER NOT NULL,  -- Tiempo en milisegundos
    response_size_bytes INTEGER NOT NULL, -- Tamaño en bytes
    success INTEGER NOT NULL,           -- 1 = éxito, 0 = error
    error_message TEXT,                 -- Mensaje de error si aplica
    timestamp INTEGER NOT NULL           -- Unix timestamp
);

-- Índices para consultas rápidas
CREATE INDEX idx_metrics_timestamp ON request_metrics(timestamp DESC);
CREATE INDEX idx_metrics_tool ON request_metrics(tool_name);
CREATE INDEX idx_metrics_method ON request_metrics(method);
```

## Ejemplos de Análisis

### Identificar Herramientas Más Usadas

```sql
SELECT 
    tool_name,
    COUNT(*) as usage_count,
    AVG(response_time_ms) as avg_time
FROM request_metrics
WHERE tool_name IS NOT NULL
GROUP BY tool_name
ORDER BY usage_count DESC;
```

### Analizar Tendencia de Tiempos

```sql
SELECT 
    DATE(timestamp, 'unixepoch') as date,
    AVG(response_time_ms) as avg_time,
    COUNT(*) as request_count
FROM request_metrics
GROUP BY date
ORDER BY date DESC;
```

### Detectar Problemas de Rendimiento

```sql
SELECT 
    tool_name,
    COUNT(*) as slow_requests
FROM request_metrics
WHERE response_time_ms > 200
GROUP BY tool_name
ORDER BY slow_requests DESC;
```

## Mejores Prácticas

1. **Revisar métricas regularmente**: Al menos una vez por semana
2. **Monitorear tendencias**: Comparar métricas entre períodos
3. **Investigar anomalías**: Si hay picos en tiempos o errores
4. **Optimizar basado en datos**: Usar métricas para identificar mejoras
5. **Limpiar datos antiguos**: Considerar archivar métricas antiguas (> 30 días)

## Limitaciones

- Las métricas se guardan localmente en SQLite
- No hay sincronización automática entre máquinas
- El tamaño de la base de datos crece con el tiempo (considerar limpieza periódica)

## Próximas Mejoras

- [ ] Dashboard web para visualizar métricas
- [ ] Alertas automáticas por email/notificación
- [ ] Exportación automática a sistemas de monitoreo (Prometheus, Grafana)
- [ ] Rotación automática de logs antiguos
- [ ] Métricas agregadas por proyecto (usando project_path)
