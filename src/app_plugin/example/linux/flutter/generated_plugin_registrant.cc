//
//  Generated file. Do not edit.
//

// clang-format off

#include "generated_plugin_registrant.h"

#include <app_plugin/app_plugin.h>

void fl_register_plugins(FlPluginRegistry* registry) {
  g_autoptr(FlPluginRegistrar) app_plugin_registrar =
      fl_plugin_registry_get_registrar_for_plugin(registry, "AppPlugin");
  app_plugin_register_with_registrar(app_plugin_registrar);
}
