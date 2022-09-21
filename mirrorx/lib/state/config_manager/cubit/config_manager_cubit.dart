import 'dart:developer';

import 'package:bloc/bloc.dart';
import 'package:equatable/equatable.dart';
import 'package:flutter/material.dart';
import 'package:meta/meta.dart';
import 'package:mirrorx/env/sdk/mirrorx_core.dart';
import 'package:mirrorx/env/sdk/mirrorx_core.dart';
import 'package:mirrorx/env/sdk/mirrorx_core_sdk.dart';
import 'package:mirrorx/env/utility/error_notifier.dart';
import 'package:path_provider/path_provider.dart';

part 'config_manager_state.dart';

class ConfigManagerCubit extends Cubit<ConfigManagerState> {
  ConfigManagerCubit(String configPath, String? primaryDomain,
      DomainConfig? primaryDomainConfig)
      : super(ConfigManagerState(
            configPath: configPath,
            domain: primaryDomain,
            domainConfig: primaryDomainConfig));

  Future switchDomain(String domain) async {
    if (state.configPath != null) {
      final domainConfig = await MirrorXCoreSDK.instance
          .readDomainConfig(path: state.configPath!, domain: domain);
      emit(state.copyWith(domain: domain, domainConfig: domainConfig));
    }
  }
}
