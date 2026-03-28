$ErrorActionPreference = 'Stop'

$packageName = 'mdterm'
$url64 = 'https://github.com/fasterinnerlooper/mdterm/releases/download/vVERSION_PLACEHOLDER/mdterm-win-x64.zip'
$checksum64 = 'SHA256_PLACEHOLDER'

$packageArgs = @{
    packageName    = $packageName
    unzipLocation  = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
    url64bit       = $url64
    checksum64     = $checksum64
    checksumType64 = 'sha256'
}

Install-ChocolateyZipPackage @packageArgs

$installPath = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
Install-ChocolateyPath -PathToInstall $installPath -PathType 'User'
