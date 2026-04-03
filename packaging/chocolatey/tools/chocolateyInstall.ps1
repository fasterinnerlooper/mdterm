$ErrorActionPreference = 'Stop'

$packageName = 'mdterm'
$url64 = 'https://github.com/fasterinnerlooper/mdterm/releases/download/v1.2.11/mdterm-win-x64.zip'
$checksum64 = '2ccd4ee084a45e0c38aa4ddd6745c3c524fc236fa5722ef604f7225d27c07cf7'

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
