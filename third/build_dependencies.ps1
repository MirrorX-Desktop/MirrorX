Import-Module BitsTransfer
Write-Output "MirrorX dependencies build script"

# trap {
#     Write-Output "Build failed, please try again!"
#     Exit
# }

function Get-IsElevated {
    $id = [System.Security.Principal.WindowsIdentity]::GetCurrent()
    $p = New-Object System.Security.Principal.WindowsPrincipal($id)
    if ($p.IsInRole([System.Security.Principal.WindowsBuiltInRole]::Administrator))
    { Write-Output $true }
    else
    { Write-Output $false }
}

function Get-IfDirectoryIsEmpty([string]$path) {
    if ((Get-ChildItem $path | Measure-Object).Count -eq 0) {
        Write-Output $true
    }
    else { Write-Output $false }
}

function Get-Source([string]$branch, [string]$repo, [string]$destination) {
    $folder_exist = Test-Path -Path $destination
    if (($folder_exist -eq $false) -or (Get-IfDirectoryIsEmpty)) {
        $res = Start-Process -Wait -FilePath "git.exe" -PassThru -NoNewWindow -ArgumentList "clone", "-b $branch", "--depth=1", "$repo", "$destination"
       
        if ($res.ExitCode -ne 0) {
            Write-Output "Get-Source: clone failed"
            Exit
        }
    }
    else {
        Write-Output "Get-Source: $destination is exists and not empty, skip clone"
    }
}

function Get-DependenciesSource {
    # clone ffmpeg
    Write-Output "Get-DependenciesSource: FFmpeg"
    Get-Source "release/5.1" "https://github.com/FFmpeg/FFmpeg.git" ".\dependencies\source\ffmpeg"
    
    # clone libx264
    Write-Output "Get-DependenciesSource: x264"
    Get-Source "0.164.r3094" "https://github.com/ShiftMediaProject/x264.git" ".\dependencies\source\x264"
    # Get-Source "stable" "https://code.videolan.org/videolan/x264.git" ".\dependencies\source\x264"

    # clone libopus
    Write-Output "Get-DependenciesSource: opus"
    Get-Source "v1.3.1-1" "https://github.com/ShiftMediaProject/opus.git" ".\dependencies\source\opus"
    # Get-Source "v1.3.1" "https://gitlab.xiph.org/xiph/opus.git" ".\dependencies\source\opus"

    # clone nv-codec-headers
    Write-Output "Get-DependenciesSource: nv-codec-headers"
    Get-Source "n11.1.5.1" "https://github.com/FFmpeg/nv-codec-headers.git" ".\dependencies\source\nv-codec-headers"

    # clone amf
    Write-Output "Get-DependenciesSource: AMF"
    Get-Source "v1.4.26" "https://github.com/GPUOpen-LibrariesAndSDKs/AMF.git" ".\dependencies\source\AMF"

    # clone mfx_dispatcher(Intel Media SDK dispatcher)
    Write-Output "Get-DependenciesSource: mfx_dispatcher"
    Get-Source "1.35.r89" "https://github.com/ShiftMediaProject/mfx_dispatch.git" ".\dependencies\source\mfx_dispatcher"

    # clone libyuv
    Write-Output "Get-DependenciesSource: libyuv"
    Get-Source "stable" "https://chromium.googlesource.com/libyuv/libyuv" ".\dependencies\source\libyuv"
}

function Get-Component {
    # clone VSNASM
    Write-Output "Get-Component: VSNASM"
    Get-Source "master" "https://github.com/ShiftMediaProject/VSNASM.git" ".\dependencies\VSNASM"

    # clone VSWhere
    Write-Output "Get-Component: VSWhere"
    Get-VSWhere

    # clone FFVS-Project-Generator
    Write-Output "Get-Component: FFVS-Project-Generator"
    Get-Source "master" "https://github.com/ShiftMediaProject/FFVS-Project-Generator.git" ".\dependencies\source\FFVS-Project-Generator"
}

