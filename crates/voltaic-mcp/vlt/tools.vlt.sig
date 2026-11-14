# voltaic 2 signature sidecar — generated, do not edit
build : &BuildRequest -> Result<BuildResponse, String> ! fail String @ request shared
check : &CheckRequest -> Result<CheckResponse, String> ! fail String @ request shared
format : &FormatRequest -> Result<FormatResponse, String> ! fail String @ request shared
source_map : &SourceMapRequest -> Result<SourceMapResponse, String> ! fail String @ request shared
BuildRequest.path field
BuildResponse.ok field
BuildResponse.output field
BuildResponse.map field
BuildResponse.diagnostics field
CheckRequest.path field
CheckResponse.ok field
CheckResponse.diagnostics field
FormatRequest.path field
FormatResponse.ok field
FormatResponse.formatted field
FormatResponse.changed field
FormatResponse.diagnostics field
SourceMapRequest.path field
SourceMapRequest.line field
SourceMapRequest.column field
SourceMapResponse.source field
SourceMapResponse.line field
SourceMapResponse.column field
SourceMapResponse.reason field
