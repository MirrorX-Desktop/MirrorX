import 'package:bloc/bloc.dart';
import 'package:meta/meta.dart';

part 'connect_event.dart';
part 'connect_state.dart';

class ConnectBloc extends Bloc<ConnectEvent, ConnectState> {
  ConnectBloc() : super(ConnectInitial()) {
    on<ConnectEvent>((event, emit) {
      // TODO: implement event handler
    });
  }
}
