@echo off
REM Launcher que abre una nueva ventana de CMD para mostrar las estadísticas
REM Útil para ejecutar desde el explorador de archivos (doble clic)

REM Obtener la ruta del script
set SCRIPT_DIR=%~dp0

REM Abrir nueva ventana de CMD y ejecutar el programa
start "IDE Memory Stats" cmd /k "cd /d %SCRIPT_DIR% && IDE_Memory_Stats.exe --metrics-db ide_memory_metrics.db && pause"