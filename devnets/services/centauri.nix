{ name, execCommands, configPathSource, configPathContainer, dependsOn
, restartPolicy, pkgs, packages, devnetTools }: {
  image = {
    contents = [ packages.hyperspace-composable-rococo-picasso-rococo ]
      ++ devnetTools.withBaseContainerTools;
    enableRecommendedContents = true;
  };
  service = {
    restart = restartPolicy;
    environment = { RUST_LOG = "trace,socketto=debug,tracing::span=debug,mio::poll=debug"; };
    entrypoint = "${pkgs.lib.meta.getExe
      packages.hyperspace-composable-rococo-picasso-rococo}";
    command = execCommands;
    volumes = [{
      source = configPathSource;
      target = configPathContainer;
      type = "bind";
    }];
  } // pkgs.lib.optionalAttrs (dependsOn != null) { depends_on = dependsOn; };
}
