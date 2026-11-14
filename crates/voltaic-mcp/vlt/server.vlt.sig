# voltaic 2 signature sidecar — generated, do not edit
new : Server
voltaic_check : Parameters<CheckRequest> -> Server -> Result<CallToolResult, ErrorData> ! fail ErrorData + async @ params shared, self shared
voltaic_build : Parameters<BuildRequest> -> Server -> Result<CallToolResult, ErrorData> ! fail ErrorData + async @ params shared, self shared
voltaic_fmt : Parameters<FormatRequest> -> Server -> Result<CallToolResult, ErrorData> ! fail ErrorData + async @ params shared, self shared
voltaic_source_map : Parameters<SourceMapRequest> -> Server -> Result<CallToolResult, ErrorData> ! fail ErrorData + async @ params shared, self shared
get_info : Server -> ServerConfig ! mut config @ self shared
server_content : Result<T, String> -> Result<CallToolResult, ErrorData> ! fail ErrorData @ outcome shared
Server.new method
Server.voltaic_check method
Server.voltaic_build method
Server.voltaic_fmt method
Server.voltaic_source_map method
Server.get_info method
