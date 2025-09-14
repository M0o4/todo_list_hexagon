window.onload = function() {
  //<editor-fold desc="Changeable Configuration Block">

  const HideTopbarPlugin = () => {
    return {
      components: {
        Topbar: () => null, // Hide the top bar
      },
    };
  };

  const HideServersPlugin = () => {
    return {
      components: {
        Servers: () => null, // Hide the servers dropdown
      },
    };
  };

  // the following lines will be replaced by docker/configurator, when it runs in a docker-container
  window.ui = SwaggerUIBundle({
    url: "openapi.yaml",
    dom_id: '#swagger-ui',
    deepLinking: true,
    filter: true,
    presets: [
      SwaggerUIBundle.presets.apis,
      SwaggerUIStandalonePreset
    ],
    plugins: [
      SwaggerUIBundle.plugins.DownloadUrl,
      HideTopbarPlugin,
      HideServersPlugin
    ],
    layout: "StandaloneLayout",
    onComplete: () => {
      // Inject custom HTML into the title area
      const titleElement = document.querySelector(".information-container .title");
      if (titleElement) {
        titleElement.innerHTML = `
              <div class="custom-title">
                <span>TodoList</span>
                <img src="https://www.gifcen.com/wp-content/uploads/2022/06/anime-gif-4.gif" alt="Logo GIF" width="32px" height="32px">
                <span><small><pre class="version"> 0.0.1 </pre></small><small class="version-stamp"><pre class="version">OAS 3.0</pre></small></span>
              </div>
            `;
      }
    },
  });

  //</editor-fold>
};