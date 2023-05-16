//! Semantic conventions

/// Semantic conventions for Resource
pub struct ResourceSemConvention {
    /// Private field to prevent instantiation
    _private: (),
}

impl ResourceSemConvention {
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
}
