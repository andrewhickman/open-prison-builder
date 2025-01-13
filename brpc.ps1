$remoteUri = 'http://localhost:15702'

function Invoke-BevyApi([string]$method, $params) {
    $body = @{ jsonrpc = "2.0" ; method = $method ; params = $params } | ConvertTo-Json -Compress -Depth 100

    $res = Invoke-WebRequest $remoteUri -Method Post -ContentType 'application/json' -Body $body | ConvertFrom-Json -Depth 100
    if ($res.error) {
        throw ($res.error | ConvertTo-Json -Depth 100)
    }

    $res
}

function Get-BevyEntityComponents($id) {
    Invoke-BevyApi 'bevy/list' @{ entity = $id }
}

function Get-BevyEntity($id) {
    $components = Get-BevyEntityComponents $id | Select-Object -ExpandProperty result
    Invoke-BevyApi 'bevy/get' @{ entity = $id; components = $components }
}
