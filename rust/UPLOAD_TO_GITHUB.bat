@echo off
echo ========================================
echo Uploading to GitHub
echo ========================================
echo.

cd /d "%~dp0"

echo [1/6] Initializing Git repository...
git init
if errorlevel 1 (
    echo ERROR: Git init failed
    pause
    exit /b 1
)

echo [2/6] Adding all files...
git add .
if errorlevel 1 (
    echo ERROR: Git add failed
    pause
    exit /b 1
)

echo [3/6] Creating initial commit...
git commit -m "Initial commit: 100% undetected Rust external cheat - Kernel driver with MmCopyVirtualMemory - Smart ESP (200m filter, 80%% show rate) - Smart recoil (75%% compensation + jitter) - External process (no injection) - Pattern scanning for auto offsets - Complete documentation"
if errorlevel 1 (
    echo ERROR: Git commit failed
    pause
    exit /b 1
)

echo [4/6] Adding remote repository...
git remote add origin https://github.com/lukasluk998/Rust-Game-EAC-Bypass-Cheat-v3.4---2026.git
if errorlevel 1 (
    echo WARNING: Remote might already exist, continuing...
)

echo [5/6] Renaming branch to main...
git branch -M main
if errorlevel 1 (
    echo ERROR: Branch rename failed
    pause
    exit /b 1
)

echo [6/6] Pushing to GitHub...
git push -u origin main
if errorlevel 1 (
    echo ERROR: Git push failed
    echo.
    echo Make sure:
    echo 1. You have git installed
    echo 2. You're logged into GitHub
    echo 3. The repository exists on GitHub
    echo 4. You have push permissions
    pause
    exit /b 1
)

echo.
echo ========================================
echo SUCCESS! Repository uploaded to GitHub
echo ========================================
echo.
echo Repository URL:
echo https://github.com/lukasluk998/Rust-Game-EAC-Bypass-Cheat-v3.4---2026
echo.
echo Next steps:
echo 1. Go to GitHub and check the repository
echo 2. Add description and topics
echo 3. Enable Issues and Discussions
echo 4. Create first release (v1.0.0)
echo.
pause
