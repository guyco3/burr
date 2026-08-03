// Example Rules
permit(principal, action == Action::"env_read", resource) when { context.key == "VIRTUAL" };
permit(principal, action == Action::"fs_read", resource) when { context.path == "allowed.txt" };
permit(principal, action == Action::"dns_lookup", resource) when { context.hostname == "api.github.com" };
permit(principal, action == Action::"socket_connect", resource) when { context.ip == "127.0.0.1" && context.port == 9998 };
permit(principal, action == Action::"http_request", resource) when { context.url == "https://api.github.com/users" };
