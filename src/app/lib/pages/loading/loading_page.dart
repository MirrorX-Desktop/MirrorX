import 'package:app/components/layout/layout.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'bloc/loading_bloc.dart';

class LoadingPage extends StatelessWidget {
  const LoadingPage({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return BlocProvider(
        create: (context) => LoadingBloc()..add(LoadingEventLoad()),
        child: const _LoadingView());
  }
}

class _LoadingView extends StatelessWidget {
  const _LoadingView({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<LoadingBloc, LoadingState>(builder: ((context, state) {
      switch (state.status) {
        case LoadingStateStatus.initial:
          return Container();
        case LoadingStateStatus.loading:
          return const Center(child: CircularProgressIndicator());
        case LoadingStateStatus.success:
          return const Layout();
        case LoadingStateStatus.failure:
          return Center(child: Text(state.loadingError.toString()));
      }
    }));
  }
}
