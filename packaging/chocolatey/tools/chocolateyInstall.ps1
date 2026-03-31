$ErrorActionPreference = 'Stop'

$packageName = 'mdterm'
$url64 = 'https://github.com/fasterinnerlooper/mdterm/releases/download/v1.1.5/mdterm-win-x64.zip'
$checksum64 = '3b6629e46dff3a37c80c2096c2014248984e6a0d2e4acaed8f06f52180b04011'

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
