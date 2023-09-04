//! Semantic conventions

/// Semantic conventions for services
pub struct SemConv {
    _private: (),
}

impl SemConv {
    /// Service name (string)
    pub const SERVICE_NAME: &str = "service.name";

    /// Service namespace (string)
    pub const SERVICE_NAMESPACE: &str = "service.namespace";

    /// Service instance id (string)
    pub const SERVICE_INSTANCE_ID: &str = "service.instance.id";

    /// Service version (string)
    pub const SERVICE_VERSION: &str = "service.version";

    /// Telemetry SDK name (string)
    pub const TELEMETRY_SDK_NAME: &str = "telemetry.sdk.name";

    /// Telemetry SDK language (string)
    pub const TELEMETRY_SDK_LANGUAGE: &str = "telemetry.sdk.language";

    /// Telemetry SDK version (string)
    pub const TELEMETRY_SDK_VERSION: &str = "telemetry.sdk.version";

    /// Telemetry auto version (string)
    pub const TELEMETRY_AUTO_VERSION: &str = "telemetry.auto.version";

    /// Container name (string)
    pub const CONTAINER_NAME: &str = "container.name";

    /// Container id (string)
    pub const CONTAINER_ID: &str = "container.id";

    /// Container runtime (string)
    pub const CONTAINER_RUNTIME: &str = "container.runtime";

    /// Container image name (string)
    pub const CONTAINER_IMAGE_NAME: &str = "container.image.name";

    /// Container image tag (string)
    pub const CONTAINER_IMAGE_TAG: &str = "container.image.tag";

    /// Faas name (string)
    pub const FAAS_NAME: &str = "faas.name";

    /// Faas version (string)
    pub const FAAS_VERSION: &str = "faas.version";

    /// Faas instance (string)
    pub const FAAS_INSTANCE: &str = "faas.instance";

    /// Faas instance (int - bytes)
    pub const FAAS_MAX_MEMORY: &str = "faas.max_memory";

    /// Process PID (int)
    pub const PROCESS_PID: &str = "process.pid";

    /// Process parent PID (int)
    pub const PROCESS_PARENT_ID: &str = "process.parent_id";

    /// Process exec name (string)
    pub const PROCESS_EXECUTABLE_NAME: &str = "process.executable.name";

    /// Process exec path (string)
    pub const PROCESS_EXECUTABLE_PATH: &str = "process.executable.path";

    /// Process command (string)
    pub const PROCESS_COMMAND: &str = "process.command";

    /// Process command line (string)
    pub const PROCESS_COMMAND_LINE: &str = "process.command_line";

    /// Process command line (string[])
    pub const PROCESS_COMMAND_ARGS: &str = "process.command_args";

    /// Process owner (string)
    pub const PROCESS_OWNER: &str = "process.owner";

    /// Web engine name (string)
    pub const WEBENGINE_NAME: &str = "webengine.name";

    /// Web engine version (string)
    pub const WEBENGINE_VERSION: &str = "webengine.version";

    /// Web engine description (string)
    pub const WEBENGINE_DESCRIPTION: &str = "webengine.description";

    /// Host ID (string)
    pub const HOST_ID: &str = "host.id";

    /// Host name (string)
    pub const HOST_NAME: &str = "host.name";

    /// Host type (string)
    pub const HOST_TYPE: &str = "host.type";

    /// Host architecture (string)
    pub const HOST_ARCH: &str = "host.arch";

    /// Host image name (string)
    pub const HOST_IMAGE_NAME: &str = "host.image.name";

    /// Host image id (string)
    pub const HOST_IMAGE_ID: &str = "host.image.id";

    /// Host image version (string)
    pub const HOST_IMAGE_VERSION: &str = "host.image.version";

    /// OS type (string)
    pub const OS_TYPE: &str = "os.type";

    /// OS description (string)
    pub const OS_DESCRIPTION: &str = "os.description";

    /// OS name (string)
    pub const OS_NAME: &str = "os.name";

    /// OS version (string)
    pub const OS_VERSION: &str = "os.version";

    /// Device id (string)
    pub const DEVICE_ID: &str = "device.id";

    /// Device model identifier (string)
    pub const DEVICE_MODEL_IDENTIFIER: &str = "device.model.identifier";

    /// Device model name (string)
    pub const DEVICE_MODEL_NAME: &str = "device.model.name";

    /// Device manufacturer (string)
    pub const DEVICE_MANUFACTURER: &str = "device.manufacturer";

    /// Cloud provider (string)
    pub const CLOUD_ID: &str = "cloud.provider";

    /// Cloud account ID (string)
    pub const CLOUD_ACCOUNT_ID: &str = "cloud.account.id";

    /// Cloud region (string)
    pub const CLOUD_REGION: &str = "cloud.region";

    /// Cloud resource ID (string)
    pub const CLOUD_RESOURCE_ID: &str = "cloud.resource_id";

    /// Cloud zone (string)
    pub const CLOUD_AVAILABILITY_ZONE: &str = "cloud.availability_zone";

    /// Cloud platform (string)
    pub const CLOUD_PLATFORM: &str = "cloud.platform";

    /// Browser brands (string[])
    pub const BROWSER_BRANDS: &str = "browser.brands";

    /// Browser platform (string)
    pub const BROWSER_PLATFORM: &str = "browser.platform";

    /// Browser mobile (bool)
    pub const BROWSER_MOBILE: &str = "browser.mobile";