function Get-VSWhere {
    if (Test-Path ".\dependencies\VSNASM\vswhere.exe") {
        Write-Output "Get-Component: VSWhere exists, skip download"
    }
    else {
        try {
            [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
            Start-BitsTransfer "https://github.com/Microsoft/vswhere/releases/download/2.8.4/vswhere.exe" ".\dependencies\VSNASM\vswhere.exe"
        }
        catch {
            Write-Output "Get-Component: Download VSWhere failed"
            Write-Output $Error
            Eixt
        }
    }
}

function Install-FFVS_Project_Generator {
    $proc = Start-Process -FilePath "MSBuild.exe" -PassThru -NoNewWindow -ArgumentList "-t:ReBuild", "-nodeReuse:false", "-p:Configuration=Release", "-p:Platform=x64", ".\dependencies\source\FFVS-Project-Generator\project_generate.sln"
    Wait-Process -InputObject $proc

    if ($proc.ExitCode -ne 0) {
        Write-Output "Install-Component: install FFVS_Project_Generator failed"
        Exit
    }

    Copy-Item -Path ".\dependencies\source\FFVS-Project-Generator\bin\project_generate.exe" -Destination ".\dependencies\source\project_generate.exe" -Force
}

function Install-VSNASM {
    $env:CI = "Local"
    $res = Start-Process -Wait -FilePath ".\dependencies\VSNASM\install_script.bat" -PassThru -NoNewWindow

    if ($res.ExitCode -ne 0) {
        Write-Output "Install-Component: install VSNASM failed"
        Exit
    }
}

function Install-Component {    
    Write-Output "Install-Component: VSNASM"
    Install-VSNASM

    Write-Output "Install-Component: FFVS_Project_Generator"
    Install-FFVS_Project_Generator
}

function Invoke-PrepareCompile {
    # copy nv-codec header
    Write-Output "Invoke-PrepareCompile: Copy NVCodec headers"
    Copy-Item -Path ".\dependencies\source\nv-codec-headers\include" -Recurse -Destination ".\dependencies\msvc\include" -Force

    # copy nv-codec header
    Write-Output "Invoke-PrepareCompile: Copy AMF headers"
    Copy-Item -Path ".\dependencies\source\AMF\amf\public\include" -Recurse -Destination ".\dependencies\msvc\include\AMF" -Force
}

function Invoke-CompileDependencies {
    try {
        Invoke-CompileOpus
        Invoke-CompileX264
        Invoke-CompileMFXDispatcher
    }
    catch {
        Write-Output "Invoke-CompileDependencies: Build failed"
        Exit
    }
    
}

function Invoke-CompileOpus {
    try {
        Write-Output "Invoke-CompileOpus: Upgrade project"
        Start-Process -Wait -FilePath "devenv.exe" -PassThru -NoNewWindow -ArgumentList ".\dependencies\source\opus\SMP\libopus.vcxproj", "-upgrade"

        Write-Output "Invoke-CompileOpus: Compile"
        $proc = Start-Process -FilePath "MSBuild.exe" -PassThru -NoNewWindow -ArgumentList "-t:ReBuild", "-nodeReuse:false", "-p:Configuration=Release", "-p:Platform=x64", "-p:PlatformToolset=v143", ".\dependencies\source\opus\SMP\libopus.vcxproj"
        Wait-Process -InputObject $proc

        if ($proc.ExitCode -ne 0) {
            Write-Output "Invoke-CompileOpus: Build failed"
            Exit
        }
    }
    catch {
        Write-Output "Invoke-CompileOpus: Build failed"
        Write-Output $Error
        Exit
    }
}

function Invoke-CompileX264 {
    try {
        Write-Output "Invoke-CompileX264: Upgrade project"
        Start-Process -Wait -FilePath "devenv.exe" -PassThru -NoNewWindow -ArgumentList ".\dependencies\source\x264\SMP\libx264.vcxproj", "-upgrade"
       
        Write-Output "Invoke-CompileX264: Compile"
        $proc = Start-Process -FilePath "MSBuild.exe" -PassThru -NoNewWindow -ArgumentList "-t:ReBuild", "-nodeReuse:false", "-p:Configuration=Release", "-p:Platform=x64", "-p:PlatformToolset=v143", ".\dependencies\source\x264\SMP\libx264.vcxproj"
        Wait-Process -InputObject $proc

        if ($proc.ExitCode -ne 0) {
            Write-Output "Invoke-CompileX264: Build failed"
            Exit
        }
    }
    catch {
        Write-Output "Invoke-CompileX264: Build failed"
        Write-Output $Error
        Exit
    }
}

function Invoke-CompileMFXDispatcher {
    try {
        Write-Output "Invoke-CompileMFXDispatcher: Upgrade project"
        Start-Process -Wait -FilePath "devenv.exe" -PassThru -NoNewWindow -ArgumentList ".\dependencies\source\mfx_dispatcher\SMP\libmfx.vcxproj", "-upgrade"

        Write-Output "Invoke-CompileMFXDispatcher: Compile"
        $proc = Start-Process -FilePath "MSBuild.exe" -PassThru -NoNewWindow -ArgumentList "-t:ReBuild", "-nodeReuse:false", "-p:Configuration=Release", "-p:Platform=x64", "-p:PlatformToolset=v143", ".\dependencies\source\mfx_dispatcher\SMP\libmfx.vcxproj"
        Wait-Process -InputObject $proc

        if ($proc.ExitCode -ne 0){
            Write-Output "Invoke-CompileMFXDispatcher: Build failed"
            Exit
        }
    }
    catch {
        Write-Output "Invoke-CompileMFXDispatcher: Build failed"
        Write-Output $Error
        Exit
    }
}

function Invoke-CompileLibYUV {
    try {
        Write-Output "Invoke-CompileLibYUV: Compile"
        Set-Location -Path ".\dependencies\source\libyuv"

        $proc = Start-Process -FilePath "CMake.exe" -PassThru -NoNewWindow -ArgumentList "-DCMAKE_INSTALL_PREFIX=..\..\libyuv", "-DCMAKE_BUILD_TYPE=Release"
        Wait-Process -InputObject $proc
        if ($proc.ExitCode -ne 0){
            Write-Output "Invoke-CompileLibYUV: Build failed"
            Exit
        }

        $proc = Start-Process -FilePath "CMake.exe" -PassThru -NoNewWindow -ArgumentList "--build", ".", "--target", "install", "--config", "Release"
        Wait-Process -InputObject $proc
        if ($proc.ExitCode -ne 0){
            Write-Output "Invoke-CompileLibYUV: Build failed"
            Exit
        }
    }
    catch {
        Write-Output "Invoke-CompileLibYUV: Build failed"
        Write-Output $Error
        Exit
    }
}

function Invoke-GenerateFFmpegVSProject {
    # generate vs project
    Write-Output "Invoke-GenerateFFmpegVSProject: Generate VS project"
    Set-Location ".\dependencies\source"
    $res = Start-Process -Wait -FilePath ".\project_generate.exe" -PassThru -NoNewWindow -ArgumentList `
        "--disable-all", `
        "--disable-autodetect", `
        "--enable-dxva2", `
        "--enable-d3d11va", `
        "--enable-cuvid", `
        "--enable-amf", `
        "--enable-libmfx", `
        "--enable-ffnvcodec", `
        "--enable-w32threads", `
        "--enable-gpl", `
        "--enable-version3", `
        "--enable-avutil", `
        "--enable-avdevice", `
        "--enable-avcodec", `
        "--enable-avformat", `
        "--enable-libx264", `
        "--enable-encoder=libx264", `
        "--enable-decoder=h264", `
        "--disable-doc", `
        "--disable-htmlpages", `
        "--disable-manpages", `
        "--disable-podpages", `
        "--disable-txtpages", `
        "--disable-network", `
        "--enable-nvenc", `
        "--enable-nvdec", `
        "--enable-encoder=h264_amf", `
        "--enable-encoder=h264_qsv", `
        "--enable-encoder=h264_nvenc", `
        "--enable-decoder=h264_cuvid", `
        "--enable-decoder=h264_qsv", `
        "--enable-hwaccel=h264_d3d11va", `
        "--enable-hwaccel=h264_d3d11va2", `
        "--enable-hwaccel=h264_dxva2" `
 
    Set-Location "..\.."
    if ($res.ExitCode -ne 0) {
        Write-Output "Invoke-PrepareCompile: Generate VS project failed"
        Exit
    }
}

function Invoke-CompileFFmpeg {
    try {
        Write-Output "Invoke-CompileMFXDispatcher: Compile"
        $proc = Start-Process -FilePath "MSBuild.exe" -PassThru -NoNewWindow -ArgumentList "-t:ReBuild", "-nodeReuse:false", "-p:Configuration=Release", "-p:Platform=x64", "-p:PlatformToolset=v143", ".\dependencies\source\ffmpeg\SMP\ffmpeg.sln"
        Wait-Process -InputObject $proc

        if ($proc.ExitCode -ne 0){
            Write-Output "Invoke-CompileFFmpeg: Build failed"
            Exit
        }
    }
    catch {
        Write-Output "Invoke-CompileFFmpeg: Build failed"
        Write-Output $Error
        Exit
    }
}

$elevated = Get-IsElevated
if ($elevated -eq $false) {
    Write-Output "Pleause run this script as Administrator"
    Exit
}

Get-DependenciesSource

# Get-Component

# Install-Component

# Invoke-PrepareCompile

# Invoke-CompileDependencies

# Invoke-GenerateFFmpegVSProject

# Invoke-CompileFFmpeg

Invoke-CompileLibYUV