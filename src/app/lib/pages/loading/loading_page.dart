import 'package:app/business/mirrorx_core/mirrorx_core_bloc.dart';
import 'package:app/components/layout/layout.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

class LoadingPage extends StatelessWidget {
  const LoadingPage({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<MirrorXCoreBloc, MirrorXCoreState>(
        builder: ((context, state) {
      switch (state.status) {
        case MirrorXCoreStateStatus.initial:
          return Container();
        case MirrorXCoreStateStatus.loading:
          return const Center(child: CircularProgressIndicator());
        case MirrorXCoreStateStatus.success:
          return const Layout();
        case MirrorXCoreStateStatus.failure:
          return Center(child: Text(state.lastError.toString()));
      }
    }));
  }
}
