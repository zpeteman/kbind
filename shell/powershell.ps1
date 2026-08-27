Set-PSReadLineKeyHandler -Chord 'Ctrl+g' -ScriptBlock {
    $line = [Microsoft.PowerShell.PSConsoleReadLine]::GetBufferState().Command
    $cmd = kb gen $line
    if ($LASTEXITCODE -eq 0) {
        [Microsoft.PowerShell.PSConsoleReadLine]::Replace(0, $line.Length, $cmd)
    }
}
