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
        Invoke-Expression "git clone -b $branch --depth=1 $repo $destination"
    }
    else {
        Write-Output "Get-Source: $destination is exists and not empty, skip clone"
    }
}

function Get-DependenciesSource {
    # clone ffmpeg
    Write-Output "Get-DependenciesSource: FFmpeg"
    Get-Source "n5.1" "https://git.ffmpeg.org/ffmpeg.git" ".\dependencies_source\source\ffmpeg"
    
    # clone libx264
    Write-Output "Get-DependenciesSource: x264"
    Get-Source "0.164.r3094" "https://github.com/ShiftMediaProject/x264.git" ".\dependencies_source\source\x264"
    # Get-Source "stable" "https://code.videolan.org/videolan/x264.git" ".\dependencies_source\source\x264"

    # clone libopus
    Write-Output "Get-DependenciesSource: opus"
    Get-Source "v1.3.1-1" "https://github.com/ShiftMediaProject/opus.git" ".\dependencies_source\source\opus"
    # Get-Source "v1.3.1" "https://gitlab.xiph.org/xiph/opus.git" ".\dependencies_source\source\opus"

    # clone nv-codec-headers
    Write-Output "Get-DependenciesSource: nv-codec-headers"
    Get-Source "n11.1.5.1" "https://github.com/FFmpeg/nv-codec-headers.git" ".\dependencies_source\source\nv-codec-headers"

    # clone amf
    Write-Output "Get-DependenciesSource: AMF"
    Get-Source "v1.4.26" "https://github.com/GPUOpen-LibrariesAndSDKs/AMF.git" ".\dependencies_source\source\AMF"

    # clone mfx_dispatcher(Intel Media SDK dispatcher)
    Write-Output "Get-DependenciesSource: mfx_dispatcher"
    Get-Source "1.35.r89" "https://github.com/ShiftMediaProject/mfx_dispatch.git" ".\dependencies_source\source\mfx_dispatcher"
}

function Get-Component {
    # clone VSNASM
    Write-Output "Get-Component: VSNASM"
    Get-Source "master" "https://github.com/ShiftMediaProject/VSNASM.git" ".\dependencies_source\VSNASM"

    # clone VSWhere
    Write-Output "Get-Component: VSWhere"
    Get-VSWhere

    # clone FFVS-Project-Generator
    Write-Output "Get-Component: FFVS-Project-Generator"
    Get-FFVS_Project_Generator
}

function Get-VSWhere {
    try {
        [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
        Start-BitsTransfer "https://github.com/Microsoft/vswhere/releases/download/2.8.4/vswhere.exe" ".\dependencies_source\VSNASM\vswhere.exe"
    }
    catch {
        Write-Output "Install-Component: Download VSWhere failed"
        Write-Output $Error
        Eixt
    }
}

function Get-FFVS_Project_Generator {
    try {
        [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
        Start-BitsTransfer "https://github.com/ShiftMediaProject/FFVS-Project-Generator/releases/download/1.11.4/FFVS-Project-Generator_1.11.4_x64.zip" ".\dependencies_source\source\FFVS-Project-Generator_1.11.4_x64.zip"
    }
    catch {
        Write-Output "Install-Component: Download FFVS-Project-Generator failed: $Error"
        Eixt
    }

    Expand-Archive -Path ".\dependencies_source\source\FFVS-Project-Generator_1.11.4_x64.zip" -DestinationPath ".\dependencies_source\source" -Force
}

function Install-VSNASM {
    $env:CI = "Local"
    $res = Start-Process -FilePath ".\dependencies_source\VSNASM\install_script.bat" -Wait -PassThru -NoNewWindow
    $res = $res.ExitCode
    if ($res -ne 0) {
        Write-Output "Install-Component: install VSNASM failed"
        Exit
    }
}

function Install-Component {    
    Write-Output "Install-Component: VSNASM"
    Install-VSNASM
}

function Invoke-PrepareCompile {
    # copy nv-codec header
    Write-Output "Invoke-PrepareCompile: Copy NVCodec headers"
    Copy-Item -Path ".\dependencies_source\source\nv-codec-headers\include" -Recurse -Destination ".\dependencies_source\msvc\include" -Force

    # copy nv-codec header
    Write-Output "Invoke-PrepareCompile: Copy AMF headers"
    Copy-Item -Path ".\dependencies_source\source\AMF\amf\public\include" -Recurse -Destination ".\dependencies_source\msvc\include\AMF" -Force
}

function Invoke-CompileDependencies {
    try {
        Write-Output "Invoke-CompileDependencies: Generate opus VS project"
        Start-Process -FilePath ".\dependencies_source\source\opus\SMP\libopus_with_latest_sdk.bat" -Wait -PassThru -NoNewWindow

        Write-Output "Invoke-CompileDependencies: Generate x264 VS project"
        Start-Process -FilePath ".\dependencies_source\source\x264\SMP\x264_with_latest_sdk.bat" -Wait -PassThru -NoNewWindow

        Write-Output "Invoke-CompileDependencies: Generate mfx_dispatcher VS project"
        Start-Process -FilePath ".\dependencies_source\source\mfx_dispatcher\SMP\libmfx_with_latest_sdk.bat" -Wait -PassThru -NoNewWindow
    }
    catch {
        {1:<#Do this if a terminating exception happens#>}
    }
    
}

function Invoke-CompileOpus{
    try {
        Write-Output "Invoke-CompileOpus: Generate opus VS project"
        Start-Process -FilePath ".\dependencies_source\source\opus\SMP\libopus_with_latest_sdk.bat" -Wait -PassThru -NoNewWindow

        Write-Output "Invoke-CompileOpus: Upgrade project"
        Start-Process -FilePath "devenv.exe" -Wait -PassThru -NoNewWindow -ArgumentList ".\dependencies_source\source\opus\SMP\libopus.sln","-upgrade"
    }
    catch {
        Write-Output "Invoke-CompileOpus: Build failed"
        Write-Output $Error
        Exit
    }
}

function Invoke-GenerateVSProject {
    # generate vs project
    Write-Output "Invoke-GenerateVSProject: Generate VS project"
    Set-Location ".\dependencies_source\source"
    $res = Start-Process -FilePath ".\project_generate.exe" -Wait -PassThru -NoNewWindow -ArgumentList `
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

$elevated = Get-IsElevated
if ($elevated -eq $false) {
    Write-Output "Pleause run this script as Administrator"
    Exit
}

Get-Component

Install-Component

Get-DependenciesSource

Invoke-PrepareCompile

Invoke-CompileDependencies