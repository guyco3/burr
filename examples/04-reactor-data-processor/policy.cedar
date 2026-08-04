permit(
    principal,
    action == Warden::Action::"env_read",
    resource
) when {
    context.key == "SECRET_KEY"
};

permit(
    principal,
    action == Warden::Action::"fs_read",
    resource
) when {
    context.path == "config.json" || context.path == "allowed.txt"
};
