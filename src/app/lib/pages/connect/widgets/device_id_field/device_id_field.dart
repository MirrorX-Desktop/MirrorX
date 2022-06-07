import 'package:app/business/mirrorx_core/mirrorx_core_bloc.dart';
import 'package:app/env/langs/tr.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

class DeviceIdField extends StatelessWidget {
  const DeviceIdField({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<MirrorXCoreBloc, MirrorXCoreState>(
      builder: (context, state) => Container(
        height: 110,
        width: 360,
        decoration: const BoxDecoration(
          border: Border(left: BorderSide(color: Colors.yellow, width: 4)),
        ),
        child: Padding(
          padding: const EdgeInsets.only(left: 12.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Text(
                    Tr.of(context).connectPageDeviceIDTitle,
                    style: const TextStyle(fontSize: 27),
                  ),
                  IconButton(
                    onPressed: () {
                      Clipboard.setData(ClipboardData(text: state.deviceId))
                          .then(
                        (_) => ScaffoldMessenger.of(context).showSnackBar(SnackBar(
                            content: Text(Tr.of(context)
                                .connectPageDeviceIDButtonCopySnackbarContent))),
                      );
                    },
                    icon: const Icon(Icons.copy),
                    splashRadius: 20,
                    hoverColor: Colors.yellow,
                    tooltip:
                        Tr.of(context).connectPageDeviceIDButtonCopyTooltip,
                  ),
                ],
              ),
              Expanded(child: _buildDeviceId(context, state))
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildDeviceId(BuildContext context, MirrorXCoreState state) {
    if (state.deviceId != null) {
      return Text(
        state.deviceId!,
        style: const TextStyle(fontSize: 45),
      );
    }

    context.read<MirrorXCoreBloc>().add(MirrorXCoreConfigReadDeviceId());
    return const Center(child: CircularProgressIndicator());
  }
}
