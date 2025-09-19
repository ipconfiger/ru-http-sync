@echo off
setlocal

:: 获取当前脚本的目录
for %%I in ("%~dp0.") do set "BatchPath=%%~fI"

:: 添加到用户的 PATH 环境变量
setx path "%path%;%BatchPath%"

endlocal