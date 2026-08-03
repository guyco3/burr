permit(
    principal == Warden::Module::"telemetry-demo",
    action == Warden::Action::"env_read",
    resource == Warden::Resource::"environment"
) when {
    context.key == "VIRTUAL"
};

permit(
    principal == Warden::Module::"telemetry-demo",
    action == Warden::Action::"fs_read",
    resource == Warden::Resource::"filesystem"
) when {
    context.path == "allowed.txt"
};

permit(
    principal == Warden::Module::"telemetry-demo",
    action == Warden::Action::"dns_lookup",
    resource == Warden::Resource::"network"
) when {
    context.hostname == "api.github.com"
};

permit(
    principal == Warden::Module::"telemetry-demo",
    action == Warden::Action::"http_request",
    resource == Warden::Resource::"network"
) when {
    context.url like "https://api.github.com/*"
};

permit(
    principal == Warden::Module::"telemetry-demo",
    action == Warden::Action::"socket_connect",
    resource == Warden::Resource::"network"
) when {
    context.ip == "127.0.0.1" && context.port == 9998
};