    /// Browser language (string)
    pub const BROWSER_LANGUAGE: &str = "browser.language";

    /// User agent (string)
    pub const USER_AGENT_ORIGINAL: &str = "user_agent.original";

    /// Transport protocol (string)
    pub const NET_TRANSPORT: &str = "net.transport";

    /// Transport protocol name (string)
    pub const NET_PROTOCOL_NAME: &str = "net.protocol.name";

    /// Transport protocol version (string)
    pub const NET_PROTOCOL_VERSION: &str = "net.protocol.version";

    /// Remote socket peer name (string)
    pub const NET_SOCK_PEER_NAME: &str = "net.sock.peer.name";

    /// Remote socket peer address (string)
    pub const NET_SOCK_PEER_ADDR: &str = "net.sock.peer.addr";

    /// Remote socket peer port (int)
    pub const NET_SOCK_PEER_PORT: &str = "net.sock.peer.port";

    /// Remote socket family (string)
    pub const NET_SOCK_FAMILY: &str = "net.sock.family";

    /// Remote peer name (string)
    pub const NET_PEER_NAME: &str = "net.peer.name";

    /// Remote peer port (int)
    pub const NET_PEER_PORT: &str = "net.peer.port";

    /// Local host name (string)
    pub const NET_HOST_NAME: &str = "net.host.name";

    /// Local host name (int)
    pub const NET_HOST_PORT: &str = "net.host.port";

    /// Local socket address (string)
    pub const NET_SOCK_HOST_ADDR: &str = "net.sock.host.addr";

    /// Local socket port (int)
    pub const NET_SOCK_HOST_PORT: &str = "net.sock.host.port";

    /// Host internet connection type (string)
    pub const NET_HOST_CONNECTION_TYPE: &str = "net.host.connection.type";

    /// Host internet connection subtype (string)
    pub const NET_HOST_CONNECTION_SUBTYPE: &str = "net.host.connection.subtype";

    /// Host internet carrier (string)
    pub const NET_HOST_CARRIER_NAME: &str = "net.host.carrier.name";

    /// Host internet carrier country code (string)
    pub const NET_HOST_CARRIER_MCC: &str = "net.host.carrier.mcc";

    /// Host internet carrier network code (string)
    pub const NET_HOST_CARRIER_MNC: &str = "net.host.carrier.mnc";

    /// Host internet carrier country code (string)
    pub const NET_HOST_CARRIER_ICC: &str = "net.host.carrier.icc";

    /// Remote service name (string)
    pub const PEER_SERVICE: &str = "peer.service";

    /// End user ID (string)
    pub const ENDUSER_ID: &str = "enduser.id";

    /// End user role (string)
    pub const ENDUSER_ROLE: &str = "enduser.role";

    /// End user scope (string)
    pub const ENDUSER_SCOPE: &str = "enduser.scope";

    /// Thread ID (int)
    pub const THREAD_ID: &str = "thread.id";

    /// Thread name (string)
    pub const THREAD_NAME: &str = "thread.name";

    /// Source code function (string)
    pub const CODE_FUNCTION: &str = "code.function";

    /// Source code namespace (string)
    pub const CODE_NAMESPACE: &str = "code.namespace";

    /// Source code filepath (string)
    pub const CODE_FILEPATH: &str = "code.filepath";

    /// Source code line number (int)
    pub const CODE_LINENO: &str = "code.lineno";

    /// Source code column number (int)
    pub const CODE_COLUMN: &str = "code.column";

    /// HTTP status code (int)
    pub const HTTP_STATUS_CODE: &str = "http.status_code";

    /// HTTP request content length (int)
    pub const HTTP_REQUEST_CONTENT_LENGTH: &str = "http.request_content_length";

    /// HTTP response content length (int)
    pub const HTTP_RESPONSE_CONTENT_LENGTH: &str = "http.response_content_length";

    /// HTTP request method (string)
    pub const HTTP_METHOD: &str = "http.method";

    /// HTTP request header (string[]) - this is the prefix
    pub const HTTP_REQUEST_HEADER_: &str = "http.request.header.";

    /// HTTP url (string)
    pub const HTTP_URL: &str = "http.url";

    /// HTTP resend count (int)
    pub const HTTP_RESEND_COUNT: &str = "http.resend_count";

    /// HTTP route (string)
    pub const HTTP_ROUTE: &str = "http.route";

    /// HTTP target (string)
    pub const HTTP_TARGET: &str = "http.target";

    /// HTTP client IP (string)
    pub const HTTP_CLIENT_IP: &str = "http.client_ip";

    /// HTTP scheme (string)
    pub const HTTP_SCHEME: &str = "http.scheme";

    /// DB system (string)
    pub const DB_SYSTEM: &str = "db.system";

    /// DB connections string (string)
    pub const DB_CONNECTION_STRING: &str = "db.connection_string";

    /// DB user (string)
    pub const DB_USER: &str = "db.user";

    /// DB name (string)
    pub const DB_NAME: &str = "db.name";

    /// DB statement (string)
    pub const DB_STATEMENT: &str = "db.statement";

    /// DB operation (string)
    pub const DB_OPERATION: &str = "db.operation";

    /// DB redis db index (string)
    pub const DB_REDIS_DATABASE_INDEX: &str = "db.redis.database_index";

    /// DB mongoDB collection (string)
    pub const DB_MONGODB_COLLECTION: &str = "db.mongodb.collection";

    /// DB SQL table (string)
    pub const DB_SQL_TABLE: &str = "db.sql.table";
}
