@echo off
REM Script para ejecutar IDE_Memory_Stats en una nueva ventana de CMD
REM Mantiene la ventana abierta después de mostrar las estadísticas

REM Cambiar al directorio del script
cd /d "%~dp0"

REM Ejecutar el programa con los argumentos pasados
IDE_Memory_Stats.exe %*

REM Pausar para mantener la ventana abierta
pause